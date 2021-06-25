extern crate modbus_protocol;
use modbus_protocol::exception_code::{Result, Error, Reason};
use modbus_protocol::coils::Coil;
use modbus_protocol::requests::Requests;

enum ModbusRegisterType{
    Coil = 0x01,
    DiscreteInput = 0x02,
    InputRegisters = 0x04,
    HoldingRegister = 0x03,
}

pub struct StatusInfo {
    capacity: u16,
    coils : Vec<Coil>,
    discrete_inputs: Vec<Coil>,
    input_registers: Vec<u16>,
    holding_registers: Vec<u16>,
}

impl StatusInfo {
    pub fn create(size: usize) -> StatusInfo {
        let coils = vec![Coil::Off; size];
        let discrete_inputs = vec![Coil::Off; size];
        let input_registers = vec![0u16; size];
        let holding_registers = vec![0u16; size];
        StatusInfo{ capacity: size as u16,
                    coils: coils, discrete_inputs: discrete_inputs,
                    input_registers: input_registers, holding_registers: holding_registers}
    }

    fn convert_addr_to_index(self: &Self, register_type: ModbusRegisterType, addr: u16) -> u16 {
        match register_type {
            ModbusRegisterType::Coil => return addr - (0 * self.capacity),
            ModbusRegisterType::DiscreteInput => return addr - (1 * self.capacity),
            ModbusRegisterType::InputRegisters => return addr - (2 * self.capacity),
            ModbusRegisterType::HoldingRegister => return addr - (3 * self.capacity),
        }
    }
}

impl Requests for StatusInfo {
    /// Read `count` bits starting at address `addr`.
    fn read_coils(self: &mut Self, addr: u16, count: u16) -> Result<Vec<Coil>> {
        let address = self.convert_addr_to_index(ModbusRegisterType::Coil, addr);
        if  (address + count + 1) as usize > self.coils.len() {
            return Err(Error::InvalidData(Reason::InvalidRequestParameter));
        }
        let mut coils: Vec<Coil> = vec![Coil::Off; count as usize];
        coils.clone_from_slice(&self.coils[(address) as usize..(address+count) as usize]);
        Ok(coils)
    }

    /// Read `count` input bits starting at address `addr`.
    fn read_discrete_inputs(self: &mut Self, addr: u16, count: u16) -> Result<Vec<Coil>> {
        let address = self.convert_addr_to_index(ModbusRegisterType::DiscreteInput, addr);
        if  (address + count + 1) as usize > self.coils.len() {
            return Err(Error::InvalidData(Reason::InvalidRequestParameter));
        }
        let mut coils: Vec<Coil> = vec![Coil::Off; count as usize];
        coils.clone_from_slice(&self.discrete_inputs[address as usize..(address+count) as usize]);
        Ok(coils)
    }

    /// Read `count` 16bit registers starting at address `addr`.
    fn read_holding_registers(self: &mut Self, addr: u16, count: u16) -> Result<Vec<u16>> {
        let address = self.convert_addr_to_index(ModbusRegisterType::HoldingRegister, addr);
        if  (address + count + 1) as usize > self.holding_registers.len() {
            return Err(Error::InvalidData(Reason::InvalidRequestParameter));
        }
        let mut registers: Vec<u16> = vec![0u16; count as usize];
        registers.clone_from_slice(&self.holding_registers[address as usize..(address+count) as usize]);
        Ok(registers)
    }

    /// Read `count` 16bit input registers starting at address `addr`.
    fn read_input_registers(self: &mut Self, addr: u16, count: u16) -> Result<Vec<u16>> {
        let address = self.convert_addr_to_index(ModbusRegisterType::InputRegisters, addr);
        if  (address + count + 1) as usize > self.input_registers.len() {
            return Err(Error::InvalidData(Reason::InvalidRequestParameter));
        }
        let mut registers: Vec<u16> = vec![0u16; count as usize];
        registers.clone_from_slice(&self.input_registers[address as usize..(address+count) as usize]);
        Ok(registers)
    }

    /// Write a single coil (bit) to address `addr`.
    fn write_single_coil(self: &mut Self, addr: u16, value: Coil) -> Result<()> {
        let address = self.convert_addr_to_index(ModbusRegisterType::Coil, addr);
        if  (address + 1) as usize > self.coils.len() {
            return Err(Error::InvalidData(Reason::InvalidRequestParameter));
        }
        self.coils[address as usize] = value;
        return Ok(())
    }

    /// Write a single 16bit register to address `addr`.
    fn write_single_register(self: &mut Self, addr: u16, value: u16) -> Result<()> {
        let address = self.convert_addr_to_index(ModbusRegisterType::HoldingRegister, addr);
        if  (address + 1) as usize > self.holding_registers.len() {
            return Err(Error::InvalidData(Reason::InvalidRequestParameter));
        }
        self.holding_registers[address as usize] = value;
        return Ok(())
    }

    /// Write a multiple coils (bits) starting at address `addr`.
    fn write_multiple_coils(self: &mut Self, addr: u16, values: &[Coil]) -> Result<()> {
        let address = self.convert_addr_to_index(ModbusRegisterType::Coil, addr);
        let n = values.len();
        if  (address + 1) as usize + n > self.coils.len() {
            return Err(Error::InvalidData(Reason::InvalidRequestParameter));
        }
        for i in 0..n {
            self.coils[i + address as usize] = values[i];
        }
        return Ok(())
    }

    /// Write a multiple 16bit registers starting at address `addr`.
    fn write_multiple_registers(self: &mut Self, addr: u16, values: &[u16]) -> Result<()> {
        let address = self.convert_addr_to_index(ModbusRegisterType::HoldingRegister, addr);
        let n = values.len();
        if  (address + 1) as usize + n > self.holding_registers.len() {
            return Err(Error::InvalidData(Reason::InvalidRequestParameter));
        }
        for i in 0..n {
            self.holding_registers[i + address as usize] = values[i];
        }
        return Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_server_initializaion(){
        let mut status_info = StatusInfo::create(10usize);
        match status_info.read_coils(7u16, 2u16) {
            Ok(coils) => {
                assert_eq!(coils, [Coil::Off, Coil::Off]);
            },
            Err(_e) => {
                assert!(false);
            },
        }
        match status_info.read_holding_registers(36u16, 3u16) {
            Ok(registers) => {
                assert_eq!(registers, [0u16, 0u16, 0u16]);
            },
            Err(_e) => {
                assert!(false);
            },
        }
    }
    #[test]
    fn test_server_invalid_param1(){
        let mut status_info = StatusInfo::create(10usize);
        match status_info.read_coils(7u16, 3u16) {
            Ok(_coils) => {
                assert!(false);
            },
            Err(_e) => {
                assert!(true);
            },
        }
        match status_info.read_holding_registers(37u16, 3u16) {
            Ok(_registers) => {
                assert!(false);
            },
            Err(_e) => {
                assert!(true);
            },
        }
    }
    #[test]
    fn test_server_writeread(){
        let mut status_info = StatusInfo::create(10usize);
        // test coils write/read
        let coils = vec![Coil::On, Coil::Off, Coil::On];
        match status_info.write_multiple_coils(6u16, &coils) {
            Ok(()) => {
                assert!(true);
            },
            Err(_e) => {
                assert!(false);
            },
        }
        match status_info.read_coils(6u16, 3u16) {
            Ok(_coils) => {
                assert_eq!(_coils, coils);
            },
            Err(_e) => {
                assert!(false);
            },
        }
        // test registers write/read
        let regs = vec![1u16, 2u16, 3u16];
        match status_info.write_multiple_registers(36u16, &regs) {
            Ok(()) => {
                assert!(true);
            },
            Err(_e) => {
                assert!(false);
            },
        }
        match status_info.read_holding_registers(36u16, 3u16) {
            Ok(registers) => {
                assert_eq!(registers, regs);
            },
            Err(_e) => {
                assert!(true);
            },
        }
    }
}
