use crate::components::{vm::VM, vm_error::VmError};

impl VM {
    /// ## Jump
    /// Jumps to a location specified by a register.
    pub fn op_jmp(&mut self, instr: u16) -> Result<(), VmError> {
        let r = (instr >> 6) & 0b111;

        self.reg.pc = self.reg.get(r)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::VM;

    #[test]
    fn jump() {
        let mut vm = VM::new();
        vm.reg.pc = 0x3000;
        let expected_pc = 0x777;
        vm.reg.update(3, expected_pc).unwrap(); // Store expected program count on register.

        let instr = 0b1100_000_011_000000; // JMP to location in register 3 (011)

        vm.op_jmp(instr).unwrap();

        assert_eq!(vm.reg.pc, expected_pc);
    }
}
