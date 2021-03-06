use num_traits::FromPrimitive;
use std::net::{Shutdown, TcpStream};
use std::net::{SocketAddr};
use std::io::{Cursor, Read, Write};
use std::sync::{Arc, Mutex};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::borrow::BorrowMut;

use modbus_protocol::utils;
use modbus_protocol::coils::Coil;
use modbus_protocol::function_code::FunctionCode;
use modbus_protocol::requests::Requests;
use modbus_protocol::exception_code::{Error, ExceptionCode};
use super::server_status::StatusInfo;
use super::mbap::Header;
use super::mbap::MODBUS_HEADER_SIZE;

const MODBUS_MAX_PACKET_SIZE: usize = 260;

fn handle_status_error(function_code: u8, e: Error,  buff: &mut [u8]) {
    let mut start = Cursor::new(buff.borrow_mut());
    start.write_u8(function_code + 0x80).unwrap();
    match e {
        Error::Exception(code) => {
            start.write_u8(code as u8).unwrap();
        },
        _ => (),
    }
}
fn write_response(stream: &mut TcpStream, header: Header,  buff: &mut [u8]) {
    if buff.is_empty() {
        return;
    }

    if buff.len() > MODBUS_MAX_PACKET_SIZE {
        return;
    }

    let reply_header = Header::new(header.tid,
                                   header.uid,
                                   buff.len() as u16);

    let head_buff = reply_header.pack();
    let mut start = Cursor::new(buff.borrow_mut());
    match start.write_all(&head_buff.unwrap()) {
        Ok(_s) => {
        },
        Err(_e) => {
        },
    }
    match stream.write_all(buff) {
        Ok(_s) => {
            log::debug!("send reply message");
        },
        Err(_e) => {
        },
    }
}

pub fn handle_pdu_data(stream: &mut TcpStream, status: &mut StatusInfo, mbap_header: Header, data: &mut [u8]){
    let mut pdu_data = Cursor::new(data.borrow_mut());
    let function_code = pdu_data.read_u8().unwrap();
    let mut buff = vec![0; MODBUS_HEADER_SIZE];
    match FromPrimitive::from_u8(function_code) {
        Some(FunctionCode::ReadCoils) =>{
            let addr= pdu_data.read_u16::<BigEndian>().unwrap();
            let count = pdu_data.read_u16::<BigEndian>().unwrap();
            log::info!("request ReadCoils addr: {}; count: {}", addr, count);
            match status.read_coils(addr, count) {
                Ok(coils) => {
                    buff.write_u8(function_code).unwrap();
                    let bits = utils::pack_bits(&coils);
                    buff.write_u8(bits.len() as u8).unwrap();
                    for v in bits {
                        buff.write_u8(v).unwrap();
                    }
                },
                Err(e) => {
                    log::info!("something wrong {}", e);
                    handle_status_error(function_code, e, &mut buff);
                }
            }
        },
        Some(FunctionCode::ReadDiscreteInputs) =>{
            let addr= pdu_data.read_u16::<BigEndian>().unwrap();
            let count = pdu_data.read_u16::<BigEndian>().unwrap();
            log::info!("request ReadDiscreteInputs, addr: {}; count: {}", addr, count);
            match status.read_discrete_inputs(addr, count) {
                Ok(coils) => {
                    buff.write_u8(function_code).unwrap();
                    let bits = utils::pack_bits(&coils);
                    buff.write_u8(bits.len() as u8).unwrap();
                    for v in bits {
                        buff.write_u8(v).unwrap();
                    }
                },
                Err(e) => {
                    log::info!("something wrong {}", e);
                    handle_status_error(function_code, e, &mut buff);
                }
            }
        },
        Some(FunctionCode::ReadHoldingRegisters) =>{
            let addr= pdu_data.read_u16::<BigEndian>().unwrap();
            let count = pdu_data.read_u16::<BigEndian>().unwrap();
            log::info!("request ReadHoldingRegisters, addr: {}; count: {}", addr, count);
            let mut buff = vec![0; MODBUS_HEADER_SIZE];
            match status.read_holding_registers(addr, count) {
                Ok(registers) => {
                    buff.write_u8(function_code).unwrap();
                    buff.write_u8(registers.len() as u8 * 2).unwrap();
                    for v in registers {
                        buff.write_u16::<BigEndian>(v).unwrap();
                    }
                },
                Err(e) => {
                    log::info!("something wrong {}", e);
                    handle_status_error(function_code, e, &mut buff);
                }
            }
        },
        Some(FunctionCode::ReadInputRegisters) =>{
            let addr= pdu_data.read_u16::<BigEndian>().unwrap();
            let count = pdu_data.read_u16::<BigEndian>().unwrap();
            log::info!("request ReadInputRegisters, addr: {}; count: {}", addr, count);
            match status.read_input_registers(addr, count) {
                Ok(registers) => {
                    buff.write_u8(function_code).unwrap();
                    buff.write_u8(registers.len() as u8 * 2).unwrap();
                    for v in registers {
                        buff.write_u16::<BigEndian>(v).unwrap();
                    }
                },
                Err(e) => {
                    log::info!("something wrong {}", e);
                    handle_status_error(function_code, e, &mut buff);
                }
            }
        },
        Some(FunctionCode::WriteSingleCoil) => {
            let addr= pdu_data.read_u16::<BigEndian>().unwrap();
            log::info!("request WriteSingleCoil, addr: {}", addr);
            let value = pdu_data.read_u16::<BigEndian>().unwrap();
            match status.write_single_coil(addr, Coil::from_u16(value).unwrap()) {
                Ok(()) => {
                    buff.write_u8(function_code).unwrap();
                },
                Err(e) => {
                    log::info!("something wrong {}", e);
                    handle_status_error(function_code, e, &mut buff);
                }
            }
        },
        Some(FunctionCode::WriteSingleRegister) => {
            let addr= pdu_data.read_u16::<BigEndian>().unwrap();
            log::info!("request WriteSingleRegisters, addr: {}", addr);
            let value = pdu_data.read_u16::<BigEndian>().unwrap();
            match status.write_single_register(addr, value) {
                Ok(()) => {
                    buff.write_u8(function_code).unwrap();
                },
                Err(e) => {
                    log::info!("something wrong {}", e);
                    handle_status_error(function_code, e, &mut buff);
                }
            }
        },
        Some(FunctionCode::WriteMultipleCoils) => {
            let addr= pdu_data.read_u16::<BigEndian>().unwrap();
            let count = pdu_data.read_u16::<BigEndian>().unwrap();
            log::info!("request WriteMultipleCoils,  addr: {}; count: {}", addr, count);
            let mut values :Vec<Coil> = Vec::with_capacity(count as usize);
            for i in 0..count-1 {
                values[i as usize] = Coil::from_u16(pdu_data.read_u16::<BigEndian>().unwrap()).unwrap();
            }
            match status.write_multiple_coils(addr, &values[..]) {
                Ok(()) => {
                    buff.write_u8(function_code).unwrap();
                },
                Err(e) => {
                    log::info!("something wrong {}", e);
                    handle_status_error(function_code, e, &mut buff);
                }
            }
        },
        Some(FunctionCode::WriteMultipleRegisters) => {
            let addr= pdu_data.read_u16::<BigEndian>().unwrap();
            let count = pdu_data.read_u16::<BigEndian>().unwrap();
            log::info!("request WriteMultipleRegisters, addr: {}; count: {}", addr, count);
            let mut values :Vec<u16> = Vec::with_capacity(count as usize);
            for i in 0..count-1 {
                values[i as usize] = pdu_data.read_u16::<BigEndian>().unwrap();
            }
            match status.write_multiple_registers(addr, &values[..]) {
                Ok(()) => {
                    buff.write_u8(function_code).unwrap();
                },
                Err(e) => {
                    log::info!("something wrong {}", e);
                    handle_status_error(function_code, e, &mut buff);
                }
            }
        },
        _ => {
            buff.write_u8(function_code + 0x80).unwrap();
            buff.write_u8(ExceptionCode::IllegalFunction as u8).unwrap();
        },
    }
    write_response(stream, mbap_header, &mut buff);
}

pub fn handle_client(mut stream: TcpStream, _tid: u16, _uid: u8,
                     shared_status: Arc<Mutex<StatusInfo>>,
                     peer_addr: &SocketAddr){
    let data = &mut [0 as u8; MODBUS_MAX_PACKET_SIZE];
    loop {
        match stream.read(data) {
            Err(_) => {
                log::info!("connection with {} terminated", peer_addr.to_string());
                match stream.shutdown(Shutdown::Both) {
                    Err(e) => {
                        log::warn!("connection with {} shutdown failed, {}",
                        peer_addr.to_string(), e);
                    },
                    Ok(_) => {
                    }
                }
                break;
            },
            Ok(size) => {
                if size > 0 {
                    let mut status = shared_status.lock().unwrap();
                    let mbap_header = Header::unpack(data).unwrap();
                    let pdu_data = &mut data[MODBUS_HEADER_SIZE..];
                    handle_pdu_data(&mut stream, &mut status, mbap_header, pdu_data);
                }
            }
        }
    }
}


