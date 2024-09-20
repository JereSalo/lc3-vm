use std::io::Error;

pub enum VmError {
    Io(Error),
    InvalidOpcode,
    BadOpcode,
    InvalidTrapCode,
}
