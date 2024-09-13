use crate::{memory::Memory, registers::Registers};

const PC_START: u16 = 0x3000;

pub struct VM {
    registers: Registers,
    memory: Memory,
}

impl VM {
    pub fn new() -> Self {
        VM { registers: Registers::new(), memory: Memory::new() }
    }

    pub fn run(&mut self) {
        self.registers.pc = PC_START;

        loop {
            // Memory has instructions. PC points to memory

            let instruction_address: usize = self.registers.pc.into();

            // Load instruction
            let instr = self.memory.read(instruction_address);

            // Increment PC
            self.registers.pc += 1;

            let opcode = instr >> 12;

            



        }
    }

    fn call_instruction(&self, opcode: u16){
        match opcode {
            // match with enum and call method
        }
    }
}
