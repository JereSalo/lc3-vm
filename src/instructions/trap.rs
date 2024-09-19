use std::io::Write;

use crate::hardware::vm::VM;

impl VM {
    /// ## Trap
    /// Executes a trap instruction, which handles I/O operations and system calls.
    pub fn op_trap(&mut self, instr: u16) {
        // Save the program counter to general-purpose register 7
        self.reg.update(7, self.reg.pc);

        // Check the lower byte of the instruction and execute the corresponding trap routine
        match instr & 0xFF {
            0x20 => { trap_getc(self); },
            0x21 => { trap_out(self); },
            0x22 => { trap_puts(self); },
            0x23 => { trap_in(self); },
            0x24 => { trap_putsp(self); },
            0x25 => { trap_halt(self); },
            _ => {}
        }
    }
}

use std::io::{self, Read};
use termios::{Termios, TCSANOW, tcsetattr, ECHO, ICANON};

fn trap_getc(vm: &mut VM) {
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
    vm.reg.update(0, buffer[0] as u16); 

    // Restore the original terminal settings
    tcsetattr(stdin_fd, TCSANOW, &termios).unwrap();
}

fn trap_out(vm: &mut VM) {
    // Extract the lower 8 bits (R0[7:0]) from the R0 register
    let ch = (vm.reg.general[0] & 0xFF) as u8 as char;
    print!("{}", ch);
    std::io::stdout().flush().unwrap();
}

fn trap_puts(vm: &mut VM) {
    let mut address = vm.reg.general[0]; // Starting address from R0

    // Loop over the memory starting from the address in R0 until we hit a zero (null terminator)
    loop {
        let value = vm.mem.read(address);

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

fn trap_in(vm: &mut VM) {
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
    vm.reg.update(0, buffer[0] as u16);
}

fn trap_putsp(vm: &mut VM) {
    let mut address = vm.reg.general[0]; // Starting address from R0

    // Loop through memory until a word containing 0x0000 is found
    loop {
        let word = vm.mem.read(address); // Read the 16-bit word from memory

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

fn trap_halt(vm: &mut VM) {
    println!("HALT");
    vm.running = false; // This will stop execution
}
