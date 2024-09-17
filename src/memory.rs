const MEMORY_MAX: usize = 1 << 16;

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

    pub fn read(&self, position: usize) -> u16 {
        self.memory[position]
    }
}
