use std::io::{self, Read, Write};
use termios::{Termios, TCSANOW, tcsetattr, ECHO, ICANON};

use crate::hardware::vm::VM;

impl VM {
    /// ## Trap
    /// Executes a trap instruction, which handles I/O operations and system calls.
    pub fn op_trap(&mut self, instr: u16) {
        // Save the program counter to general-purpose register 7
        self.reg.update(7, self.reg.pc);

        // Check the lower byte of the instruction and execute the corresponding trap routine
        let raw_trap_code = instr & 0xFF;

        match TrapCode::try_from(raw_trap_code) {
            Ok(trap_code) => {self.execute_trap(trap_code);}
            _ => {} // Handle conversion error...
        }
    }
}

enum TrapCode {
    GETC = 0x20,
    OUT = 0x21,
    PUTS = 0x22,
    IN = 0x23,
    PUTSP = 0x24,
    HALT = 0x25
}

/// Convert u16 into Opcode
impl TryFrom<u16> for TrapCode {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x20 => Ok(TrapCode::GETC),
            0x21 => Ok(TrapCode::OUT),
            0x22 => Ok(TrapCode::PUTS),
            0x23 => Ok(TrapCode::IN),
            0x24 => Ok(TrapCode::PUTSP),
            0x25 => Ok(TrapCode::HALT),
            _ => Err(()),
        }
    }
}

impl VM {
    fn execute_trap(&mut self, trap_code: TrapCode) {
        match trap_code {
            TrapCode::GETC => { self.trap_getc(); }
            TrapCode::OUT => { self.trap_out(); },
            TrapCode::PUTS => { self.trap_puts(); },
            TrapCode::IN => { self.trap_in(); },
            TrapCode::PUTSP => { self.trap_putsp(); },
            TrapCode::HALT => { self.trap_halt(); },
        }
    }

    fn trap_getc(&mut self) {
        // Save current terminal settings
        let stdin_fd = 0; // File descriptor for stdin
        let termios = Termios::from_fd(stdin_fd).unwrap();
        let mut termios_raw = termios;
    
        // Disable echo and canonical mode
        termios_raw.c_lflag &= !(ECHO | ICANON);
        tcsetattr(stdin_fd, TCSANOW, &termios_raw).unwrap();
    
        // Read a single byte (char)
        let mut buffer = [0u8; 1];
        io::stdin().read_exact(&mut buffer).unwrap();
    
        // Store the read character in R0 (converted to u16)
        self.reg.update(0, buffer[0] as u16); 
    
        // Restore the original terminal settings
        tcsetattr(stdin_fd, TCSANOW, &termios).unwrap();
    }
    
    fn trap_out(&mut self) {
        // Extract the lower 8 bits (R0[7:0]) from the R0 register
        let ch = (self.reg.get(0) & 0xFF) as u8 as char;
        print!("{}", ch);
        std::io::stdout().flush().unwrap();
    }
    
    fn trap_puts(&mut self) {
        let mut address = self.reg.get(0); // Starting address from R0
    
        // Loop over the memory starting from the address in R0 until we hit a zero (null terminator)
        loop {
            let value = self.mem.read(address);
    
            // Break the loop if we encounter a null terminator (0)
            if value == 0 {
                break;
            }
    
            // Convert the u16 memory value to a char and print it
            let ch = value as u8 as char;
            print!("{}", ch); // Print each character to stdout
    
            // Move to the next memory word
            address += 1;
        }
    
        // Flush stdout to ensure all output is printed
        std::io::stdout().flush().unwrap();
    }
    
    fn trap_in(&mut self) {
        // Print the prompt
        print!("Enter a character: ");
        io::stdout().flush().unwrap(); // Ensure the prompt is printed immediately
    
        // Read a single character from stdin
        let mut buffer = [0u8; 1]; // Buffer to hold the single byte input
        io::stdin().read_exact(&mut buffer).unwrap();
    
        // Echo the character to the console
        let ch = buffer[0] as char;
        print!("{}", ch);
        io::stdout().flush().unwrap(); // Flush the output to ensure the character is printed
    
        // Store the ASCII value of the character in R0, clearing the high 8 bits
        self.reg.update(0, buffer[0] as u16);
    }
    
    fn trap_putsp(&mut self) {
        let mut address = self.reg.get(0); // Starting address from R0
    
        // Loop through memory until a word containing 0x0000 is found
        loop {
            let word = self.mem.read(address); // Read the 16-bit word from memory
    
            if word == 0 {
                break; // Stop when we hit 0x0000 (null terminator)
            }
    
            // Extract the lower 8 bits (first character)
            let char1 = (word & 0xFF) as u8 as char;
            print!("{}", char1); // Print the first character
    
            // Extract the upper 8 bits (second character)
            let char2 = (word >> 8) as u8 as char;
            if char2 != '\0' { // If the second character isn't null, print it
                print!("{}", char2);
            }
    
            // Move to the next memory word
            address += 1;
        }
    
        // Flush the output to make sure it's displayed immediately
        std::io::stdout().flush().unwrap();
    }
    
    fn trap_halt(&mut self) {
        println!("HALT");
        self.running = false; // This will stop execution
    }

}
