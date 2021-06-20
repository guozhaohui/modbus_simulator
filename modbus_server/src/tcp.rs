extern crate enum_primitive_derive;
extern crate num_traits;
extern crate clap;
extern crate modbus_protocol;
use std::net::{Shutdown, TcpStream};
use std::io::{Cursor, Read, Write};
use std::sync::{Arc, Mutex};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::borrow::BorrowMut;

use modbus_protocol::coils::Coil;
// use modbus_protocol::function_code::Function;
use modbus_protocol::requests::Requests;
use modbus_protocol::exception_code::{Error, ExceptionCode, Reason, Result};
use super::server_status::StatusInfo;
use super::mbap::Header;
use super::mbap::MODBUS_HEADER_SIZE;

const MODBUS_MAX_PACKET_SIZE: usize = 260;

fn write_response(stream: &mut TcpStream, header: Header,  buff: &mut [u8]) {
    if buff.is_empty() {
        return;
    }

    if buff.len() > MODBUS_MAX_PACKET_SIZE {
        return;
    }

    let head_buff = header.pack();
    {
        let mut start = Cursor::new(buff.borrow_mut());
        start.write_all(&head_buff.unwrap());
    }
    match stream.write_all(buff) {
        Ok(_s) => {
        },
        Err(e) => {
        },
    }
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
                let mut _status = shared_status.lock().unwrap();
                _status.set_status(14, 17);
                let header = Header::unpack(data).unwrap();
                let mut mbdata = &data[MODBUS_HEADER_SIZE + 2..];
                let code = mbdata.read_u8().unwrap();
                match code {
                    0x01 =>{
                        let addr= mbdata.read_u16::<BigEndian>().unwrap();
                        let count = mbdata.read_u16::<BigEndian>().unwrap();
                        let ret = _status.read_coils(addr, count);
                        let mut buff = vec![0; MODBUS_HEADER_SIZE];
                        match ret {
                            Ok(coils) => {
                                buff.write_u8(ExceptionCode::Acknowledge as u8).unwrap();
                                buff.write_u8(coils.len() as u8).unwrap();
                                for v in coils {
                                    buff.write_u16::<BigEndian>(v.code()).unwrap();
                                }
                            },
                            Err(e) => {
                                println!("something wrong {}", e);
                                buff.write_u8(ExceptionCode::IllegalDataValue as u8).unwrap();
                            }
                        }
                        write_response(&mut stream, header, &mut buff);
                    },
                    0x02 =>{
                        let addr= mbdata.read_u16::<BigEndian>().unwrap();
                        let count = mbdata.read_u16::<BigEndian>().unwrap();
                        let ret = _status.read_discrete_inputs(addr, count);
                        let mut buff = vec![0; MODBUS_HEADER_SIZE];
                        match ret {
                            Ok(coils) => {
                                buff.write_u8(ExceptionCode::Acknowledge as u8).unwrap();
                                buff.write_u8(coils.len() as u8).unwrap();
                                for v in coils {
                                    buff.write_u16::<BigEndian>(v.code()).unwrap();
                                }
                            },
                            Err(e) => {
                                println!("something wrong {}", e);
                                buff.write_u8(ExceptionCode::IllegalDataValue as u8).unwrap();
                            }
                        }
                        write_response(&mut stream, header, &mut buff);
                    },
                    0x03 =>{
                        let addr= mbdata.read_u16::<BigEndian>().unwrap();
                        let count = mbdata.read_u16::<BigEndian>().unwrap();
                        let ret = _status.read_holding_registers(addr, count);
                        let mut buff = vec![0; MODBUS_HEADER_SIZE];
                        match ret {
                            Ok(registers) => {
                                buff.write_u8(ExceptionCode::Acknowledge as u8).unwrap();
                                buff.write_u8(registers.len() as u8).unwrap();
                                for v in registers {
                                    buff.write_u16::<BigEndian>(v).unwrap();
                                }
                            },
                            Err(e) => {
                                println!("something wrong {}", e);
                                buff.write_u8(ExceptionCode::IllegalDataValue as u8).unwrap();
                            }
                        }
                        write_response(&mut stream, header, &mut buff);
                    },
                    0x04 =>{
                        let addr= mbdata.read_u16::<BigEndian>().unwrap();
                        let count = mbdata.read_u16::<BigEndian>().unwrap();
                        let ret = _status.read_input_registers(addr, count);
                        let mut buff = vec![0; MODBUS_HEADER_SIZE];
                        buff.write_u8(0x01 as u8).unwrap();
                        match ret {
                            Ok(registers) => {
                                buff.write_u8(ExceptionCode::Acknowledge as u8).unwrap();
                                buff.write_u8(registers.len() as u8).unwrap();
                                for v in registers {
                                    buff.write_u16::<BigEndian>(v).unwrap();
                                }
                            },
                            Err(e) => {
                                println!("something wrong {}", e);
                                buff.write_u8(ExceptionCode::IllegalDataValue as u8).unwrap();
                            }
                        }
                        write_response(&mut stream, header, &mut buff);
                    },
                    0x05 => {
                        let addr= mbdata.read_u16::<BigEndian>().unwrap();
                        let value = mbdata.read_u16::<BigEndian>().unwrap();
                        let ret = _status.write_single_coil(addr, Coil::from_u16(value).unwrap());
                        let mut buff = vec![0; MODBUS_HEADER_SIZE];
                        match ret {
                            Ok(()) => {
                                buff.write_u8(ExceptionCode::Acknowledge as u8).unwrap();
                            },
                            Err(e) => {
                                println!("something wrong {}", e);
                                buff.write_u8(ExceptionCode::IllegalDataValue as u8).unwrap();
                            }
                        }
                        write_response(&mut stream, header, &mut buff);
                    },
                    0x06 => {
                        let addr= mbdata.read_u16::<BigEndian>().unwrap();
                        let value = mbdata.read_u16::<BigEndian>().unwrap();
                        let ret = _status.write_single_register(addr, value);
                        let mut buff = vec![0; MODBUS_HEADER_SIZE];
                        match ret {
                            Ok(()) => {
                                buff.write_u8(ExceptionCode::Acknowledge as u8).unwrap();
                            },
                            Err(e) => {
                                println!("something wrong {}", e);
                                buff.write_u8(ExceptionCode::IllegalDataValue as u8).unwrap();
                            }
                        }
                        write_response(&mut stream, header, &mut buff);
                    },
                    0x0f => {
                        let addr= mbdata.read_u16::<BigEndian>().unwrap();
                        let count = mbdata.read_u16::<BigEndian>().unwrap();
                        let mut values :Vec<Coil> = Vec::with_capacity(count as usize);
                        for i in 0..count-1 {
                            values[i as usize] = Coil::from_u16(mbdata.read_u16::<BigEndian>().unwrap()).unwrap();
                        }
                        let ret = _status.write_multiple_coils(addr, &values[..]);
                        let mut buff = vec![0; MODBUS_HEADER_SIZE];
                        buff.write_u8(0x01 as u8).unwrap();
                        match ret {
                            Ok(()) => {
                                buff.write_u8(ExceptionCode::Acknowledge as u8).unwrap();
                            },
                            Err(e) => {
                                println!("something wrong {}", e);
                                buff.write_u8(ExceptionCode::IllegalDataValue as u8).unwrap();
                            }
                        }
                        write_response(&mut stream, header, &mut buff);
                    },
                    0x10 => {
                        let addr= mbdata.read_u16::<BigEndian>().unwrap();
                        let count = mbdata.read_u16::<BigEndian>().unwrap();
                        let mut values :Vec<u16> = Vec::with_capacity(count as usize);
                        for i in 0..count-1 {
                            values[i as usize] = mbdata.read_u16::<BigEndian>().unwrap();
                        }
                        let ret = _status.write_multiple_registers(addr, &values[..]);
                        let mut buff = vec![0; MODBUS_HEADER_SIZE];
                        buff.write_u8(0x01 as u8).unwrap();
                        match ret {
                            Ok(()) => {
                                buff.write_u8(ExceptionCode::Acknowledge as u8).unwrap();
                            },
                            Err(e) => {
                                println!("something wrong {}", e);
                                buff.write_u8(ExceptionCode::IllegalDataValue as u8).unwrap();
                            }
                        }
                        write_response(&mut stream, header, &mut buff);
                    },
                    _ => {
                    },
                }

            }
        }
    }
}


