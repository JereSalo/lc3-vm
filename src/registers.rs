use crate::condition_flag::ConditionFlag;

pub struct Registers {
    general: [u16;8],
    pc: u16,
    cond: ConditionFlag
}

