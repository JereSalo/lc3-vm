use crate::components::{vm::VM, vm_error::VmError};

use super::sign_extend;

impl VM {
    /// ## Load Effective Address
    /// Loads the effective address (not the value) into a register.
    pub fn op_lea(&mut self, instr: u16) -> Result<(), VmError> {
        let dr = (instr >> 9) & 0b111; // Destination Register
        let pc_offset = sign_extend(instr & 0x1FF, 9);

        let address = self.reg.pc.wrapping_add(pc_offset);
        self.reg.update(dr, address)?;
        Ok(())
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

        vm.op_lea(instr).unwrap();

        assert_eq!(vm.reg.get(1).unwrap(), expected_address)
    }
}
