use crate::{memory::Memory, registers::Registers};

pub struct VM {
    registers: Registers,
    memory: Memory,
}

impl VM {
    pub fn new() -> Self {
        VM { registers: Registers::new(), memory: Memory::new() }
    }

    pub fn run(&self) {
        
    }
}
