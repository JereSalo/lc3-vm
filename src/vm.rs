use crate::{memory::Memory, registers::Registers, opcode::Opcode};

const PC_START: u16 = 0x3000;

pub struct VM {
    registers: Registers,
    memory: Memory,
}

impl VM {
    pub fn new() -> Self {
        VM {
            registers: Registers::new(),
            memory: Memory::new(),
        }
    }

    pub fn run(&mut self) {
        self.registers.pc = PC_START;

        loop {
            // Fetch instruction from memory
            let instruction_address: usize = self.registers.pc.into();
            let instruction = self.memory.read(instruction_address);

            // Increment PC
            self.registers.pc += 1;

            // Decode opcode
            let raw_opcode = instruction >> 12;
            match Opcode::try_from(raw_opcode) {
                Ok(opcode) => self.execute_instruction(opcode),
                Err(_) => {
                    eprintln!("Unknown opcode: {:#X}", raw_opcode);
                    break;
                }
            }
        }
    }

    fn execute_instruction(&mut self, opcode: Opcode) {
        match opcode {
            Opcode::OpAdd => self.op_add(),
            Opcode::OpAnd => self.op_and(),
            Opcode::OpNot => self.op_not(),
            Opcode::OpBr => self.op_br(),
            Opcode::OpJmp => self.op_jmp(),
            Opcode::OpJsr => self.op_jsr(),
            Opcode::OpLd => self.op_ld(),
            Opcode::OpLdi => self.op_ldi(),
            Opcode::OpLdr => self.op_ldr(),
            Opcode::OpLea => self.op_lea(),
            Opcode::OpSt => self.op_st(),
            Opcode::OpSti => self.op_sti(),
            Opcode::OpStr => self.op_str(),
            Opcode::OpTrap => self.op_trap(),
            Opcode::OpRes => self.op_res(),
            Opcode::OpRti => self.op_rti(),
        }
    }
}
