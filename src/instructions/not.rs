use crate::vm::vm::VM;

impl VM {
    pub fn op_not(&mut self, instr: u16) {
        // Destination Register (DR) number
        let r0: usize = ((instr >> 9) & 0x7).into();

        // Source Register
        let r1: usize = ((instr >> 6) & 0x7).into();

        self.reg.update(r0, !self.reg.general[r1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_not_basic() {
        let mut vm = VM::new();

        // Set source register R1
        vm.reg.general[1] = 0b0000000000001111; // R1 = 15

        // Instruction: NOT R0, R1 (0001 000 001 000 000)
        let instr = 0b1001_000_001_000_000; // Opcode: NOT (1001), DR = 0, SR = 1

        // Perform the NOT operation
        vm.op_not(instr);

        // Expected result: NOT 15 = 0b1111111111110000 (in 16-bit)
        assert_eq!(vm.reg.general[0], 0b1111111111110000);
    }

    #[test]
    fn op_not_zero() {
        let mut vm = VM::new();

        // Set source register R1
        vm.reg.general[1] = 0b0000000000000000; // R1 = 0

        // Instruction: NOT R0, R1 (0001 000 001 000 000)
        let instr = 0b1001_000_001_000_000; // Opcode: NOT (1001), DR = 0, SR = 1

        // Perform the NOT operation
        vm.op_not(instr);

        // Expected result: NOT 0 = 0b1111111111111111 (in 16-bit)
        assert_eq!(vm.reg.general[0], 0b1111111111111111);
    }

    #[test]
    fn op_not_all_ones() {
        let mut vm = VM::new();

        // Set source register R1
        vm.reg.general[1] = 0b1111111111111111; // R1 = 65535

        // Instruction: NOT R0, R1 (0001 000 001 000 000)
        let instr = 0b1001_000_001_000_000; // Opcode: NOT (1001), DR = 0, SR = 1

        // Perform the NOT operation
        vm.op_not(instr);

        // Expected result: NOT 65535 = 0b0000000000000000 (in 16-bit)
        assert_eq!(vm.reg.general[0], 0b0000000000000000);
    }

    #[test]
    fn test_op_not_mixed_bits() {
        let mut vm = VM::new();

        // Set source register R1 with mixed bits
        vm.reg.general[1] = 0b1010101010101010; // R1 = 43690 (in decimal)

        // Instruction: NOT R0, R1 (0001 000 001 000 000)
        let instr = 0b1001_000_001_000_000; // Opcode: NOT (1001), DR = 0, SR = 1

        // Perform the NOT operation
        vm.op_not(instr);

        // Expected result: NOT 43690 = 0b0101010101010101 (in 16-bit)
        assert_eq!(vm.reg.general[0], 0b0101010101010101);
    }
}
