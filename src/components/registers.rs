use super::vm_error::VmError;

pub const PC_START: u16 = 0x3000;

pub struct Registers {
    pub general: [u16; 8], // General purpose registers
    pub pc: u16,           // Program Counter
    pub cond: ConditionFlag,
}

/// Flag that saves if last operation was zero, positive or negative.
#[derive(PartialEq, Clone, Copy)]
pub enum ConditionFlag {
    Pos = 1 << 0, // 001
    Zro = 1 << 1, // 010
    Neg = 1 << 2, // 100
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            general: [0; 8],
            pc: PC_START,
            cond: ConditionFlag::Zro,
        }
    }

    pub fn update(&mut self, r: u16, value: u16) -> Result<(), VmError> {
        if (r as usize) < self.general.len() {
            self.general[r as usize] = value;
            self.update_flags(r as usize);
            Ok(())
        } else {
            Err(VmError::InvalidRegister(r))
        }
    }

    /// Gets value of general purpose register
    pub fn get(&self, r: u16) -> Result<u16, VmError> {
        self.general
            .get(r as usize)
            .copied()
            .ok_or(VmError::InvalidRegister(r))
    }

    fn update_flags(&mut self, r: usize) {
        if self.general[r] == 0 {
            self.cond = ConditionFlag::Zro;
        } else if (self.general[r] >> 15) == 1 {
            self.cond = ConditionFlag::Neg;
        } else {
            self.cond = ConditionFlag::Pos;
        }
    }
}
