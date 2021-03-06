extern crate num_derive;
extern crate num_traits;
extern crate clap;
extern crate modbus_protocol;
extern crate log;
extern crate log4rs;

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
    fn set_port(self: &mut Self, port: u16) {
        self.tcp_port = port;
    }
    fn set_uid(self: &mut Self, uid: u8) {
        self.modbus_uid = uid;
    }
}

fn main() {
    let mut children = vec![];
    let mut uid: u8 = 0;
    let mut tid: u16 = 0;
    let mut size: usize = 0;

    #[cfg(feature="log4rs_yaml")]
    log4rs::init_file("modbus_server_log.yaml", Default::default()).unwrap();
    #[cfg(not(feature="log4rs_yaml"))]
    {
        let logfile = log4rs::append::file::FileAppender::builder()
            .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new("{d} - {T} - {m}{n}")))
            .build("log/modbus_server.log").unwrap();

        let log4rs_config = log4rs::config::Config::builder()
            .appender(log4rs::config::Appender::builder().build("logfile", Box::new(logfile)))
            .build(log4rs::config::Root::builder()
                       .appender("logfile")
                       .build(log::LevelFilter::Info)).unwrap();

        log4rs::init_config(log4rs_config).unwrap();
    }

    let matches = App::new("Modbus Server")
        .author("Zhaohui GUO <guo.zhaohui@gmail.com>")
        .version(&crate_version!()[..])
        .about("Modbus Tcp Server")
        .args_from_usage(
        "<SERVER> 'The IP address or hostname of the server'
                        \
                          --port=[port] 'port number'
                        \
                          --unit_id=[UID] 'unit identifier'
                        \
                          --capacity=[size] 'register number'",
        )
        .get_matches();

    let mut modbus_config = ModbusConfig::default();
    let addr = matches.value_of("SERVER").unwrap();
    if let Some(args) = matches.values_of("port") {
        let args: Vec<&str> = args.collect();
        let port = args[0].parse().expect(matches.usage());
        modbus_config.set_port(port);
    }

    if let Some(args) = matches.values_of("unit_id") {
        let args: Vec<&str> = args.collect();
        uid = args[0].parse().expect(matches.usage());
        modbus_config.set_uid(uid);
    }
    if let Some(args) = matches.values_of("capacity") {
        let args: Vec<&str> = args.collect();
        size = args[0].parse().expect(matches.usage());
    }

    let status_info = Arc::new(Mutex::new(StatusInfo::create(size)));
    let listener = TcpListener::bind((addr, modbus_config.tcp_port)).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(_socket) => {
                let peer_addr = _socket.peer_addr().unwrap();
                log::info!("new client: {:?}", peer_addr);
                let my_status = status_info.clone();
                children.push(thread::Builder::new().name(peer_addr.to_string()).spawn(move|| {
                    tcp::handle_client(_socket, tid, uid, my_status, &peer_addr)
                }).unwrap());
            }
            Err(e) => {
                log::info!("failed to accept a client: {:?}" , e);
            }
        }
        tid = tid.wrapping_add(1);
    }

    for child in children {
        let _ = child.join();
    }

    drop(listener);
}
