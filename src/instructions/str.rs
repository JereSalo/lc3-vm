use crate::hardware::vm::VM;

use super::sign_extend;

impl VM {
    pub fn op_str(&mut self, instr: u16) {
        let sr: usize = ((instr >> 9) & 0b111).into();
        let br: usize = ((instr >> 6) & 0b111).into();
        let offset = instr & 0b111111;

        let address = (self.reg.general[br].wrapping_add(sign_extend(offset, 6))).into();
        let value = self.reg.general[sr];
        self.mem.write(address, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str() {
        let mut vm = VM::new();
        let expected_value = 50;
        vm.reg.update(0, expected_value);
        vm.reg.update(1, 0x5000);

        let instr = 0b0111_000_001_001010; // Write value from reg 0 into the memory address that's in reg 1 + an offset of 10

        vm.op_str(instr);

        let actual_value = vm.mem.read((vm.reg.general[1] + 10).into());

        assert_eq!(actual_value, expected_value);
    }
}
