use crate::hardware::{vm::VM, vm_error::VmError};

impl VM {
    /// ## Bitwise NOT
    /// Performs a bitwise NOT (complement) on a value and stores the result in a register.
    pub fn op_not(&mut self, instr: u16) -> Result<(), VmError> {
        // Destination Register (DR) number
        let dr = (instr >> 9) & 0x7;

        // Source Register
        let sr = (instr >> 6) & 0x7;

        // '!' is the Bitwise NOT operator for unsigned integers.
        self.reg.update(dr, !self.reg.get(sr)?)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::VM;

    #[test]
    fn op_not_basic() {
        let mut vm = VM::new();

        // Set source register R1
        vm.reg.update(1, 0b0000000000001111).unwrap(); // R1 = 15

        // Instruction: NOT R0, R1 (0001 000 001 000 000)
        let instr = 0b1001_000_001_000_000; // Opcode: NOT (1001), DR = 0, SR = 1

        vm.op_not(instr).unwrap();

        // Expected result: NOT 15 = 0b1111111111110000 (in 16-bit)
        assert_eq!(vm.reg.get(0).unwrap(), 0b1111111111110000);
    }

    #[test]
    fn op_not_zero() {
        let mut vm = VM::new();

        // Set source register R1
        vm.reg.update(1, 0b0000000000000000).unwrap(); // R1 = 0

        // Instruction: NOT R0, R1 (0001 000 001 000 000)
        let instr = 0b1001_000_001_000_000; // Opcode: NOT (1001), DR = 0, SR = 1

        vm.op_not(instr).unwrap();

        // Expected result: NOT 0 = 0b1111111111111111 (in 16-bit)
        assert_eq!(vm.reg.get(0).unwrap(), 0b1111111111111111);
    }

    #[test]
    fn op_not_all_ones() {
        let mut vm = VM::new();

        // Set source register R1
        vm.reg.update(1, 0b1111111111111111).unwrap(); // R1 = 65535

        // Instruction: NOT R0, R1 (0001 000 001 000 000)
        let instr = 0b1001_000_001_000_000; // Opcode: NOT (1001), DR = 0, SR = 1

        vm.op_not(instr).unwrap();

        // Expected result: NOT 65535 = 0b0000000000000000 (in 16-bit)
        assert_eq!(vm.reg.get(0).unwrap(), 0b0000000000000000);
    }

    #[test]
    fn test_op_not_mixed_bits() {
        let mut vm = VM::new();

        // Set source register R1 with mixed bits
        vm.reg.update(1, 0b1010101010101010).unwrap(); // R1 = 43690 (in decimal)

        // Instruction: NOT R0, R1 (0001 000 001 000 000)
        let instr = 0b1001_000_001_000_000; // Opcode: NOT (1001), DR = 0, SR = 1

        vm.op_not(instr).unwrap();

        // Expected result: NOT 43690 = 0b0101010101010101 (in 16-bit)
        assert_eq!(vm.reg.get(0).unwrap(), 0b0101010101010101);
    }
}
