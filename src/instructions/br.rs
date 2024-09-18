use crate::hardware::vm::VM;

use super::sign_extend;

impl VM {
    /// Branch
    /// Conditional branch based on the condition flags.
    pub fn op_br(&mut self, instr: u16) {
        // nzp: Negative - Zero - Positive. 
        let nzp = (instr >> 9) & 0b111;
        let pc_offset = instr & 0x1FF;

        if nzp & self.reg.cond as u16 > 0 {
            self.reg.pc = self.reg.pc.wrapping_add(sign_extend(pc_offset, 9));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VM;
    use crate::hardware::condition_flag::ConditionFlag;

    #[test]
    fn test_op_br_pos_true() {
        // instr: 0000 001 000001010 -> This is BRp with 10 offset.
        let mut vm = VM::new();
        vm.reg.cond = ConditionFlag::Pos; // Condition flag set to Pos
        vm.reg.pc = 0x3000;

        let instr = 0b0000_001_000001010; // BRp with offset 10
        vm.op_br(instr);

        let expected_pc = 0x300A;

        assert_eq!(expected_pc, vm.reg.pc);
    }

    #[test]
    fn test_op_br_pos_false() {
        // instr: 0000 001 000001010 -> This is BRp with 10 offset.
        let mut vm = VM::new();
        vm.reg.cond = ConditionFlag::Neg; // Condition flag set to Neg!
        vm.reg.pc = 0x3000;
        let expected_pc = vm.reg.pc; // Shouldn't change the PC after this operation.

        let instr = 0b0000_001_000001010; // BRp with offset 10
        vm.op_br(instr);

        assert_eq!(expected_pc, vm.reg.pc);
    }
}
