use crate::hardware::vm::VM;

impl VM {
    /// Jump
    /// Jumps to a location specified by a register.
    pub fn op_jmp(&mut self, instr: u16) {
        let r = (instr >> 6) & 0b111;

        self.reg.pc = self.reg.get(r);
    }
}

#[cfg(test)]
mod tests {
    use super::VM;

    #[test]
    fn jump() {
        let mut vm = VM::new();
        vm.reg.pc = 0x3000;
        let r = 3;
        let expected_pc = 0x777;
        vm.reg.update(r, expected_pc); // Store expected program count on register.

        let instr = 0b1100_000_011_000000; // JMP to location in register 3 (011)

        vm.op_jmp(instr);

        assert_eq!(vm.reg.pc, expected_pc);
    }
}
