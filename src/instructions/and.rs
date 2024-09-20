use crate::hardware::vm::VM;

use super::sign_extend;

impl VM {
    /// ## Bitwise AND
    /// Performs bitwise AND between two values and stores the result in a register.
    pub fn op_and(&mut self, instr: u16) -> Result<(), VmError> {
        // Destination Register (DR) number
        let dr = (instr >> 9) & 0x7;

        // First Operand (SR1) register number
        let sr1 = (instr >> 6) & 0x7;

        // Flag that indicates mode (Immediate || Register)
        let imm_flag = (instr >> 5) & 0x1;

        let final_value = if imm_flag == 1 {
            // Immediate mode: sign-extend the 5-bit imm value to 16bit and perform bitwise and with SR1
            let imm5 = sign_extend(instr & 0x1F, 5);
            self.reg.get(sr1)? & imm5
        } else {
            // Register mode: Perform 'bitwise and' with both registers' content.
            let r2 = instr & 0x7;
            self.reg.get(sr1)? & self.reg.get(r2)
        };

        self.reg.update(dr, final_value);
    }
}

#[cfg(test)]
mod tests {
    use super::VM;

    #[test]
    fn op_and_two_registers() {
        let mut vm = VM::new();

        // Set registers for testing
        vm.reg.update(1, 0b1010101010101010); // SR1
        vm.reg.update(2, 0b1100110011001100); // SR2

        // Instruction: AND R0, R1, R2 (0001 000 001 000 010)
        let instr = 0b0101_000_001_000_010; // Opcode: AND (0101), DR = 0, SR1 = 1, SR2 = 2

        // Perform the AND operation
        vm.op_and(instr);

        // Expected result: 0b1000100010001000
        assert_eq!(vm.reg.get(0), 0b1000100010001000);
    }

    #[test]
    fn op_and_register_with_immediate() {
        let mut vm = VM::new();

        // Set register
        vm.reg.update(1, 0b1111000011111000); // SR1

        // Instruction: AND R0, R1, #0b00010 (Immediate)
        // Binary: Opcode: AND (0101), DR = 0, SR1 = 1, Immediate flag = 1, imm5 = 0b00010 (2)
        let instr = 0b0101_000_001_1_01010;

        // Perform the AND operation
        vm.op_and(instr);

        // Expected result: 0b0000000000001000
        assert_eq!(vm.reg.get(0), 0b0000000000001000);
    }

    #[test]
    fn op_and_register_with_sign_extended_immediate() {
        let mut vm = VM::new();

        // Set register
        vm.reg.update(1, 0b1111111111111111); // SR1

        // Instruction: AND R0, R1, #-1 (Immediate, 5-bit signed value)
        // Binary: Opcode: AND (0101), DR = 0, SR1 = 1, Immediate flag = 1, imm5 = 0b11111 (-1 after sign extension)
        let instr = 0b0101_000_001_1_11111;

        // Perform the AND operation
        vm.op_and(instr);

        // Expected result: 0b1111111111111111 (AND-ing with -1 should return the original value)
        assert_eq!(vm.reg.get(0), 0b1111111111111111);
    }

    #[test]
    fn op_and_zero_register() {
        let mut vm = VM::new();

        // Set registers
        vm.reg.update(1, 0b1111111111111111); // SR1
        vm.reg.update(2, 0b0000000000000000); // SR2

        // Instruction: AND R0, R1, R2
        let instr = 0b0101_000_001_000_010; // AND R0 = R1 & R2

        // Perform the AND operation
        vm.op_and(instr);

        // Expected result: 0 (AND-ing with zero should result in zero)
        assert_eq!(vm.reg.get(0), 0b0000000000000000);
    }
}
