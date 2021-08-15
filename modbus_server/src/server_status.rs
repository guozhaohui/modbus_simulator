extern crate rand;
extern crate modbus_protocol;
use rand::Rng;
use modbus_protocol::exception_code::{Result, Error, ExceptionCode};
use modbus_protocol::coils::Coil;
use modbus_protocol::requests::Requests;

enum ModbusRegisterType{
    Coil = 0x01,
    DiscreteInput = 0x02,
    InputRegister = 0x04,
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
        let mut coils = vec![Coil::Off; size];
        let discrete_inputs : Vec<Coil> = (0..size).map(|_| Coil::from(rand::thread_rng().gen_bool(0.5))).collect();
        let input_registers: Vec<u16> = (0..size).map(|_| rand::thread_rng().gen_range(0..100)).collect();
        let holding_registers = vec![0u16; size];
        for i in 0..size {
            coils[i] = Coil::from(rand::thread_rng().gen_bool(0.5));
        }
        StatusInfo{ capacity: size as u16,
                    coils: coils, discrete_inputs: discrete_inputs,
                    input_registers: input_registers, holding_registers: holding_registers}
    }

    fn convert_addr_to_index(self: &Self, register_type: ModbusRegisterType, addr: u16) -> Result<u16> {
        match register_type {
            ModbusRegisterType::Coil => {
                if addr > self.capacity {
                    return Err(Error::Exception(ExceptionCode::IllegalDataAddress));
                }
                return Ok(addr - (0 * self.capacity));
            }
            ModbusRegisterType::DiscreteInput => {
                if addr < self.capacity || addr > 2 * self.capacity {
                    return Err(Error::Exception(ExceptionCode::IllegalDataAddress));
                }
                return Ok(addr - (1 * self.capacity));
            }
            ModbusRegisterType::InputRegister => {
                if addr < 2 * self.capacity || addr > 3 * self.capacity {
                    return Err(Error::Exception(ExceptionCode::IllegalDataAddress));
                }
                return Ok(addr - (2 * self.capacity));
            }
            ModbusRegisterType::HoldingRegister => {
                if addr < 3 * self.capacity {
                    return Err(Error::Exception(ExceptionCode::IllegalDataAddress));
                }
                return Ok(addr - (3 * self.capacity));
            }
        }
    }
    fn check_range(self: &Self, register_type: ModbusRegisterType, addr: u16, count: u16) -> Result<()> {
        match register_type {
            ModbusRegisterType::Coil => {
                if  (addr + count + 1) as usize > self.coils.len() {
                    return Err(Error::Exception(ExceptionCode::IllegalDataAddress));
                }
                return Ok(());
            }
            ModbusRegisterType::DiscreteInput => {
                if  (addr + count + 1) as usize > self.discrete_inputs.len() {
                    return Err(Error::Exception(ExceptionCode::IllegalDataAddress));
                }
                return Ok(());
            }
            ModbusRegisterType::InputRegister => {
                if  (addr + count + 1) as usize > self.input_registers.len() {
                    return Err(Error::Exception(ExceptionCode::IllegalDataAddress));
                }
                return Ok(());
            }
            ModbusRegisterType::HoldingRegister => {
                if  (addr + count + 1) as usize > self.holding_registers.len() {
                    return Err(Error::Exception(ExceptionCode::IllegalDataAddress));
                }
                return Ok(());
            }
        }
    }
}

impl Requests for StatusInfo {
    /// Read `count` bits starting at address `addr`.
    fn read_coils(self: &mut Self, addr: u16, count: u16) -> Result<Vec<Coil>> {
        let address = self.convert_addr_to_index(ModbusRegisterType::Coil, addr)?;
        self.check_range(ModbusRegisterType::Coil, address, count)?;

        let mut coils: Vec<Coil> = vec![Coil::Off; count as usize];
        coils.clone_from_slice(&self.coils[(address) as usize..(address+count) as usize]);
        Ok(coils)
    }

    /// Read `count` input bits starting at address `addr`.
    fn read_discrete_inputs(self: &mut Self, addr: u16, count: u16) -> Result<Vec<Coil>> {
        let address = self.convert_addr_to_index(ModbusRegisterType::DiscreteInput, addr)?;
        self.check_range(ModbusRegisterType::DiscreteInput, address, count)?;

        let mut coils: Vec<Coil> = vec![Coil::Off; count as usize];
        coils.clone_from_slice(&self.discrete_inputs[address as usize..(address+count) as usize]);
        Ok(coils)
    }

    /// Read `count` 16bit registers starting at address `addr`.
    fn read_holding_registers(self: &mut Self, addr: u16, count: u16) -> Result<Vec<u16>> {
        let address = self.convert_addr_to_index(ModbusRegisterType::HoldingRegister, addr)?;
        self.check_range(ModbusRegisterType::HoldingRegister, address, count)?;

        let mut registers: Vec<u16> = vec![0u16; count as usize];
        registers.clone_from_slice(&self.holding_registers[address as usize..(address+count) as usize]);
        Ok(registers)
    }

    /// Read `count` 16bit input registers starting at address `addr`.
    fn read_input_registers(self: &mut Self, addr: u16, count: u16) -> Result<Vec<u16>> {
        let address = self.convert_addr_to_index(ModbusRegisterType::InputRegister, addr)?;
        self.check_range(ModbusRegisterType::InputRegister, address, count)?;

        let mut registers: Vec<u16> = vec![0u16; count as usize];
        registers.clone_from_slice(&self.input_registers[address as usize..(address+count) as usize]);
        Ok(registers)
    }

    /// Write a single coil (bit) to address `addr`.
    fn write_single_coil(self: &mut Self, addr: u16, value: Coil) -> Result<()> {
        let address = self.convert_addr_to_index(ModbusRegisterType::Coil, addr)?;
        self.check_range(ModbusRegisterType::Coil, address, 1)?;

        self.coils[address as usize] = value;
        return Ok(())
    }

    /// Write a single 16bit register to address `addr`.
    fn write_single_register(self: &mut Self, addr: u16, value: u16) -> Result<()> {
        let address = self.convert_addr_to_index(ModbusRegisterType::HoldingRegister, addr)?;
        self.check_range(ModbusRegisterType::HoldingRegister, address, 1)?;

        self.holding_registers[address as usize] = value;
        return Ok(())
    }

    /// Write a multiple coils (bits) starting at address `addr`.
    fn write_multiple_coils(self: &mut Self, addr: u16, values: &[Coil]) -> Result<()> {
        let address = self.convert_addr_to_index(ModbusRegisterType::Coil, addr)?;
        let n = values.len();
        self.check_range(ModbusRegisterType::Coil, address, n as u16)?;

        for i in 0..n {
            self.coils[i + address as usize] = values[i];
        }
        return Ok(())
    }

    /// Write a multiple 16bit registers starting at address `addr`.
    fn write_multiple_registers(self: &mut Self, addr: u16, values: &[u16]) -> Result<()> {
        let address = self.convert_addr_to_index(ModbusRegisterType::HoldingRegister, addr)?;
        let n = values.len();
        self.check_range(ModbusRegisterType::HoldingRegister, address, n as u16)?;

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
            Ok(_coils) => {
                // assert_eq!(coils, [Coil::Off, Coil::Off]);
                assert!(true);
            },
            Err(_e) => {
                assert!(false);
            },
        }
        match status_info.read_discrete_inputs(17u16, 2u16) {
            Ok(_coils) => {
                // assert_eq!(coils, [Coil::Off, Coil::Off]);
                assert!(true);
            },
            Err(_e) => {
                assert!(false);
            },
        }
        match status_info.read_input_registers(26u16, 3u16) {
            Ok(_registers) => {
               // assert_eq!(registers, [0u16, 0u16, 0u16]);
               assert!(true);
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
