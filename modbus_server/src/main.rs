extern crate enum_primitive_derive;
extern crate num_traits;
extern crate clap;
extern crate modbus_protocol;
use clap::App;
use clap::crate_version;
use std::net::{Shutdown, TcpStream, TcpListener};
use std::io::{Read};
use std::{
    thread,
    sync::{Arc, Mutex},
};
use modbus_protocol::coils::Coil;

mod tcp;
use tcp::Config;
use tcp::MODBUS_MAX_PACKET_SIZE;

struct StatusInfo {
    registers: Vec<u16>,
    coils : Vec<Coil>,
}

impl StatusInfo {
    pub fn create(size: usize) -> StatusInfo {
        let mut status_info = StatusInfo {
                registers: Vec::with_capacity(size),
                coils: Vec::with_capacity(size)
        };
        for i in 0..size {
            status_info.registers[i] = 0;
        }
        for i in 0..size {
            status_info.coils[i] = Coil::Off;
        }
        status_info
    }

    pub fn set_status(&mut self, offset: usize, value: u16) {
        self.registers[offset] = value;
    }
}

fn handle_client(mut stream: TcpStream, _tid: u16, _uid: u8, shared_status: Arc<Mutex<StatusInfo>>){
    let mut data = [0 as u8; MODBUS_MAX_PACKET_SIZE];
    while match stream.read(&mut data) {
        Ok(size) => {
            println!("received {:?} bytes", size);
            true
        }
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {
        let mut _status = shared_status.lock().unwrap();
        _status.set_status(14, 17);
    }
}

fn main() {
    let mut children = vec![];
    let mut uid: u8 = 0;
    let mut tid: u16 = 0;
    let mut size: usize = 0;
    let matches = App::new("Modbus Server")
        .author("Zhaohui GUO <guo.zhaohui@gmail.com>")
        .version(&crate_version!()[..])
        .about("Modbus Tcp Server")
        .args_from_usage(
        "<SERVER> 'The IP address or hostname of the server'
                        \
                          --unit_id=[UID] 'unit identifier'
                        \
                          --capacity=[size] 'register number'",
        )
        .get_matches();

    let addr = matches.value_of("SERVER").unwrap();

    if let Some(args) = matches.values_of("unit_id") {
        let args: Vec<&str> = args.collect();
        uid = args[0].parse().expect(matches.usage());
    }
    if let Some(args) = matches.values_of("capacity") {
        let args: Vec<&str> = args.collect();
        size = args[0].parse().expect(matches.usage());
    }

    let status_info = Arc::new(Mutex::new(StatusInfo::create(size)));
    let config = Config::default();
    let listener = TcpListener::bind((addr, config.tcp_port)).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(_socket) => {
                println!("new client: {:?}", _socket.peer_addr().unwrap());
                let my_status = status_info.clone();
                children.push(thread::spawn(move|| {
                    handle_client(_socket, tid, uid, my_status)
                }));
            }
            Err(e) => {
                println!("couldn't get client: {:?}" , e);
            }
        }
        tid = tid.wrapping_add(1);
    }

    for child in children {
        let _ = child.join();
    }

    drop(listener);
}
