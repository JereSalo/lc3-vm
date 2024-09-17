pub enum Opcode {
    OpBr,   // 0
    OpAdd,  // 1
    OpLd,   // 2
    OpSt,   // 3
    OpJsr,  // 4
    OpAnd,  // 5
    OpLdr,  // 6
    OpStr,  // 7
    OpRti,  // 8
    OpNot,  // 9
    OpLdi,  // 10
    OpSti,  // 11
    OpJmp,  // 12
    OpRes,  // 13
    OpLea,  // 14
    OpTrap, // 15
}

/// Convert u16 into Opcode
impl TryFrom<u16> for Opcode {
    type Error = ();

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
            _ => Err(()),
        }
    }
}
