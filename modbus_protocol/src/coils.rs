/// Single bit status values, used in read or write coil functions
use super::exception_code::Error;
use super::exception_code::Result;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Coil {
    On,
    Off,
}

impl Coil {
    pub fn code(self) -> u16 {
        match self {
            Coil::On => 0xff00,
            Coil::Off => 0x0000,
        }
    }
}

impl FromStr for Coil {
    type Err = Error;
    fn from_str(s: &str) -> Result<Coil> {
        if s == "On" {
            Ok(Coil::On)
        } else if s == "Off" {
            Ok(Coil::Off)
        } else {
            Err(Error::ParseCoilError)
        }
    }
}

impl From<bool> for Coil {
    fn from(b: bool) -> Coil {
        if b {
            Coil::On
        } else {
            Coil::Off
        }
    }
}

impl std::ops::Not for Coil {
    type Output = Coil;

    fn not(self) -> Coil {
        match self {
            Coil::On => Coil::Off,
            Coil::Off => Coil::On,
        }
    }
}


