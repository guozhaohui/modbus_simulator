use num_derive::FromPrimitive;

use std::fmt;
use std::io;

#[derive(Debug, PartialEq, FromPrimitive)]
/// Modbus exception codes returned from the server.
pub enum ExceptionCode {
    IllegalFunction         = 0x01,
    IllegalDataAddress      = 0x02,
    IllegalDataValue        = 0x03,
    SlaveOrServerFailure    = 0x04,
    Acknowledge             = 0x05,
    SlaveOrServerBusy       = 0x06,
    NegativeAcknowledge     = 0x07,
    MemoryParity            = 0x08,
    NotDefined              = 0x09,
    GatewayPath             = 0x0a,
    GatewayTarget           = 0x0b
}

/// `InvalidData` reasons
#[derive(Debug, PartialEq)]
pub enum Reason {
    UnexpectedReplySize,
    BytecountNotEven,
    SendBufferEmpty,
    RecvBufferEmpty,
    SendBufferTooBig,
    DecodingError,
    EncodingError,
    InvalidByteorder,
    InvalidRequestParameter,
    Custom(String),
}

/// Combination of Modbus, IO and data corruption errors
#[derive(Debug)]
pub enum Error {
    Exception(ExceptionCode),
    Io(io::Error),
    InvalidResponse,
    InvalidData(Reason),
    InvalidFunction,
    ParseCoilError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match *self {
            Exception(ref code) => write!(f, "modbus exception: {:?}", code),
            Io(ref err) => write!(f, "I/O error: {}", err),
            InvalidResponse => write!(f, "invalid response"),
            InvalidData(ref reason) => write!(f, "invalid data: {:?}", reason),
            InvalidFunction => write!(f, "invalid modbus function"),
            ParseCoilError => write!(f, "parse coil could not be parsed"),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        use Error::*;

        match *self {
            Exception(_) => "modbus exception",
            Io(_) => "I/O error",
            InvalidResponse => "invalid response",
            InvalidData(_) => "invalid data",
            InvalidFunction => "invalid modbus function",
            ParseCoilError => "parse coil could not be parsed",
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            Error::Io(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<ExceptionCode> for Error {
    fn from(err: ExceptionCode) -> Error {
        Error::Exception(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

/// Result type used to nofify success or failure in communication
pub type Result<T> = std::result::Result<T, Error>;


