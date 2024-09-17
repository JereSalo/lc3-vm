use crate::hardware::vm::VM;

use super::sign_extend;

impl VM {
    pub fn op_add(&mut self, instr: u16) {
        // Destination Register (DR) number
        let r0: usize = ((instr >> 9) & 0x7).into();

        // First Operand (SR1) register number
        let r1: usize = ((instr >> 6) & 0x7).into();

        // Flag that indicates mode (Immediate || Register)
        let imm_flag = (instr >> 5) & 0x1;

        let final_value = if imm_flag == 1 {
            // Immediate mode: sign-extend the 5-bit immediate value to a 16bit one.
            let imm5 = sign_extend(instr & 0x1F, 5);
            self.reg.general[r1].wrapping_add(imm5)
        } else {
            // Register mode: add the contents of the registers
            let r2: usize = (instr & 0x7).into();
            self.reg.general[r1].wrapping_add(self.reg.general[r2])
        };
        // I used wrapping_add because it handles overflow cases correctly

        self.reg.update(r0, final_value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_add_register_mode() {
        let mut vm = VM::new();

        // Set initial values in registers
        vm.reg.general[1] = 5; // SR1 (r1)
        vm.reg.general[2] = 10; // SR2 (r2)

        // Encoded instruction: ADD r0, r1, r2
        // Opcode: ADD (0001), DR: r0 (000), SR1: r1 (001), Mode: Register (0), SR2: r2 (010)
        let instr: u16 = 0b0001_000_001_0_00_010;

        vm.op_add(instr);

        // r0 should now contain r1 + r2 (5 + 10 = 15)
        assert_eq!(vm.reg.general[0], 15);
    }

    #[test]
    fn op_add_zero() {
        let mut vm = VM::new();

        // Set initial values in registers
        vm.reg.general[1] = 10; // SR1 (r1)
        vm.reg.general[2] = 0; // SR2 (r2)

        // Encoded instruction: ADD r0, r1, r2
        // Opcode: ADD (0001), DR: r0 (000), SR1: r1 (001), Mode: Register (0), SR2: r2 (010)
        let instr: u16 = 0b0001_000_001_0_00_010;

        vm.op_add(instr);

        // r0 should now contain r1 + r2 (10 + 0 = 10)
        assert_eq!(vm.reg.general[0], 10);
    }

    #[test]
    fn test_op_add_immediate_mode_positive() {
        let mut vm = VM::new();

        // Set initial values in registers
        vm.reg.general[1] = 5; // SR1 (r1)

        // Encoded instruction: ADD r0, r1, imm5
        // Opcode: ADD (0001), DR: r0 (000), SR1: r1 (001), Mode: Immediate (1), imm5: 2 (00010)
        let instr: u16 = 0b0001_000_001_1_00010;

        vm.op_add(instr);

        // r0 should now contain r1 + imm5 (5 + 2 = 7)
        assert_eq!(vm.reg.general[0], 7);
    }

    #[test]
    fn op_add_immediate_mode_negative() {
        let mut vm = VM::new();

        // Set initial values in registers
        vm.reg.general[1] = 5; // SR1 (r1)

        // Encoded instruction: ADD r0, r1, imm5
        // Opcode: ADD (0001), DR: r0 (000), SR1: r1 (001), Mode: Immediate (1), imm5: -1 (11111)
        let instr: u16 = 0b0001_000_001_1_11111;

        vm.op_add(instr);

        // r0 should now contain r1 + imm5 (5 + (-1) = 4)
        assert_eq!(vm.reg.general[0], 4);
    }

    #[test]
    fn op_add_immediate_mode_negative_change_sign() {
        let mut vm = VM::new();

        vm.reg.general[0] = 3; // SR1 (r1)

        // Opcode: ADD (0001), DR: r0 (000), SR1: r1 (000), Mode: Immediate (1), imm5: -14
        let instr: u16 = 0b0001_000_000_1_10010;

        vm.op_add(instr);

        // r0 should now contain r1 + imm5 (3 + (-14) = -11)
        // Since we're using u16, -11 needs to be represented correctly in unsigned form
        assert_eq!(vm.reg.general[0], (u16::MAX - 10)); // -11 in u16 is equivalent to 65525
    }
}
