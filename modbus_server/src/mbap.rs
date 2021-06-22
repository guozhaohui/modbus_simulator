extern crate modbus_protocol;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;
use modbus_protocol::exception_code::{Error, Reason, Result};
const MODBUS_PROTOCOL_TCP: u16 = 0x0000;
pub const MODBUS_HEADER_SIZE: usize = 7;
#[derive(Debug, PartialEq)]
pub struct Header {
    pub tid: u16,
    pub pid: u16,
    pub len: u16,
    pub uid: u8,
}

impl Header {
    pub fn new(tid: u16, uid: u8, len: u16) -> Header {
        Header {
            tid: tid,
            pid: MODBUS_PROTOCOL_TCP,
            len: len - MODBUS_HEADER_SIZE as u16,
            uid: uid,
        }
    }

    pub fn pack(&self) -> Result<Vec<u8>> {
        let mut buff = vec![];
        buff.write_u16::<BigEndian>(self.tid)?;
        buff.write_u16::<BigEndian>(self.pid)?;
        buff.write_u16::<BigEndian>(self.len)?;
        buff.write_u8(self.uid)?;
        Ok(buff)
    }

    pub fn unpack(buff: &[u8]) -> Result<Header> {
        let mut rdr = Cursor::new(buff);
        Ok(Header {
            tid: rdr.read_u16::<BigEndian>()?,
            pid: rdr.read_u16::<BigEndian>()?,
            len: rdr.read_u16::<BigEndian>()?,
            uid: rdr.read_u8()?,
        })
    }

    fn _get_data(reply: &[u8], expected_bytes: usize) -> Result<Vec<u8>> {
        if reply[8] as usize != expected_bytes
            || reply.len() != MODBUS_HEADER_SIZE + expected_bytes + 2
        {
            Err(Error::InvalidData(Reason::UnexpectedReplySize))
        } else {
            let mut d = Vec::new();
            d.extend_from_slice(&reply[MODBUS_HEADER_SIZE + 2..]);
            Ok(d)
        }
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

    #[test]
    fn create_header() {
        let header = Header::new(1u16, 10u8, 100u16);
        assert_eq!(header.pid, 0u16);
        assert_eq!(header.len, (100-7) as u16);
    }
}

