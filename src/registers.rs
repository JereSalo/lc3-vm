use crate::condition_flag::ConditionFlag;

pub struct Registers {
    pub general: [u16;8], // General purpose registers
    pub pc: u16, // Program Counter
    pub cond: ConditionFlag
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

impl Registers {
    pub fn new() -> Self {
        Registers { general: [0;8], pc: 0, cond: ConditionFlag::Zro }
    }

    pub fn update(&mut self, r: usize, value: u16) {
        self.general[r] = value;
        self.update_flags(r);
    }

    fn update_flags(&mut self, r: usize) {
        if self.general[r] == 0 {
            self.cond = ConditionFlag::Zro;
        }
        else if (self.general[r] >> 15) == 1 {
            self.cond = ConditionFlag::Neg;
        }
        else {
            self.cond = ConditionFlag::Pos;
        }
    }
}

