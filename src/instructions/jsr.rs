use crate::hardware::vm::VM;

use super::sign_extend;

impl VM {
    /// Jump to Subroutine
    /// Saves the current PC and jumps to a subroutine.
    pub fn op_jsr(&mut self, instr: u16) {
        let mode = (instr >> 11) & 1;

        self.reg.update(7, self.reg.pc);

        if mode == 0 {
            let br = (instr >> 6) & 0b111;
            self.reg.pc = self.reg.get(br);
        } else {
            let pc_offset = instr & 0b11111111111; // Because it is the last 11 bits
            self.reg.pc = self.reg.pc.wrapping_add(sign_extend(pc_offset, 11));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VM;

    #[test]
    fn jsr() {
        let mut vm = VM::new();
        vm.reg.pc = 0x3000;
        let expected_pc = vm.reg.pc + 127;

        let instr = 0b0100_1_00001111111; // JSR with offset 127

        vm.op_jsr(instr);

        assert_eq!(vm.reg.pc, expected_pc);
    }

    #[test]
    fn jsrr() {
        let mut vm = VM::new();
        vm.reg.pc = 0x3000;
        let expected_pc = vm.reg.pc + 127;
        vm.reg.update(4, expected_pc);

        let instr = 0b0100_0_00_100_000000; // JSRR with offset in register 4 (100)

        vm.op_jsr(instr);

        assert_eq!(vm.reg.pc, expected_pc);
    }
}
