use crate::components::{vm::VM, vm_error::VmError};

use super::sign_extend;

impl VM {
    /// ## Store Base + Offset
    /// Stores a register value into memory using a base register and an offset.
    pub fn op_str(&mut self, instr: u16) -> Result<(), VmError> {
        let sr = (instr >> 9) & 0b111; // Source Register
        let br = (instr >> 6) & 0b111; // Base Register
        let offset = sign_extend(instr & 0b111111, 6);

        let value = self.reg.get(sr)?;

        let address = self.reg.get(br)?.wrapping_add(offset);

        self.mem.write(address, value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::VM;

    #[test]
    fn str() {
        let mut vm = VM::new();
        let expected_value = 50;
        vm.reg.update(0, expected_value).unwrap();
        vm.reg.update(1, 0x5000).unwrap();

        let instr = 0b0111_000_001_001010; // Write value from reg 0 into the memory address that's in reg 1 + an offset of 10

        vm.op_str(instr).unwrap();

        let actual_value = vm.mem.read(vm.reg.get(1).unwrap() + 10);

        assert_eq!(actual_value, expected_value);
    }
}
