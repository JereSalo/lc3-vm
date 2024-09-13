const MEMORY_MAX: usize = 1 << 16;

pub struct Memory {
    memory: [u16;MEMORY_MAX],
}

impl Memory {
    fn new() -> Self {
        Memory { memory: [0;MEMORY_MAX] }
    }
}
