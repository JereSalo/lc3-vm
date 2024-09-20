use crate::hardware::{memory::Memory, registers::*};
use crate::instructions::*;
use byteorder::{BigEndian, ReadBytesExt};
use std::{env, fs::File, io::{BufReader}};
use termios::{tcsetattr, Termios, TCSANOW, ECHO, ICANON};

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

    pub fn load_arguments(&mut self) {
        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            eprintln!("cargo run [image-file1] ...");
            return;
        }

        // Iterate over each argument (skipping the first one which is the program name)
        for arg in &args[1..] {
            self.read_image_file(arg);
        }
    }

    pub fn run(&mut self) {
        // Disable input buffering and store the original terminal settings
        let original_termios = disable_input_buffering();

        self.reg.pc = PC_START;
        while self.running {
            // Fetch instruction from memory
            let instruction_address = self.reg.pc;
            let instruction = self.mem.read(instruction_address);

            // Increment PC
            self.reg.pc += 1;

            // Decode opcode
            let raw_opcode = instruction >> 12;
            match Opcode::try_from(raw_opcode) {
                Ok(opcode) => self.execute_instruction(opcode, instruction),
                Err(_) => {
                    eprintln!("Unknown opcode: {:#X}", raw_opcode);
                    break; // It should finish execution if there's a bad opcode.
                }
            }
        }

        // Restore input buffering
        restore_input_buffering(original_termios);
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
            Opcode::OpRti => {} // The last 2 are unused opcodes, I have to define what to do when they are called.
        }
    }

    fn read_image_file(&mut self, file_path: &str) {
        let file = File::open(file_path).unwrap();
        let mut reader = BufReader::new(file);

        let mut address = reader.read_u16::<BigEndian>().unwrap();
        while let Ok(instr) = reader.read_u16::<BigEndian>() {
            self.mem.write(address, instr);
            address += 1;
        }
    }
}

// Helper function to disable input buffering and return the original terminal settings
fn disable_input_buffering() -> Termios {
    let stdin_fd = 0; // File descriptor for stdin
    let mut termios = Termios::from_fd(stdin_fd).unwrap();
    let original_termios = termios.clone();
    termios.c_lflag &= !(ICANON | ECHO); // Disable canonical mode and echo
    tcsetattr(stdin_fd, TCSANOW, &termios).unwrap();
    original_termios
}

fn restore_input_buffering(original_termios: Termios) {
    tcsetattr(0, TCSANOW, &original_termios).unwrap();
}
