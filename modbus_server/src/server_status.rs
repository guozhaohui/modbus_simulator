extern crate modbus_protocol;
use modbus_protocol::exception_code::{Result, Error, Reason};
use modbus_protocol::coils::Coil;
use modbus_protocol::requests::Requests;

pub struct StatusInfo {
    registers: Vec<u16>,
    coils : Vec<Coil>,
}

impl StatusInfo {
    pub fn create(size: usize) -> StatusInfo {
        let coils = vec![Coil::Off; size];
        let registers = vec![0u16; size];
        StatusInfo{coils: coils, registers: registers}
    }
}

impl Requests for StatusInfo {
    /// Read `count` bits starting at address `addr`.
    fn read_coils(self: &mut Self, addr: u16, count: u16) -> Result<Vec<Coil>> {
        if  (addr + count + 1) as usize > self.coils.len() {
            return Err(Error::InvalidData(Reason::InvalidRequestParameter));
        }
        let mut coils: Vec<Coil> = vec![Coil::Off; count as usize];
        coils.clone_from_slice(&self.coils[addr as usize..(addr+count) as usize]);
        Ok(coils)
    }

    /// Read `count` input bits starting at address `addr`.
    fn read_discrete_inputs(self: &mut Self, addr: u16, count: u16) -> Result<Vec<Coil>> {
        if  (addr + count + 1) as usize > self.coils.len() {
            return Err(Error::InvalidData(Reason::InvalidRequestParameter));
        }
        let mut coils: Vec<Coil> = vec![Coil::Off; count as usize];
        coils.clone_from_slice(&self.coils[addr as usize..(addr+count) as usize]);
        Ok(coils)
    }

    /// Read `count` 16bit registers starting at address `addr`.
    fn read_holding_registers(self: &mut Self, addr: u16, count: u16) -> Result<Vec<u16>> {
        if  (addr + count + 1) as usize > self.registers.len() {
            return Err(Error::InvalidData(Reason::InvalidRequestParameter));
        }
        let mut registers: Vec<u16> = vec![0u16; count as usize];
        registers.clone_from_slice(&self.registers[addr as usize..(addr+count) as usize]);
        Ok(registers)
    }

    /// Read `count` 16bit input registers starting at address `addr`.
    fn read_input_registers(self: &mut Self, addr: u16, count: u16) -> Result<Vec<u16>> {
        if  (addr + count + 1) as usize > self.registers.len() {
            return Err(Error::InvalidData(Reason::InvalidRequestParameter));
        }
        let mut registers: Vec<u16> = vec![0u16; count as usize];
        registers.clone_from_slice(&self.registers[addr as usize..(addr+count) as usize]);
        Ok(registers)
    }

    /// Write a single coil (bit) to address `addr`.
    fn write_single_coil(self: &mut Self, addr: u16, value: Coil) -> Result<()> {
        if  (addr + 1) as usize > self.coils.len() {
            return Err(Error::InvalidData(Reason::InvalidRequestParameter));
        }
        self.coils[addr as usize] = value;
        return Ok(())
    }

    /// Write a single 16bit register to address `addr`.
    fn write_single_register(self: &mut Self, addr: u16, value: u16) -> Result<()> {
        if  (addr + 1) as usize > self.registers.len() {
            return Err(Error::InvalidData(Reason::InvalidRequestParameter));
        }
        self.registers[addr as usize] = value;
        return Ok(())
    }

    /// Write a multiple coils (bits) starting at address `addr`.
    fn write_multiple_coils(self: &mut Self, addr: u16, values: &[Coil]) -> Result<()> {
        let n = values.len();
        if  (addr + 1) as usize + n > self.coils.len() {
            return Err(Error::InvalidData(Reason::InvalidRequestParameter));
        }
        for i in 0..n {
            self.coils[i + addr as usize] = values[i];
        }
        return Ok(())
    }

    /// Write a multiple 16bit registers starting at address `addr`.
    fn write_multiple_registers(self: &mut Self, addr: u16, values: &[u16]) -> Result<()> {
        let n = values.len();
        if  (addr + 1) as usize + n > self.registers.len() {
            return Err(Error::InvalidData(Reason::InvalidRequestParameter));
        }
        for i in 0..n {
            self.registers[i + addr as usize] = values[i];
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
        match status_info.read_holding_registers(6u16, 3u16) {
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
        match status_info.read_holding_registers(7u16, 3u16) {
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
        let coils = vec![Coil::On, Coil::On, Coil::On];
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
        match status_info.write_multiple_registers(6u16, &regs) {
            Ok(()) => {
                assert!(true);
            },
            Err(_e) => {
                assert!(false);
            },
        }
        match status_info.read_holding_registers(6u16, 3u16) {
            Ok(registers) => {
                assert_eq!(registers, regs);
            },
            Err(_e) => {
                assert!(true);
            },
        }
    }
}
