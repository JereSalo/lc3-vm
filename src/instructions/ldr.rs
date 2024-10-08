use crate::components::{vm::VM, vm_error::VmError};

use super::sign_extend;

impl VM {
    /// ## Load Base + Offset
    /// Loads a value from memory into a register using a base register and an offset.
    pub fn op_ldr(&mut self, instr: u16) -> Result<(), VmError> {
        let dr = (instr >> 9) & 0b111; // Destination register.
        let br = (instr >> 6) & 0b111; // Base register
        let br_offset = sign_extend(instr & 0b111111, 6); // Offset from Base Register

        let final_address = self.reg.get(br)?.wrapping_add(br_offset);
        let value = self.mem.read(final_address);

        self.reg.update(dr, value)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::VM;

    #[test]
    fn ldr() {
        let mut vm = VM::new();
        // Put something in some memory address, say 0x300A
        let written_value = 99;
        vm.mem.write(0x300A, written_value);
        vm.reg.update(1, 0x3000).unwrap();

        let instr = 0b0110_000_001_001010; // LDR to register 0 from register 1's address with offset 10.
        vm.op_ldr(instr).unwrap();

        assert_eq!(vm.reg.get(0).unwrap(), written_value); // It should be equal to that value that was written in memory before
    }
}
