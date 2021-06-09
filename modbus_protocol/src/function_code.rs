pub type Address = u16;
pub type Quantity = u16;
pub type Value = u16;

pub enum Function<'a> {
    ReadCoils(Address, Quantity),
    ReadDiscreteInputs(Address, Quantity),
    ReadHoldingRegisters(Address, Quantity),
    ReadInputRegisters(Address, Quantity),
    WriteSingleCoil(Address, Value),
    WriteSingleRegister(Address, Value),
    WriteMultipleCoils(Address, Quantity, &'a [u8]),
    WriteMultipleRegisters(Address, Quantity, &'a [u8]),
}

impl<'a> Function<'a> {
    pub fn code(&self) -> u8 {
        match *self {
            Function::ReadCoils(_, _) => 0x01,
            Function::ReadDiscreteInputs(_, _) => 0x02,
            Function::ReadHoldingRegisters(_, _) => 0x03,
            Function::ReadInputRegisters(_, _) => 0x04,
            Function::WriteSingleCoil(_, _) => 0x05,
            Function::WriteSingleRegister(_, _) => 0x06,
            Function::WriteMultipleCoils(_, _, _) => 0x0f,
            Function::WriteMultipleRegisters(_, _, _) => 0x10,
        }
        // ReadExceptionStatus     = 0x07,
        // ReportSlaveId           = 0x11,
        // MaskWriteRegister       = 0x16,
        // WriteAndReadRegisters   = 0x17
    }
}


