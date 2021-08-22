use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::borrow::BorrowMut;
use std::io::{self, Cursor, Read, Write};
use std::net::{Shutdown, TcpStream, ToSocketAddrs};
use std::time::Duration;
use modbus_protocol::exception_code::{Error, ExceptionCode, Reason, Result};
use modbus_protocol::coils::Coil;
use modbus_protocol::requests::Requests;
use modbus_protocol::function_code::Function;
use modbus_protocol::utils;
use num_traits::FromPrimitive;

const MODBUS_PROTOCOL_TCP: u16 = 0x0000;
const MODBUS_TCP_DEFAULT_PORT: u16 = 502;
const MODBUS_HEADER_SIZE: usize = 7;
const MODBUS_MAX_PACKET_SIZE: usize = 260;

/// ModbusConfig structure for more control over the tcp socket settings
#[derive(Clone, Copy)]
pub struct ModbusConfig {
    /// The TCP port to use for communication (Default: `502`)
    pub tcp_port: u16,
    /// Connection timeout for TCP socket (Default: `OS Default`)
    pub tcp_connect_timeout: Option<Duration>,
    /// Timeout when reading from the TCP socket (Default: `infinite`)
    pub tcp_read_timeout: Option<Duration>,
    /// Timeout when writing to the TCP socket (Default: `infinite`)
    pub tcp_write_timeout: Option<Duration>,
    /// The modbus Unit Identifier used in the modbus layer (Default: `1`)
    pub modbus_uid: u8,
}

impl Default for ModbusConfig {
    fn default() -> ModbusConfig {
        ModbusConfig {
            tcp_port: MODBUS_TCP_DEFAULT_PORT,
            tcp_connect_timeout: None,
            tcp_read_timeout: None,
            tcp_write_timeout: None,
            modbus_uid: 1,
        }
    }
}

impl ModbusConfig {
    pub fn set_port(self: &mut Self, port: u16) {
        self.tcp_port = port;
    }
    pub fn set_uid(self: &mut Self, uid: u8) {
        self.modbus_uid = uid;
    }
}


#[derive(Debug, PartialEq)]
struct Header {
    tid: u16,
    pid: u16,
    len: u16,
    uid: u8,
}

impl Header {
    fn new(transport: &mut Transport, len: u16) -> Header {
        Header {
            tid: transport.new_tid(),
            pid: MODBUS_PROTOCOL_TCP,
            len: len - MODBUS_HEADER_SIZE as u16,
            uid: transport.uid,
        }
    }

    fn pack(&self) -> Result<Vec<u8>> {
        let mut buff = vec![];
        buff.write_u16::<BigEndian>(self.tid)?;
        buff.write_u16::<BigEndian>(self.pid)?;
        buff.write_u16::<BigEndian>(self.len)?;
        buff.write_u8(self.uid)?;
        Ok(buff)
    }

    fn unpack(buff: &[u8]) -> Result<Header> {
        let mut rdr = Cursor::new(buff);
        Ok(Header {
            tid: rdr.read_u16::<BigEndian>()?,
            pid: rdr.read_u16::<BigEndian>()?,
            len: rdr.read_u16::<BigEndian>()?,
            uid: rdr.read_u8()?,
        })
    }
}

/// Context object which holds state for all modbus operations.
pub struct Transport {
    tid: u16,
    uid: u8,
    stream: TcpStream,
}

impl Transport {
    /// Create a new context object and connect it to `addr` on port `port`
    pub fn new_with_cfg(addr: &str, cfg: ModbusConfig) -> io::Result<Transport> {
        let stream = match cfg.tcp_connect_timeout {
            Some(timeout) => {
                // Call to connect_timeout needs to be done on a single address
                let mut socket_addrs = (addr, cfg.tcp_port).to_socket_addrs()?;
                TcpStream::connect_timeout(&socket_addrs.next().unwrap(), timeout)
            }
            None => TcpStream::connect((addr, cfg.tcp_port)),
        };

        match stream {
            Ok(socket) => {
                let peer_addr = socket.peer_addr().unwrap();
                log::info!("connected server: {:?}", peer_addr);
                socket.set_read_timeout(cfg.tcp_read_timeout)?;
                socket.set_write_timeout(cfg.tcp_write_timeout)?;
                socket.set_nodelay(true)?;
                Ok(Transport {
                    tid: 0,
                    uid: cfg.modbus_uid,
                    stream: socket,
                })
            }
            Err(e) => Err(e),
        }
    }

    /// Set the unit identifier.
    pub fn _set_uid(&mut self, uid: u8) {
        self.uid = uid;
    }
    // Create a new transaction Id, incrementing the previous one.
    // The Id is wrapping around if the Id reaches `u16::MAX`.
    fn new_tid(&mut self) -> u16 {
        self.tid = self.tid.wrapping_add(1);
        self.tid
    }

    fn read(self: &mut Self, fun: &Function) -> Result<Vec<u8>> {
        let packed_size = |v: u16| v / 8 + if v % 8 > 0 { 1 } else { 0 };
        let (addr, count, expected_bytes) = match *fun {
            Function::ReadCoils(a, c) | Function::ReadDiscreteInputs(a, c) => {
                (a, c, packed_size(c) as usize)
            }
            Function::ReadHoldingRegisters(a, c) | Function::ReadInputRegisters(a, c) => {
                (a, c, 2 * c as usize)
            }
            _ => return Err(Error::InvalidFunction),
        };

        if count < 1 {
            return Err(Error::InvalidData(Reason::RecvBufferEmpty));
        }

        if count as usize > MODBUS_MAX_PACKET_SIZE {
            return Err(Error::InvalidData(Reason::UnexpectedReplySize));
        }

        let header = Header::new(self, MODBUS_HEADER_SIZE as u16 + 6u16);
        let mut buff = header.pack()?;
        buff.write_u8(fun.code())?;
        buff.write_u16::<BigEndian>(addr)?;
        buff.write_u16::<BigEndian>(count)?;

        match self.stream.write_all(&buff) {
            Ok(_s) => {
                let mut reply = vec![0; MODBUS_HEADER_SIZE + expected_bytes + 2];
                match self.stream.read(&mut reply) {
                    Ok(_s) => {
                        let resp_hd = Header::unpack(&reply[..MODBUS_HEADER_SIZE])?;
                        Transport::validate_response_header(&header, &resp_hd)?;
                        Transport::validate_response_code(&buff, &reply)?;
                        Transport::get_reply_data(&reply, expected_bytes)
                    }
                    Err(e) => Err(Error::Io(e)),
                }
            }
            Err(e) => Err(Error::Io(e)),
        }
    }

    fn validate_response_header(req: &Header, resp: &Header) -> Result<()> {
        if req.tid != resp.tid || resp.pid != MODBUS_PROTOCOL_TCP {
            log::info!("Invalid response header:");
            log::info!("   tid: expected: {}, result: {}", req.tid, resp.tid);
            log::info!("   pid: expected: {}, result: {}", MODBUS_PROTOCOL_TCP, resp.pid);
            Err(Error::InvalidResponse)
        } else {
            Ok(())
        }
    }

    fn validate_response_code(req: &[u8], resp: &[u8]) -> Result<()> {
        if req[7] + 0x80 == resp[7] {
            match ExceptionCode::from_u8(resp[8]) {
                Some(code) => Err(Error::Exception(code)),
                None => {
                    log::info!("Invalid Exception code: {}", resp[8]);
                    Err(Error::InvalidResponse)
                }
            }
        } else if req[7] == resp[7] {
            Ok(())
        } else {
            log::info!("invalid response code");
            log::info!("   expected: {}, result: {}", req[7], resp[7]);
            Err(Error::InvalidResponse)
        }
    }

    fn get_reply_data(reply: &[u8], expected_bytes: usize) -> Result<Vec<u8>> {
        if reply[8] as usize != expected_bytes
            || reply.len() != MODBUS_HEADER_SIZE + expected_bytes + 2
        {
            log::info!("Unexpected reply size");
            log::info!("   length field, expected: {}, result: {}",
                       expected_bytes, reply[8]);
            log::info!("   length expected: {}, result: {}",
                       MODBUS_HEADER_SIZE + expected_bytes + 2, reply.len());
            Err(Error::InvalidData(Reason::UnexpectedReplySize))
        } else {
            let mut d = Vec::new();
            d.extend_from_slice(&reply[MODBUS_HEADER_SIZE + 2..]);
            Ok(d)
        }
    }

    fn write_single(self: &mut Self, fun: &Function) -> Result<()> {
        let (addr, value) = match *fun {
            Function::WriteSingleCoil(a, v) | Function::WriteSingleRegister(a, v) => (a, v),
            _ => return Err(Error::InvalidFunction),
        };

        let mut buff = vec![0; MODBUS_HEADER_SIZE]; // Header gets filled in later
        buff.write_u8(fun.code())?;
        buff.write_u16::<BigEndian>(addr)?;
        buff.write_u16::<BigEndian>(value)?;
        self.write(&mut buff)
    }

    fn write_multiple(self: &mut Self, fun: &Function) -> Result<()> {
        let (addr, quantity, values) = match *fun {
            Function::WriteMultipleCoils(a, q, v) | Function::WriteMultipleRegisters(a, q, v) => {
                (a, q, v)
            }
            _ => return Err(Error::InvalidFunction),
        };

        let mut buff = vec![0; MODBUS_HEADER_SIZE]; // Header gets filled in later
        buff.write_u8(fun.code())?;
        buff.write_u16::<BigEndian>(addr)?;
        buff.write_u16::<BigEndian>(quantity)?;
        buff.write_u8(values.len() as u8)?;
        for v in values {
            buff.write_u8(*v)?;
        }
        self.write(&mut buff)
    }

    fn write(self: &mut Self, buff: &mut [u8]) -> Result<()> {
        if buff.is_empty() {
            return Err(Error::InvalidData(Reason::SendBufferEmpty));
        }

        if buff.len() > MODBUS_MAX_PACKET_SIZE {
            return Err(Error::InvalidData(Reason::SendBufferTooBig));
        }

        let header = Header::new(self, buff.len() as u16 + 1u16);
        let head_buff = header.pack()?;
        {
            let mut start = Cursor::new(buff.borrow_mut());
            start.write_all(&head_buff)?;
        }
        match self.stream.write_all(buff) {
            Ok(_s) => {
                let reply = &mut [0; 12];
                match self.stream.read(reply) {
                    Ok(_s) => {
                        let resp_hd = Header::unpack(reply)?;
                        Transport::validate_response_header(&header, &resp_hd)?;
                        Transport::validate_response_code(buff, reply)
                    }
                    Err(e) => Err(Error::Io(e)),
                }
            }
            Err(e) => Err(Error::Io(e)),
        }
    }

    pub fn _close(self: &mut Self) -> Result<()> {
        self.stream.shutdown(Shutdown::Both).map_err(Error::Io)
    }
}

impl Requests for Transport {
    /// Read `count` bits starting at address `addr`.
    fn read_coils(self: &mut Self, addr: u16, count: u16) -> Result<Vec<Coil>> {
        let bytes = self.read(&Function::ReadCoils(addr, count))?;
        Ok(utils::unpack_bits(&bytes, count))
    }

    /// Read `count` input bits starting at address `addr`.
    fn read_discrete_inputs(self: &mut Self, addr: u16, count: u16) -> Result<Vec<Coil>> {
        let bytes = self.read(&Function::ReadDiscreteInputs(addr, count))?;
        Ok(utils::unpack_bits(&bytes, count))
    }

    /// Read `count` 16bit registers starting at address `addr`.
    fn read_holding_registers(self: &mut Self, addr: u16, count: u16) -> Result<Vec<u16>> {
        let bytes = self.read(&Function::ReadHoldingRegisters(addr, count))?;
        utils::pack_bytes(&bytes[..])
    }

    /// Read `count` 16bit input registers starting at address `addr`.
    fn read_input_registers(self: &mut Self, addr: u16, count: u16) -> Result<Vec<u16>> {
        let bytes = self.read(&Function::ReadInputRegisters(addr, count))?;
        utils::pack_bytes(&bytes[..])
    }

    /// Write a single coil (bit) to address `addr`.
    fn write_single_coil(self: &mut Self, addr: u16, value: Coil) -> Result<()> {
        self.write_single(&Function::WriteSingleCoil(addr, value.code()))
    }

    /// Write a single 16bit register to address `addr`.
    fn write_single_register(self: &mut Self, addr: u16, value: u16) -> Result<()> {
        self.write_single(&Function::WriteSingleRegister(addr, value))
    }

    /// Write a multiple coils (bits) starting at address `addr`.
    fn write_multiple_coils(self: &mut Self, addr: u16, values: &[Coil]) -> Result<()> {
        let bytes = utils::pack_bits(values);
        self.write_multiple(&Function::WriteMultipleCoils(
            addr,
            values.len() as u16,
            &bytes,
        ))
    }

    /// Write a multiple 16bit registers starting at address `addr`.
    fn write_multiple_registers(self: &mut Self, addr: u16, values: &[u16]) -> Result<()> {
        let bytes = utils::unpack_bytes(values);
        self.write_multiple(&Function::WriteMultipleRegisters(
            addr,
            values.len() as u16,
            &bytes,
        ))
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn serialize_header() {
        let header = Header {
            tid: 12816,
            pid: 3930,
            len: 99,
            uid: 68,
        };
        let serialized = header.pack().unwrap();
        let deserialized = Header::unpack(&vec![50, 16, 15, 90, 0, 99, 68]).unwrap();
        let re_deserialized = Header::unpack(&serialized).unwrap();
        assert_eq!(serialized, vec![50, 16, 15, 90, 0, 99, 68]);
        assert_eq!(deserialized, header);
        assert_eq!(re_deserialized, header);
    }
}
