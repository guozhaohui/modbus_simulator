extern crate num_derive;
extern crate num_traits;
extern crate clap;
extern crate modbus_protocol;
use clap::App;
use clap::crate_version;
use std::net::{TcpListener};
use std::{
    thread,
    sync::{Arc, Mutex},
};
use std::time::Duration;

mod server_status;
mod mbap;
use server_status::StatusInfo;

mod tcp;
const MODBUS_TCP_DEFAULT_PORT: u16 = 502;

/// Config structure for more control over the tcp socket settings
#[derive(Clone, Copy)]
pub struct Config {
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

impl Default for Config {
    fn default() -> Config {
        Config {
            tcp_port: MODBUS_TCP_DEFAULT_PORT,
            tcp_connect_timeout: None,
            tcp_read_timeout: None,
            tcp_write_timeout: None,
            modbus_uid: 1,
        }
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
                    tcp::handle_client(_socket, tid, uid, my_status)
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
