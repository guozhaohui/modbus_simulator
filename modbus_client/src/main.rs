extern crate num_derive;
extern crate num_traits;
extern crate clap;
extern crate modbus_protocol;
extern crate log;
extern crate log4rs;

use clap::App;
use clap::crate_version;
use modbus_protocol::coils::Coil;
use modbus_protocol::requests::Requests;
use modbus_protocol::exception_code::{Error};
mod tcp;
use tcp::ModbusConfig;

fn handle_error(e: Error) {
    log::info!("failed with {}", e);
}

fn main() {

  #[cfg(feature="log4rs_yaml")]
  log4rs::init_file("modbus_client_log.yaml", Default::default()).unwrap();
  #[cfg(not(feature="log4rs_yaml"))]
  {
      let logfile = log4rs::append::file::FileAppender::builder()
          .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new("{d} - {m}{n}")))
          .build("log/modbus_client.log").unwrap();

      let log4rs_config = log4rs::config::Config::builder()
          .appender(log4rs::config::Appender::builder().build("logfile", Box::new(logfile)))
          .build(log4rs::config::Root::builder()
                     .appender("logfile")
                     .build(log::LevelFilter::Info)).unwrap();

      log4rs::init_config(log4rs_config).unwrap();
  }
  let matches = App::new("client")
    .author("Zhaohui GUO <guo.zhaohui@gmail.com>")
    .version(&crate_version!()[..])
    .about("Modbus Tcp client")
    .args_from_usage(
      "<SERVER> 'The IP address or hostname of the server'
                        \
                          --port=[port] 'port number'
                        \
                          --unit_id=[UID] 'unit identifier'
                        \
                          --read-coils=[ADDR] [QUANTITY] 'Read QUANTITY coils from ADDR'
                        \
                          --read-discrete-inputs=[ADDR] [QUANTITY] 'Read QUANTITY inputs from \
                          ADDR'
                        --write-single-coil=[ADDR] [On,Off] \
                          'Write the coil value (On or Off) to ADDR'
                        \
                          --write-multiple-coils=[ADDR] [On,Off..] 'Write multiple coil values \
                          (On or Off) to ADDR (use \"..\" without spaces to group them e.g. \
                          \"On, Off, On, Off\")'
                        \
                          --read-input-registers=[ADDR], [QUANTITY] 'Read QUANTITY input \
                          registersfrom ADDR'
                        \
                          --read-holding-registers=[ADDR], [QUANTITY] 'Read QUANTITY holding \
                          registers from ADDR'
                        \
                          --write-single-register=[ADDR] [VALUE] 'Write VALUE to register ADDR'
                        \
                          --write-multiple-registers=[ADDR] [V1,V2...] 'Write multiple register \
                          values to ADDR (use \"..\" to group them e.g. \"23, 24, 25\")'",
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
      let uid = args[0].parse().expect(matches.usage());
      modbus_config.set_uid(uid);
  }
  let mut client = tcp::Transport::new_with_cfg(addr, modbus_config).unwrap();

  if let Some(args) = matches.values_of("read-coils") {
    let args: Vec<&str> = args.collect();
    let addr: u16 = args[0].parse().expect(matches.usage());
    let qtty: u16 = args[1].parse().expect(matches.usage());
    match client.read_coils(addr, qtty) {
        Err(e) =>{
            handle_error(e);
        },
        Ok(_) => {
            log::info!("Succeeded");
        }
    };
  } else if let Some(args) = matches.values_of("read-discrete-inputs") {
    let args: Vec<&str> = args.collect();
    let addr: u16 = args[0].parse().expect(matches.usage());
    let qtty: u16 = args[1].parse().expect(matches.usage());
    match client.read_discrete_inputs(addr, qtty) {
        Err(e) =>{
            handle_error(e);
        },
        Ok(_) => {
            log::info!("Succeeded");
        }
    };
  } else if let Some(args) = matches.values_of("write-single-coil") {
    let args: Vec<&str> = args.collect();
    let addr: u16 = args[0].parse().expect(matches.usage());
    let value: Coil = args[1].parse().expect(matches.usage());
    match client.write_single_coil(addr, value) {
        Err(e) =>{
            handle_error(e);
        },
        Ok(_) => {
            log::info!("Succeeded");
        }
    };
  } else if let Some(args) = matches.values_of("write-multiple-coils") {
    let args: Vec<&str> = args.collect();
    let addr: u16 = args[0].parse().expect(matches.usage());
    let values: Vec<Coil> = args[1]
      .split(',')
      .map(|s| s.trim().parse().expect(matches.usage()))
      .collect();
    match client.write_multiple_coils(addr, &values) {
        Err(e) =>{
            handle_error(e);
        },
        Ok(_) => {
            log::info!("Succeeded");
        }
    };
  } else if let Some(args) = matches.values_of("read-holding-registers") {
    let args: Vec<&str> = args.collect();
    let addr: u16 = args[0].parse().expect(matches.usage());
    let qtty: u16 = args[1].parse().expect(matches.usage());
    match client.read_holding_registers(addr, qtty) {
        Err(e) =>{
            handle_error(e);
        },
        Ok(_) => {
            log::info!("Succeeded");
        }
    };
  } else if let Some(args) = matches.values_of("write-single-register") {
    let args: Vec<&str> = args.collect();
    let addr: u16 = args[0].parse().expect(matches.usage());
    let value: u16 = args[1].parse().expect(matches.usage());
    match client.write_single_register(addr, value){
        Err(e) =>{
            handle_error(e);
        },
        Ok(_) => {
            log::info!("Succeeded");
        }
    };
  } else if let Some(args) = matches.values_of("write-multiple-registers") {
    let args: Vec<&str> = args.collect();
    let addr: u16 = args[0].parse().expect(matches.usage());
    let values: Vec<u16> = args[1]
      .split(',')
      .map(|s| s.trim().parse().expect(matches.usage()))
      .collect();
    match client.write_multiple_registers(addr, &values) {
        Err(e) =>{
            handle_error(e);
        },
        Ok(_) => {
            log::info!("Succeeded");
        }
    };
  };
}
