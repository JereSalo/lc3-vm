use std::fmt::{self};
use std::io::Error;

pub enum VmError {
    Io(Error),
    InvalidOpcode,
    BadOpcode,
    InvalidTrapCode,
    ReadImage(String),
    InvalidArguments
}

// Implementing `Display` for `VmError`
impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmError::Io(err) => write!(f, "I/O Error: {}", err),
            VmError::InvalidOpcode => write!(f, "Invalid Opcode encountered"),
            VmError::BadOpcode => write!(f, "Bad Opcode encountered"),
            VmError::InvalidTrapCode => write!(f, "Invalid Trap Code encountered"),
            VmError::ReadImage(msg) => write!(f, "Error reading image file: {}", msg),
            VmError::InvalidArguments => write!(f, "Invalid arguments: Use 'cargo run <image_file> ..'"),
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
