use crate::hardware::vm::VM;

use super::sign_extend;

impl VM {
    /// Load Effective Address
    /// Loads the effective address (not the value) into a register.
    pub fn op_lea(&mut self, instr: u16) {
        let dr: usize = ((instr >> 9) & 0b111).into();
        let pc_offset = sign_extend(instr & 0x1FF, 9);

        let value = self.reg.pc.wrapping_add(pc_offset);
        self.reg.update(dr, value);
    }
}

#[cfg(test)]
mod tests {
    use super::VM;

    #[test]
    fn lea() {
        let mut vm = VM::new();
        vm.reg.pc = 0x3000;
        let expected_address = 0x3000 + 10;

        let instr = 0b1110_001_000001010; // Load Effective Address to register 1 with pc_offset 10

        vm.op_lea(instr);

        assert_eq!(vm.reg.general[1], expected_address)
    }
}
