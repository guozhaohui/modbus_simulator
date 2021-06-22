extern crate modbus_protocol;
use modbus_protocol::exception_code::{Result};
use modbus_protocol::coils::Coil;
use modbus_protocol::requests::Requests;

pub struct StatusInfo {
    registers: Vec<u16>,
    coils : Vec<Coil>,
}

impl StatusInfo {
    pub fn create(size: usize) -> StatusInfo {
        let mut status_info = StatusInfo {
                registers: Vec::with_capacity(size),
                coils: Vec::with_capacity(size)
        };
        for i in 0..size {
            status_info.registers[i] = 0;
        }
        for i in 0..size {
            status_info.coils[i] = Coil::Off;
        }
        status_info
    }

}

impl Requests for StatusInfo {
    /// Read `count` bits starting at address `addr`.
    fn read_coils(self: &mut Self, addr: u16, count: u16) -> Result<Vec<Coil>> {
        let mut coils: Vec<Coil> = Vec::with_capacity(count as usize);
        coils.clone_from_slice(&self.coils[addr as usize..(addr+count) as usize]);
        Ok(coils)
    }

    /// Read `count` input bits starting at address `addr`.
    fn read_discrete_inputs(self: &mut Self, addr: u16, count: u16) -> Result<Vec<Coil>> {
        let mut coils: Vec<Coil> = Vec::with_capacity(count as usize);
        coils.clone_from_slice(&self.coils[addr as usize..(addr+count) as usize]);
        Ok(coils)
    }

    /// Read `count` 16bit registers starting at address `addr`.
    fn read_holding_registers(self: &mut Self, addr: u16, count: u16) -> Result<Vec<u16>> {
        let mut registers: Vec<u16> = Vec::with_capacity(count as usize);
        registers.clone_from_slice(&self.registers[addr as usize..(addr+count) as usize]);
        Ok(registers)
    }

    /// Read `count` 16bit input registers starting at address `addr`.
    fn read_input_registers(self: &mut Self, addr: u16, count: u16) -> Result<Vec<u16>> {
        let mut registers: Vec<u16> = Vec::with_capacity(count as usize);
        registers.clone_from_slice(&self.registers[addr as usize..(addr+count) as usize]);
        Ok(registers)
    }

    /// Write a single coil (bit) to address `addr`.
    fn write_single_coil(self: &mut Self, addr: u16, value: Coil) -> Result<()> {
        self.coils[addr as usize] = value;
        return Ok(())
    }

    /// Write a single 16bit register to address `addr`.
    fn write_single_register(self: &mut Self, addr: u16, value: u16) -> Result<()> {
        self.registers[addr as usize] = value;
        return Ok(())
    }

    /// Write a multiple coils (bits) starting at address `addr`.
    fn write_multiple_coils(self: &mut Self, addr: u16, values: &[Coil]) -> Result<()> {
        let n = values.len();
        for i in 0..n-1 {
            self.coils[i + addr as usize] = values[i];
        }
        return Ok(())
    }

    /// Write a multiple 16bit registers starting at address `addr`.
    fn write_multiple_registers(self: &mut Self, addr: u16, values: &[u16]) -> Result<()> {
        let n = values.len();
        for i in 0..n-1 {
            self.registers[i + addr as usize] = values[i];
        }
        return Ok(())
    }
}

