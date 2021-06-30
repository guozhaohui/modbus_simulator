# modbus_simulator
A modbus simulator written by rust.

This project is derived from https://github.com/hirschenberger/modbus-rs, with the following modifications aimed to provide full-set function simulation of modbus protocol.

* Retrieve common implementation of Server/Client side to form a new modbus_protocol crate.
* Create modbus_client crate which link with modbus_protocol crate.
* Add new modbus_server crate to provide server side implementation.



    
