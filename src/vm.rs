use crate::{memory::Memory, registers::Registers,opcode::Opcode};

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

            self.call_instruction(opcode)

        }
    }

    fn call_instruction(&mut self, opcode: u16) {
        match opcode {
            0  => self.op_add(),
            1  => self.op_and(),
            2  => self.op_not(),
            3  => self.op_br(),
            4  => self.op_jmp(),
            5  => self.op_jsr(),
            6  => self.op_ld(),
            7  => self.op_ldi(),
            8  => self.op_ldr(),
            9  => self.op_lea(),
            10 => self.op_st(),
            11 => self.op_sti(),
            12 => self.op_str(),
            13 => self.op_trap(),
            14 => self.op_res(),
            15 => self.op_rti(),
            _  => {}
        }
    }
}
