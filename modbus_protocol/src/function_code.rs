use num_derive::FromPrimitive;
use super::coils::Coil;

pub type Address = u16;
pub type Quantity = u16;
pub type Value = u16;
pub type Values= Vec<u16> ;
pub type Coils= Vec<Coil> ;

#[derive(FromPrimitive)]
pub enum FunctionCode{
    ReadCoils = 0x01,
    ReadDiscreteInputs = 0x02,
    ReadHoldingRegisters = 0x03,
    ReadInputRegisters = 0x04,
    WriteSingleCoil = 0x05,
    WriteSingleRegister = 0x06,
    WriteMultipleCoils = 0x0f,
    WriteMultipleRegisters = 0x10,
}

pub enum ModbusFunction<'a> {
    ReadCoils(Address, Quantity),
    ReadDiscreteInputs(Address, Quantity),
    ReadHoldingRegisters(Address, Quantity),
    ReadInputRegisters(Address, Quantity),
    WriteSingleCoil(Address, Coil),
    WriteSingleRegister(Address, Value),
    WriteMultipleCoils(Address, &'a Coils),
    WriteMultipleRegisters(Address, &'a Values),
}

