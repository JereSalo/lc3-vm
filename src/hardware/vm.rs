use crate::hardware::{memory::Memory, registers::*, vm_error::VmError};
use crate::instructions::*;
use byteorder::{BigEndian, ReadBytesExt};
use std::os::macos::raw;
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

    pub fn load_arguments(&mut self) -> Result<(), VmError> {
        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            return Err(VmError::InvalidArguments);
        }

        // Iterate over each argument (skipping the first one which is the program name)
        for arg in &args[1..] {
            self.read_image_file(arg)?;
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), VmError> {
        self.load_arguments()?;
        // Disable input buffering and store the original terminal settings
        let original_termios = disable_input_buffering()?;

        self.reg.pc = PC_START;
        while self.running {
            // Fetch instruction from memory
            let instruction_address = self.reg.pc;
            let instruction = self.mem.read(instruction_address);

            // Increment PC
            self.reg.pc += 1;

            // Decode opcode
            let raw_opcode = instruction >> 12;
            self.execute_instruction(Opcode::try_from(raw_opcode)?, instruction)?;
        }

        // Restore input buffering
        restore_input_buffering(original_termios)?;
        Ok(())
    }

    fn execute_instruction(&mut self, opcode: Opcode, instr: u16) -> Result<(), VmError> {
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
            Opcode::OpRes => {return Err(VmError::BadOpcode)}
            Opcode::OpRti => {return Err(VmError::BadOpcode)} // The last 2 are unused opcodes, I have to define what to do when they are called.
        }
        Ok(())
    }

    fn read_image_file(&mut self, file_path: &str) -> Result<(), VmError> {
        // Attempt to open the file and map any I/O error to a `VmError::ReadImage`
        let file = File::open(file_path).map_err(|e| VmError::ReadImage(format!("Failed to open file '{}': {}", file_path, e)))?;
        let mut reader = BufReader::new(file);
    
        // Read the initial address from the file and map any I/O error to a `VmError::ReadImage`
        let mut address = reader.read_u16::<BigEndian>()
            .map_err(|e| VmError::ReadImage(format!("Failed to read initial address from '{}': {}", file_path, e)))?;
        
        // Read instructions from the file and write them to memory until EOF or an error occurs
        while let Ok(instr) = reader.read_u16::<BigEndian>() {
            self.mem.write(address, instr);
            address += 1;
        }
    
        Ok(()) // Return Ok(()) if no errors occurred
    }
}

// Helper function to disable input buffering and return the original terminal settings
fn disable_input_buffering() -> Result<Termios, VmError> {
    let stdin_fd = 0; // File descriptor for stdin
    let mut termios = Termios::from_fd(stdin_fd).map_err(|e| VmError::Io(e))?;
    let original_termios = termios.clone();
    termios.c_lflag &= !(ICANON | ECHO); // Disable canonical mode and echo
    tcsetattr(stdin_fd, TCSANOW, &termios).map_err(|e| VmError::Io(e))?;
    Ok(original_termios)
}

fn restore_input_buffering(original_termios: Termios) -> Result<(), VmError> {
    tcsetattr(0, TCSANOW, &original_termios).map_err(|e| VmError::Io(e))?;
    Ok(())
}
