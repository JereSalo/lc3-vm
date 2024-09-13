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

    fn call_instruction(&self, opcode: u16) {
        match opcode {
            0  => self.op_add(),   // OpAdd
            1  => self.op_and(),   // OpAnd
            2  => self.op_not(),   // OpNot
            3  => self.op_br(),    // OpBr
            4  => self.op_jmp(),   // OpJmp
            5  => self.op_jsr(),   // OpJsr
            6  => self.op_ld(),    // OpLd
            7  => self.op_ldi(),   // OpLdi
            8  => self.op_ldr(),   // OpLdr (new addition)
            9  => self.op_lea(),   // OpLea
            10 => self.op_st(),    // OpSt
            11 => self.op_sti(),   // OpSti
            12 => self.op_str(),   // OpStr
            13 => self.op_trap(),  // OpTrap
            14 => self.op_res(),   // OpRes
            15 => self.op_rti(),   // OpRti
            _  => {}               // Fallback for invalid opcodes
        }
    }
}
