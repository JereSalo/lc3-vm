use std::fmt::{self};
use std::io::Error;

use crate::instructions::Opcode;

pub enum VmError {
    Io(Error),
    ReadImage(String),
    BadOpcode(Opcode),
    InvalidOpcode(u16),
    InvalidTrapCode(u16),
    InvalidArguments,
    InvalidRegister(u16),
    PcOverflow,
}

// Implementing `Display` for `VmError`
impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmError::Io(err) => write!(f, "I/O Error: {}", err),
            VmError::InvalidOpcode(value) => write!(f, "Invalid Opcode encountered: {}", value),
            VmError::BadOpcode(opcode) => write!(f, "Bad Opcode encountered: {}", opcode),
            VmError::InvalidTrapCode(value) => {
                write!(f, "Invalid Trap Code encountered: {}", value)
            }
            VmError::ReadImage(msg) => write!(f, "Error reading image file: {}", msg),
            VmError::InvalidArguments => {
                write!(f, "Invalid arguments: Use 'cargo run <image_file> ..'")
            }
            VmError::InvalidRegister(reg) => {
                write!(
                    f,
                    "Register out of bounds! Valid registers are 0 to 7. Actual value: {}",
                    reg
                )
            } // This would happen if programmer made a mistake
            VmError::PcOverflow => {
                write!(f, "Overflow of Program Counter has ocurred!")
            }
        }
    }
}

// Implementing `Debug` for `VmError` (optional, but often useful)
impl fmt::Debug for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f) // Reuse `Display` implementation
    }
}

// Implementing `From` to convert `std::io::Error` into `VmError::Io`
impl From<Error> for VmError {
    fn from(err: Error) -> Self {
        VmError::Io(err)
    }
}
