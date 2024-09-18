use crate::hardware::vm::VM;

use super::sign_extend;

impl VM {
    pub fn op_sti(&mut self, instr: u16) {
        let sr: usize = ((instr >> 9) & 0b111).into();
        let pc_offset = sign_extend(instr & 0x1FF, 9);

        let intermediate_address = (self.reg.pc.wrapping_add(pc_offset)).into();
        let value = self.reg.general[sr];

        let destination_address = (self.mem.read(intermediate_address)).into();

        self.mem.write(destination_address, value);
    }
}

#[cfg(test)]
mod tests {
    use super::VM;

    #[test]
    fn sti() {
        let mut vm = VM::new();
        let expected_value = 50;
        vm.reg.update(1, expected_value);
        vm.reg.pc = 0x3000;

        vm.mem.write(0x300A, 0x4000);

        let instr = 0b1011_001_000001010; // Write in the address that is contained in the address that is 10 positions away from PC the value stored in register 1
                                          // Example: If PC = 1 and offset is 10: We read Mem[11], it's going to have another address. And then we write Mem[address] = value

        vm.op_sti(instr);

        let actual_value = vm.mem.read(0x4000);

        assert_eq!(actual_value, expected_value);
    }
}
