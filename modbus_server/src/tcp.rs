extern crate num_derive;
extern crate num_traits;
extern crate clap;
extern crate modbus_protocol;
use num_traits::FromPrimitive;
use std::net::{Shutdown, TcpStream};
use std::io::{Cursor, Read, Write};
use std::sync::{Arc, Mutex};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::borrow::BorrowMut;

use modbus_protocol::coils::Coil;
use modbus_protocol::function_code::FunctionCode;
use modbus_protocol::requests::Requests;
use modbus_protocol::exception_code::{Error, ExceptionCode};
use super::server_status::StatusInfo;
use super::mbap::Header;
use super::mbap::MODBUS_HEADER_SIZE;

const MODBUS_MAX_PACKET_SIZE: usize = 260;

fn handle_status_error(e: Error,  buff: &mut [u8]) {
    let mut start = Cursor::new(buff.borrow_mut());
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
            match status.read_coils(addr, count) {
                Ok(coils) => {
                    buff.write_u8(ExceptionCode::Acknowledge as u8).unwrap();
                    buff.write_u8(coils.len() as u8).unwrap();
                    for v in coils {
                        buff.write_u16::<BigEndian>(v.code()).unwrap();
                    }
                },
                Err(e) => {
                    println!("something wrong {}", e);
                    handle_status_error(e, &mut buff);
                }
            }
        },
        Some(FunctionCode::ReadDiscreteInputs) =>{
            let addr= pdu_data.read_u16::<BigEndian>().unwrap();
            let count = pdu_data.read_u16::<BigEndian>().unwrap();
            match status.read_discrete_inputs(addr, count) {
                Ok(coils) => {
                    buff.write_u8(ExceptionCode::Acknowledge as u8).unwrap();
                    buff.write_u8(coils.len() as u8).unwrap();
                    for v in coils {
                        buff.write_u16::<BigEndian>(v.code()).unwrap();
                    }
                },
                Err(e) => {
                    println!("something wrong {}", e);
                    handle_status_error(e, &mut buff);
                }
            }
        },
        Some(FunctionCode::ReadHoldingRegisters) =>{
            let addr= pdu_data.read_u16::<BigEndian>().unwrap();
            let count = pdu_data.read_u16::<BigEndian>().unwrap();
            let mut buff = vec![0; MODBUS_HEADER_SIZE];
            match status.read_holding_registers(addr, count) {
                Ok(registers) => {
                    buff.write_u8(ExceptionCode::Acknowledge as u8).unwrap();
                    buff.write_u8(registers.len() as u8).unwrap();
                    for v in registers {
                        buff.write_u16::<BigEndian>(v).unwrap();
                    }
                },
                Err(e) => {
                    println!("something wrong {}", e);
                    handle_status_error(e, &mut buff);
                }
            }
        },
        Some(FunctionCode::ReadInputRegisters) =>{
            let addr= pdu_data.read_u16::<BigEndian>().unwrap();
            let count = pdu_data.read_u16::<BigEndian>().unwrap();
            match status.read_input_registers(addr, count) {
                Ok(registers) => {
                    buff.write_u8(ExceptionCode::Acknowledge as u8).unwrap();
                    buff.write_u8(registers.len() as u8).unwrap();
                    for v in registers {
                        buff.write_u16::<BigEndian>(v).unwrap();
                    }
                },
                Err(e) => {
                    println!("something wrong {}", e);
                    handle_status_error(e, &mut buff);
                }
            }
        },
        Some(FunctionCode::WriteSingleCoil) => {
            let addr= pdu_data.read_u16::<BigEndian>().unwrap();
            let value = pdu_data.read_u16::<BigEndian>().unwrap();
            match status.write_single_coil(addr, Coil::from_u16(value).unwrap()) {
                Ok(()) => {
                    buff.write_u8(ExceptionCode::Acknowledge as u8).unwrap();
                },
                Err(e) => {
                    println!("something wrong {}", e);
                    handle_status_error(e, &mut buff);
                }
            }
        },
        Some(FunctionCode::WriteSingleRegister) => {
            let addr= pdu_data.read_u16::<BigEndian>().unwrap();
            let value = pdu_data.read_u16::<BigEndian>().unwrap();
            match status.write_single_register(addr, value) {
                Ok(()) => {
                    buff.write_u8(ExceptionCode::Acknowledge as u8).unwrap();
                },
                Err(e) => {
                    println!("something wrong {}", e);
                    handle_status_error(e, &mut buff);
                }
            }
        },
        Some(FunctionCode::WriteMultipleCoils) => {
            let addr= pdu_data.read_u16::<BigEndian>().unwrap();
            let count = pdu_data.read_u16::<BigEndian>().unwrap();
            let mut values :Vec<Coil> = Vec::with_capacity(count as usize);
            for i in 0..count-1 {
                values[i as usize] = Coil::from_u16(pdu_data.read_u16::<BigEndian>().unwrap()).unwrap();
            }
            match status.write_multiple_coils(addr, &values[..]) {
                Ok(()) => {
                    buff.write_u8(ExceptionCode::Acknowledge as u8).unwrap();
                },
                Err(e) => {
                    println!("something wrong {}", e);
                    handle_status_error(e, &mut buff);
                }
            }
        },
        Some(FunctionCode::WriteMultipleRegisters) => {
            let addr= pdu_data.read_u16::<BigEndian>().unwrap();
            let count = pdu_data.read_u16::<BigEndian>().unwrap();
            let mut values :Vec<u16> = Vec::with_capacity(count as usize);
            for i in 0..count-1 {
                values[i as usize] = pdu_data.read_u16::<BigEndian>().unwrap();
            }
            match status.write_multiple_registers(addr, &values[..]) {
                Ok(()) => {
                    buff.write_u8(ExceptionCode::Acknowledge as u8).unwrap();
                },
                Err(e) => {
                    println!("something wrong {}", e);
                    handle_status_error(e, &mut buff);
                }
            }
        },
        _ => {
            buff.write_u8(ExceptionCode::IllegalFunction as u8).unwrap();
        },
    }
    write_response(stream, mbap_header, &mut buff);
}

pub fn handle_client(mut stream: TcpStream, _tid: u16, _uid: u8, shared_status: Arc<Mutex<StatusInfo>>){
    let data = &mut [0 as u8; MODBUS_MAX_PACKET_SIZE];
    loop {
        match stream.read(data) {
            Err(_) => {
                println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                stream.shutdown(Shutdown::Both).unwrap();
                break;
            }
            Ok(size) => {
                println!("received {:?} bytes", size);
                let mut status = shared_status.lock().unwrap();
                let mbap_header = Header::unpack(data).unwrap();
                let pdu_data = &mut data[MODBUS_HEADER_SIZE..];
                handle_pdu_data(&mut stream, &mut status, mbap_header, pdu_data);
            }
        }
    }
}


