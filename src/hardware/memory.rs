use std::io::Read;

const MEMORY_MAX: usize = 1 << 16;
const MR_KBSR: usize = 0xFE00;
const MR_KBDR: usize = 0xFE02;
pub struct Memory {
    pub memory: [u16; MEMORY_MAX],
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
        if address == MR_KBSR {
            self.handle_keyboard();
        }

        self.memory[address]
    }

    fn handle_keyboard(&mut self){
        let mut buffer = [0; 1];
        std::io::stdin().read_exact(&mut buffer).unwrap();
        if buffer[0] != 0 {
            self.write(MR_KBSR, 1 << 15);
            self.write(MR_KBDR, buffer[0] as u16);
        } else {
            self.write(MR_KBSR, 0)
        }
    }

    pub fn write(&mut self, address: usize, value: u16) {
        self.memory[address] = value;
    }
}
