var searchIndex = JSON.parse('{\
"modbus_client":{"doc":"","t":[5,5,5,0,4,3,17,17,17,17,3,13,13,13,13,3,13,13,13,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,11,11,11,11,11,12,11,11,11,11,11,11,11,12,12,12,12,12,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12],"n":["handle_error","handle_request","main","tcp","Function","Header","MODBUS_HEADER_SIZE","MODBUS_MAX_PACKET_SIZE","MODBUS_PROTOCOL_TCP","MODBUS_TCP_DEFAULT_PORT","ModbusConfig","ReadCoils","ReadDiscreteInputs","ReadHoldingRegisters","ReadInputRegisters","Transport","WriteMultipleCoils","WriteMultipleRegisters","WriteSingleCoil","WriteSingleRegister","_close","_set_uid","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone_any","clone_any_send","clone_any_send_sync","clone_any_sync","clone_into","code","default","eq","fmt","from","from","from","from","get_reply_data","into","into","into","into","len","modbus_uid","ne","new","new_tid","new_with_cfg","pack","pid","read","read_coils","read_discrete_inputs","read_holding_registers","read_input_registers","set_port","set_uid","stream","tcp_connect_timeout","tcp_port","tcp_read_timeout","tcp_write_timeout","tid","tid","to_owned","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","uid","uid","unpack","validate_response_code","validate_response_header","write","write_multiple","write_multiple_coils","write_multiple_registers","write_single","write_single_coil","write_single_register","0","0","0","0","0","0","0","0","1","1","1","1","1","1","1","1","2","2"],"q":["modbus_client","","","","modbus_client::tcp","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","modbus_client::tcp::Function","","","","","","","","","","","","","","","","",""],"d":["","","","","","","","","","","ModbusConfig structure for more control over the tcp …","","","","","Context object which holds state for all modbus …","","","","","","Set the unit identifier.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","The modbus Unit Identifier used in the modbus layer …","","","","Create a new context object and connect it to <code>addr</code> on …","","","","Read <code>count</code> bits starting at address <code>addr</code>.","Read <code>count</code> input bits starting at address <code>addr</code>.","Read <code>count</code> 16bit registers starting at address <code>addr</code>.","Read <code>count</code> 16bit input registers starting at address <code>addr</code>.","","","","Connection timeout for TCP socket (Default: <code>OS Default</code>)","The TCP port to use for communication (Default: <code>502</code>)","Timeout when reading from the TCP socket (Default: …","Timeout when writing to the TCP socket (Default: <code>infinite</code>)","","","","","","","","","","","","","","","","","","","","","","","Write a multiple coils (bits) starting at address <code>addr</code>.","Write a multiple 16bit registers starting at address <code>addr</code>.","","Write a single coil (bit) to address <code>addr</code>.","Write a single 16bit register to address <code>addr</code>.","","","","","","","","","","","","","","","","","",""],"i":[0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,0,1,1,1,1,2,2,1,2,3,4,1,2,3,4,3,3,3,3,3,3,1,3,4,4,1,2,3,4,2,1,2,3,4,4,3,4,4,2,2,4,4,2,2,2,2,2,3,3,2,3,3,3,3,2,4,3,1,2,3,4,1,2,3,4,1,2,3,4,2,4,4,2,2,2,2,2,2,2,2,2,5,6,7,8,9,10,11,12,5,6,7,8,9,10,11,12,11,12],"f":[[[["error",4]]],[[["transport",3],["modbusfunction",4]]],[[]],null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[],["result",6]],[[["u8",15]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["modbusconfig",3]],[[],[["global",3],["box",3,["cloneany","global"]],["cloneany",8]]],[[],[["global",3],["cloneany",8],["box",3,["cloneany","global"]]]],[[],[["box",3,["cloneany","global"]],["cloneany",8],["global",3]]],[[],[["cloneany",8],["box",3,["cloneany","global"]],["global",3]]],[[]],[[],["u8",15]],[[],["modbusconfig",3]],[[["header",3]],["bool",15]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[["usize",15]],[["vec",3,["u8"]],["result",6,["vec"]]]],[[]],[[]],[[]],[[]],null,null,[[["header",3]],["bool",15]],[[["transport",3],["u16",15]],["header",3]],[[],["u16",15]],[[["str",15],["modbusconfig",3]],[["result",6,["transport"]],["transport",3]]],[[],[["vec",3,["u8"]],["result",6,["vec"]]]],null,[[["function",4]],[["vec",3,["u8"]],["result",6,["vec"]]]],[[["u16",15]],[["vec",3,["coil"]],["result",6,["vec"]]]],[[["u16",15]],[["vec",3,["coil"]],["result",6,["vec"]]]],[[["u16",15]],[["result",6,["vec"]],["vec",3,["u16"]]]],[[["u16",15]],[["result",6,["vec"]],["vec",3,["u16"]]]],[[["u16",15]]],[[["u8",15]]],null,null,null,null,null,null,null,[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,null,[[],[["header",3],["result",6,["header"]]]],[[],["result",6]],[[["header",3]],["result",6]],[[],["result",6]],[[["function",4]],["result",6]],[[["u16",15]],["result",6]],[[["u16",15]],["result",6]],[[["function",4]],["result",6]],[[["u16",15],["coil",4]],["result",6]],[[["u16",15]],["result",6]],null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null],"p":[[4,"Function"],[3,"Transport"],[3,"ModbusConfig"],[3,"Header"],[13,"ReadCoils"],[13,"ReadDiscreteInputs"],[13,"ReadHoldingRegisters"],[13,"ReadInputRegisters"],[13,"WriteSingleCoil"],[13,"WriteSingleRegister"],[13,"WriteMultipleCoils"],[13,"WriteMultipleRegisters"]]},\
"modbus_protocol":{"doc":"","t":[0,0,0,0,0,4,13,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,13,13,13,13,13,4,13,4,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,4,13,6,13,13,13,13,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,6,6,4,4,6,13,13,13,13,13,13,13,13,6,6,13,13,13,13,13,13,13,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,8,10,10,10,10,10,10,10,10,5,5,5,5],"n":["coils","exception_code","function_code","requests","utils","Coil","Off","On","borrow","borrow_mut","clone","clone_into","code","eq","fmt","from","from","from_str","from_u16","into","not","to_owned","try_from","try_into","type_id","Acknowledge","BytecountNotEven","Custom","DecodingError","EncodingError","Error","Exception","ExceptionCode","GatewayPath","GatewayTarget","IllegalDataAddress","IllegalDataValue","IllegalFunction","InvalidByteorder","InvalidData","InvalidFunction","InvalidRequestParameter","InvalidResponse","Io","MemoryParity","NegativeAcknowledge","NotDefined","ParseCoilError","Reason","RecvBufferEmpty","Result","SendBufferEmpty","SendBufferTooBig","SlaveOrServerBusy","SlaveOrServerFailure","UnexpectedReplySize","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","cause","description","eq","eq","fmt","fmt","fmt","fmt","from","from","from","from","from","from_i64","from_u64","into","into","into","ne","to_string","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","0","0","0","0","Address","Coils","FunctionCode","ModbusFunction","Quantity","ReadCoils","ReadCoils","ReadDiscreteInputs","ReadDiscreteInputs","ReadHoldingRegisters","ReadHoldingRegisters","ReadInputRegisters","ReadInputRegisters","Value","Values","WriteMultipleCoils","WriteMultipleCoils","WriteMultipleRegisters","WriteMultipleRegisters","WriteSingleCoil","WriteSingleCoil","WriteSingleRegister","WriteSingleRegister","borrow","borrow","borrow_mut","borrow_mut","from","from","from_i64","from_u64","into","into","try_from","try_from","try_into","try_into","type_id","type_id","0","0","0","0","0","0","0","0","1","1","1","1","1","1","1","1","Requests","read_coils","read_discrete_inputs","read_holding_registers","read_input_registers","write_multiple_coils","write_multiple_registers","write_single_coil","write_single_register","pack_bits","pack_bytes","unpack_bits","unpack_bytes"],"q":["modbus_protocol","","","","","modbus_protocol::coils","","","","","","","","","","","","","","","","","","","","modbus_protocol::exception_code","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","modbus_protocol::exception_code::Error","","","modbus_protocol::exception_code::Reason","modbus_protocol::function_code","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","modbus_protocol::function_code::ModbusFunction","","","","","","","","","","","","","","","","modbus_protocol::requests","","","","","","","","","modbus_protocol::utils","","",""],"d":["","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Combination of Modbus, IO and data corruption errors","","Modbus exception codes returned from the server.","","","","","","","","","","","","","","","","<code>InvalidData</code> reasons","","Result type used to nofify success or failure in …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,2,3,3,3,3,0,4,0,2,2,2,2,2,3,4,4,3,4,4,2,2,2,4,0,3,0,3,3,2,2,3,2,3,4,2,3,4,4,4,2,3,2,3,4,4,2,3,4,4,4,2,2,2,3,4,3,4,2,3,4,2,3,4,2,3,4,5,6,7,8,0,0,0,0,0,9,10,9,10,9,10,9,10,0,0,9,10,9,10,9,10,9,10,9,10,9,10,9,10,10,10,9,10,9,10,9,10,9,10,11,12,13,14,15,16,17,18,11,12,13,14,15,16,17,18,0,19,19,19,19,19,19,19,19,0,0,0,0],"f":[null,null,null,null,null,null,null,null,[[]],[[]],[[],["coil",4]],[[]],[[],["u16",15]],[[["coil",4]],["bool",15]],[[["formatter",3]],["result",6]],[[["bool",15]],["coil",4]],[[]],[[["str",15]],[["result",6,["coil"]],["coil",4]]],[[["u16",15]],[["result",6,["coil"]],["coil",4]]],[[]],[[],["coil",4]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[],[["error",8],["option",4,["error"]]]],[[],["str",15]],[[["exceptioncode",4]],["bool",15]],[[["reason",4]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[["exceptioncode",4]],["error",4]],[[["error",3]],["error",4]],[[["i64",15]],["option",4]],[[["u64",15]],["option",4]],[[]],[[]],[[]],[[["reason",4]],["bool",15]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[["i64",15]],["option",4]],[[["u64",15]],["option",4]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[["u16",15]],[["result",6,["vec"]],["vec",3,["coil"]]]],[[["u16",15]],[["result",6,["vec"]],["vec",3,["coil"]]]],[[["u16",15]],[["vec",3,["u16"]],["result",6,["vec"]]]],[[["u16",15]],[["vec",3,["u16"]],["result",6,["vec"]]]],[[["u16",15]],["result",6]],[[["u16",15]],["result",6]],[[["u16",15],["coil",4]],["result",6]],[[["u16",15]],["result",6]],[[],[["vec",3,["u8"]],["u8",15]]],[[],[["vec",3,["u16"]],["result",6,["vec"]]]],[[["u16",15]],[["coil",4],["vec",3,["coil"]]]],[[],[["vec",3,["u8"]],["u8",15]]]],"p":[[4,"Coil"],[4,"ExceptionCode"],[4,"Reason"],[4,"Error"],[13,"Exception"],[13,"Io"],[13,"InvalidData"],[13,"Custom"],[4,"ModbusFunction"],[4,"FunctionCode"],[13,"ReadCoils"],[13,"ReadDiscreteInputs"],[13,"ReadHoldingRegisters"],[13,"ReadInputRegisters"],[13,"WriteSingleCoil"],[13,"WriteSingleRegister"],[13,"WriteMultipleCoils"],[13,"WriteMultipleRegisters"],[8,"Requests"]]},\
"modbus_server":{"doc":"","t":[17,3,11,11,11,11,11,11,11,11,11,11,11,5,0,12,0,11,11,0,12,12,12,12,11,11,11,11,11,3,17,17,11,11,11,11,11,11,11,12,11,11,11,12,12,11,11,11,12,11,11,13,13,13,13,4,3,11,11,11,11,12,11,12,11,11,12,11,11,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,17,5,5,5,5],"n":["MODBUS_TCP_DEFAULT_PORT","ModbusConfig","borrow","borrow_mut","clone","clone_any","clone_any_send","clone_any_send_sync","clone_any_sync","clone_into","default","from","into","main","mbap","modbus_uid","server_status","set_port","set_uid","tcp","tcp_connect_timeout","tcp_port","tcp_read_timeout","tcp_write_timeout","to_owned","try_from","try_into","type_id","vzip","Header","MODBUS_HEADER_SIZE","MODBUS_PROTOCOL_TCP","_get_data","borrow","borrow_mut","eq","fmt","from","into","len","ne","new","pack","pid","tid","try_from","try_into","type_id","uid","unpack","vzip","Coil","DiscreteInput","HoldingRegister","InputRegister","ModbusRegisterType","StatusInfo","borrow","borrow","borrow_mut","borrow_mut","capacity","check_range","coils","convert_addr_to_index","create","discrete_inputs","from","from","holding_registers","input_registers","into","into","read_coils","read_discrete_inputs","read_holding_registers","read_input_registers","try_from","try_from","try_into","try_into","type_id","type_id","vzip","vzip","write_multiple_coils","write_multiple_registers","write_single_coil","write_single_register","MODBUS_MAX_PACKET_SIZE","handle_client","handle_pdu_data","handle_status_error","write_response"],"q":["modbus_server","","","","","","","","","","","","","","","","","","","","","","","","","","","","","modbus_server::mbap","","","","","","","","","","","","","","","","","","","","","","modbus_server::server_status","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","modbus_server::tcp","","","",""],"d":["","Config structure for more control over the tcp socket …","","","","","","","","","","","","","","The modbus Unit Identifier used in the modbus layer …","","","","","Connection timeout for TCP socket (Default: <code>OS Default</code>)","The TCP port to use for communication (Default: <code>502</code>)","Timeout when reading from the TCP socket (Default: …","Timeout when writing to the TCP socket (Default: <code>infinite</code>)","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Read <code>count</code> bits starting at address <code>addr</code>.","Read <code>count</code> input bits starting at address <code>addr</code>.","Read <code>count</code> 16bit registers starting at address <code>addr</code>.","Read <code>count</code> 16bit input registers starting at address <code>addr</code>.","","","","","","","","","Write a multiple coils (bits) starting at address <code>addr</code>.","Write a multiple 16bit registers starting at address <code>addr</code>.","Write a single coil (bit) to address <code>addr</code>.","Write a single 16bit register to address <code>addr</code>.","","","","",""],"i":[0,0,1,1,1,1,1,1,1,1,1,1,1,0,0,1,0,1,1,0,1,1,1,1,1,1,1,1,1,0,0,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,3,3,3,3,0,0,3,4,3,4,4,4,4,4,4,4,3,4,4,4,3,4,4,4,4,4,3,4,3,4,3,4,3,4,4,4,4,4,0,0,0,0,0],"f":[null,null,[[]],[[]],[[],["modbusconfig",3]],[[],[["cloneany",8],["global",3],["box",3,["cloneany","global"]]]],[[],[["box",3,["cloneany","global"]],["cloneany",8],["global",3]]],[[],[["global",3],["box",3,["cloneany","global"]],["cloneany",8]]],[[],[["global",3],["box",3,["cloneany","global"]],["cloneany",8]]],[[]],[[],["modbusconfig",3]],[[]],[[]],[[]],null,null,null,[[["u16",15]]],[[["u8",15]]],null,null,null,null,null,[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],null,null,null,[[["usize",15]],[["vec",3,["u8"]],["result",6,["vec"]]]],[[]],[[]],[[["header",3]],["bool",15]],[[["formatter",3]],["result",6]],[[]],[[]],null,[[["header",3]],["bool",15]],[[["u16",15],["u8",15]],["header",3]],[[],[["vec",3,["u8"]],["result",6,["vec"]]]],null,null,[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,[[],[["header",3],["result",6,["header"]]]],[[]],null,null,null,null,null,null,[[]],[[]],[[]],[[]],null,[[["u16",15],["modbusregistertype",4]],["result",6]],null,[[["u16",15],["modbusregistertype",4]],[["u16",15],["result",6,["u16"]]]],[[["usize",15]],["statusinfo",3]],null,[[]],[[]],null,null,[[]],[[]],[[["u16",15]],[["result",6,["vec"]],["vec",3,["coil"]]]],[[["u16",15]],[["result",6,["vec"]],["vec",3,["coil"]]]],[[["u16",15]],[["result",6,["vec"]],["vec",3,["u16"]]]],[[["u16",15]],[["result",6,["vec"]],["vec",3,["u16"]]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[]],[[]],[[["u16",15]],["result",6]],[[["u16",15]],["result",6]],[[["u16",15],["coil",4]],["result",6]],[[["u16",15]],["result",6]],null,[[["u8",15],["arc",3,["mutex"]],["tcpstream",3],["u16",15],["mutex",3,["statusinfo"]],["socketaddr",4]]],[[["header",3],["tcpstream",3],["statusinfo",3]]],[[["error",4],["u8",15]]],[[["header",3],["tcpstream",3]]]],"p":[[3,"ModbusConfig"],[3,"Header"],[4,"ModbusRegisterType"],[3,"StatusInfo"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};