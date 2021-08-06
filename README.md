[![Rust](https://github.com/guozhaohui/modbus_simulator/actions/workflows/rust.yml/badge.svg)](https://github.com/guozhaohui/modbus_simulator/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
# modbus_simulator
A modbus simulator written by rust.

## DESCRIPTION
This project is derived from https://github.com/hirschenberger/modbus-rs, with the following modifications aimed to provide full-set function simulation of modbus protocol.

* Retrieve common implementation of Server/Client side to form a new modbus_protocol crate.
* Create modbus_client crate which link with modbus_protocol crate.
* Add new modbus_server crate to provide server side implementation.

##  USAGE
* launch modbus server

  > $ ./modbus_server 127.0.0.1 --port=1234 --unit_id=5 --capacity=64

* launch modbus client

  > $ ./modbus_client 127.0.0.1 --port=1234 --read-coils 5 3
  
  
  You can use *--help* option for detailed usage and more other command options.
  
## Explanation
1) The default Modbus TCP port number is 502, but the TCP/IP port numbers below 1024 are special in that normal users are not allowed to run servers on them, so you maybe encounter a permission denied error when you use the default port. *--port* is provided to enable the modbus server to bind on other port for test without root privilege.

2) Multiple clients can connect to the same server simultaneously. 

3) Crate _log4rs_ is used to provide logging function on both server side and client side. Two .yaml files in config_samples folder are example files for log4rs configuration, you can modify them to fulfill your needs.
