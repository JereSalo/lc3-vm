pub enum Opcode {
    OpAdd,
    OpAnd,
    OpNot,
    OpBr,
    OpJmp,
    OpJsr,
    OpLd,
    OpLdi,
    OpLdr,
    OpLea,
    OpSt,
    OpSti,
    OpStr,
    OpTrap,
    OpRes,
    OpRti,
}

/// Convert u16 into Opcode
impl TryFrom<u16> for Opcode {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Opcode::OpAdd),
            1 => Ok(Opcode::OpAnd),
            2 => Ok(Opcode::OpNot),
            3 => Ok(Opcode::OpBr),
            4 => Ok(Opcode::OpJmp),
            5 => Ok(Opcode::OpJsr),
            6 => Ok(Opcode::OpLd),
            7 => Ok(Opcode::OpLdi),
            8 => Ok(Opcode::OpLdr),
            9 => Ok(Opcode::OpLea),
            10 => Ok(Opcode::OpSt),
            11 => Ok(Opcode::OpSti),
            12 => Ok(Opcode::OpStr),
            13 => Ok(Opcode::OpTrap),
            14 => Ok(Opcode::OpRes),
            15 => Ok(Opcode::OpRti),
            _ => Err(()),
        }
    }
}
