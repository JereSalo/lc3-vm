use crate::hardware::vm::VM;

use super::sign_extend;

impl VM {
    /// Load
    /// Gets a destination register and a pc_offset, and writes into that register the value in memory location PC + Offset.
    pub fn op_ld(&mut self, instr: u16) {
        let dr = (instr >> 9) & 0b111; // Destination register.
        let pc_offset = sign_extend(instr & 0x1FF, 9);

        let final_address = self.reg.pc.wrapping_add(pc_offset);
        let value_read = self.mem.read(final_address);
        self.reg.update(dr, value_read);
    }
}

#[cfg(test)]
mod tests {
    use super::VM;

    #[test]
    fn ld() {
        let mut vm = VM::new();
        // Put something in some memory address, say 0x300A
        let written_value = 99;
        vm.mem.write(0x300A, written_value);
        vm.reg.pc = 0x3000;

        let instr = 0b0010_001_000001010; // LD to register 1 with pc_offset 10
        vm.op_ld(instr);

        assert_eq!(vm.reg.general[1], written_value); // It should be equal to that value that was written in memory before
    }
}
