use crate::hardware::vm::VM;

use super::sign_extend;

impl VM {
    pub fn op_ldr(&mut self, instr: u16) {
        let dr: usize = ((instr >> 9) & 0b111).into(); // Destination register.
        let br: usize = ((instr >> 6) & 0b111).into(); // Base register
        let pc_offset = sign_extend(instr & 0b111111, 6);

        let final_address = (self.reg.general[br].wrapping_add(pc_offset)).into();
        let value_read = self.mem.read(final_address);
        self.reg.update(dr, value_read);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ldr() {
        let mut vm = VM::new();
        // Put something in some memory address, say 0x300A
        let written_value = 99;
        vm.mem.write(0x300A, written_value);
        vm.reg.update(1, 0x3000);

        let instr = 0b0110_000_001_001010; // LDR to register 0 from register 1's address with offset 10.
        vm.op_ldr(instr);

        assert_eq!(vm.reg.general[0], written_value); // It should be equal to that value that was written in memory before
    }
}
