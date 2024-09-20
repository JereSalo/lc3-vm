use crate::hardware::vm_error::VmError;

pub mod add;
pub mod and;
pub mod br;
pub mod jmp;
pub mod jsr;
pub mod ld;
pub mod ldi;
pub mod ldr;
pub mod lea;
pub mod not;
pub mod st;
pub mod sti;
pub mod str;
pub mod trap;

fn sign_extend(mut x: u16, bit_count: u8) -> u16 {
    if (x >> (bit_count - 1)) & 1 != 0 {
        x |= 0xFFFF << bit_count;
    }
    x
}

pub enum Opcode {
    OpBr,   // Branch
    OpAdd,  // Add
    OpLd,   // Load
    OpSt,   // Store
    OpJsr,  // Jump to Subroutine
    OpAnd,  // Bitwise AND
    OpLdr,  // Load Base+Offset
    OpStr,  // Store Base+Offset
    OpRti,  // Return from Interrupt (Unused)
    OpNot,  // Bitwise NOT
    OpLdi,  // Load Indirect
    OpSti,  // Store Indirect
    OpJmp,  // Jump
    OpRes,  // Reserved (Unused)
    OpLea,  // Load Effective Address
    OpTrap, // Trap
}

use std::fmt;

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Opcode::OpBr => "Branch",
            Opcode::OpAdd => "Add",
            Opcode::OpLd => "Load",
            Opcode::OpSt => "Store",
            Opcode::OpJsr => "Jump to Subroutine",
            Opcode::OpAnd => "Bitwise AND",
            Opcode::OpLdr => "Load Base+Offset",
            Opcode::OpStr => "Store Base+Offset",
            Opcode::OpRti => "Return from Interrupt",
            Opcode::OpNot => "Bitwise NOT",
            Opcode::OpLdi => "Load Indirect",
            Opcode::OpSti => "Store Indirect",
            Opcode::OpJmp => "Jump",
            Opcode::OpRes => "Reserved (Unused)",
            Opcode::OpLea => "Load Effective Address",
            Opcode::OpTrap => "Trap",
        };
        write!(f, "{}", name)
    }
}

impl TryFrom<u16> for Opcode {
    type Error = VmError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Opcode::OpBr),
            1 => Ok(Opcode::OpAdd),
            2 => Ok(Opcode::OpLd),
            3 => Ok(Opcode::OpSt),
            4 => Ok(Opcode::OpJsr),
            5 => Ok(Opcode::OpAnd),
            6 => Ok(Opcode::OpLdr),
            7 => Ok(Opcode::OpStr),
            8 => Ok(Opcode::OpRti),
            9 => Ok(Opcode::OpNot),
            10 => Ok(Opcode::OpLdi),
            11 => Ok(Opcode::OpSti),
            12 => Ok(Opcode::OpJmp),
            13 => Ok(Opcode::OpRes),
            14 => Ok(Opcode::OpLea),
            15 => Ok(Opcode::OpTrap),
            _ => Err(VmError::InvalidOpcode(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_extend_positive_number() {
        // Positive number, should not change with sign extension
        // Example: 0b00101 (5 in 5-bit) -> should stay the same
        let x: u16 = 0b00101;
        let result = sign_extend(x, 5);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_sign_extend_negative_number() {
        // Negative number, sign should be extended
        // Example: 0b11110 (in 5-bit) -> should become 0xFFFE (-2 in 16-bit signed)
        let x: u16 = 0b11110;
        let result = sign_extend(x, 5);
        assert_eq!(result as i16, -2);
    }
}
