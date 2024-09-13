use crate::{memory::Memory, registers::Registers};

pub struct VM {
    registers: Registers,
    memory: Memory,
}
