use crate::hardware::{memory::Memory, registers::Registers};
use crate::instructions::*;

const PC_START: u16 = 0x3000;

pub struct VM {
    pub reg: Registers,
    pub mem: Memory,
    pub running: bool,
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

impl VM {
    pub fn new() -> Self {
        VM {
            reg: Registers::new(),
            mem: Memory::new(),
            running: true,
        }
    }

    pub fn read_image_file() {
        // First 16 bits specify where should program start. The Origin

        // The rest of the program is read into memory starting from the origin address.
    }

    pub fn run(&mut self) {
        self.reg.pc = PC_START;
        while self.running {
            // Fetch instruction from memory
            let instruction_address: usize = self.reg.pc.into();
            let instruction = self.mem.read(instruction_address);

            // Increment PC
            self.reg.pc += 1;

            // Decode opcode
            let raw_opcode = instruction >> 12;
            match Opcode::try_from(raw_opcode) {
                Ok(opcode) => self.execute_instruction(opcode, instruction),
                Err(_) => {
                    eprintln!("Unknown opcode: {:#X}", raw_opcode);
                    break;
                }
            }
        }
        // Shutdown goes here.
    }

    fn execute_instruction(&mut self, opcode: Opcode, instr: u16) {
        match opcode {
            Opcode::OpAdd => self.op_add(instr),
            Opcode::OpAnd => self.op_and(instr),
            Opcode::OpNot => self.op_not(instr),
            Opcode::OpBr => self.op_br(instr),
            Opcode::OpJmp => self.op_jmp(instr),
            Opcode::OpJsr => self.op_jsr(instr),
            Opcode::OpLd => self.op_ld(instr),
            Opcode::OpLdi => self.op_ldi(instr),
            Opcode::OpLdr => self.op_ldr(instr),
            Opcode::OpLea => self.op_lea(instr),
            Opcode::OpSt => self.op_st(instr),
            Opcode::OpSti => self.op_sti(instr),
            Opcode::OpStr => self.op_str(instr),
            Opcode::OpTrap => self.op_trap(instr),
            Opcode::OpRes => {}
            Opcode::OpRti => {}
            // The last 2 are unused opcodes, I have to define what to do when they are called.
        }
    }
}
