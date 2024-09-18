use std::io::{self, Read};

const MEMORY_MAX: usize = 1 << 16;

pub struct Memory {
    pub memory: [u16; MEMORY_MAX],
}

enum MemoryMappedRegister {
    /// Keyboard Status
    MrKbsr = 0xFE00,
    /// Keyboard Data
    MrKbdr = 0xFE02
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            memory: [0; MEMORY_MAX],
        }
    }

    pub fn read(&mut self, address: usize) -> u16 {
        if address == MemoryMappedRegister::MrKbsr as usize {
            if let Err(e) = self.handle_keyboard() {
                eprintln!("Critical Error: Unable to handle keyboard input: {}", e);
                std::process::exit(1); // Terminate program
            }
        }

        self.memory[address]
    }

    fn handle_keyboard(&mut self) -> io::Result<()> {
        let mut buffer = [0; 1];
        std::io::stdin().read_exact(&mut buffer)?;
        if buffer[0] != 0 {
            self.write(MemoryMappedRegister::MrKbsr as usize, 1 << 15);
            self.write(MemoryMappedRegister::MrKbdr as usize, buffer[0] as u16);
        } else {
            self.write(MemoryMappedRegister::MrKbsr as usize, 0)
        }
        Ok(())
    }

    pub fn write(&mut self, address: usize, value: u16) {
        self.memory[address] = value;
    }
}
