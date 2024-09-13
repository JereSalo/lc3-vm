use crate::condition_flag::ConditionFlag;

pub struct Registers {
    general: [u16;8],
    pc: u16,
    cond: ConditionFlag
}

impl Registers {
    pub fn new() -> Self {
        Registers { general: [0;8], pc: 0, cond: ConditionFlag::Zro }
    }
}

