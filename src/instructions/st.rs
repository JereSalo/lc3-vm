use crate::components::{vm::VM, vm_error::VmError};

use super::sign_extend;

impl VM {
    /// ## Store
    /// Stores a register value into memory, address is offset from pc.
    pub fn op_st(&mut self, instr: u16) -> Result<(), VmError> {
        let sr = (instr >> 9) & 0b111; // Source Register
        let pc_offset = sign_extend(instr & 0x1FF, 9);

        let value = self.reg.get(sr)?;
        let destination_address = self.reg.pc.wrapping_add(pc_offset);

        self.mem.write(destination_address, value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::VM;

    #[test]
    fn st() {
        let mut vm = VM::new();
        let expected_value = 50;
        vm.reg.update(1, expected_value).unwrap();
        vm.reg.pc = 0x3000;

        let instr = 0b0011_001_000001010; // Write in the address 10 positions away from PC the value stored in register 1

        vm.op_st(instr).unwrap();

        let actual_value = vm.mem.read(vm.reg.pc + 10);

        assert_eq!(actual_value, expected_value);
    }
}
