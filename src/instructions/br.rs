use crate::vm::{vm::VM, condition_flag::ConditionFlag};

use super::sign_extend;

impl VM {
    pub fn op_br(&mut self, instr: u16) {
        // It is a conditional jump based on flags N Z and P.
        let n = ((instr >> 11) & 0x1) == 1;
        let z = ((instr >> 10) & 0x1) == 1;
        let p = ((instr >> 9) & 0x1) == 1;

        let pc_offset = instr & 0x1FF;

        
        // This could be more pretty
        if n && self.reg.cond == ConditionFlag::Neg || z && self.reg.cond == ConditionFlag::Zro || p && self.reg.cond ==ConditionFlag::Pos {
            self.reg.pc = self.reg.pc.wrapping_add(sign_extend(pc_offset, 9));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
