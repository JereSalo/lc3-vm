use crate::condition_flag::ConditionFlag;

pub struct Registers {
    pub general: [u16;8], // General purpose registers
    pub pc: u16, // Program Counter
    pub cond: ConditionFlag
}

impl Registers {
    pub fn new() -> Self {
        Registers { general: [0;8], pc: 0, cond: ConditionFlag::Zro }
    }
}

