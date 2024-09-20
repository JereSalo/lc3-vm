use crate::hardware::{vm::VM, vm_error::VmError};

use super::sign_extend;

impl VM {
    /// ## Load Indirect
    /// Loads a value indirectly from memory into a register.
    pub fn op_ldi(&mut self, instr: u16) -> Result<(), VmError> {
        // Destination Register
        let r0 = (instr >> 9) & 0x7;

        // Offset from PC (Program Counter)
        let pc_offset = sign_extend(instr & 0x1FF, 9);

        let intermediate_address = self.reg.pc.wrapping_add(pc_offset);

        let final_address = self.mem.read(intermediate_address);
        let value = self.mem.read(final_address);

        self.reg.update(r0, value)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::VM;

    // Helper function to set up a VM instance for testing
    fn set_up_vm() -> VM {
        let mut vm = VM::new(); // Initialize VM with a `new` constructor
        vm.reg.pc = 0x3000; // Set program counter to an arbitrary starting value
        vm
    }

    #[test]
    fn op_ldi_basic() {
        let mut vm = set_up_vm();

        // Set memory at the current PC + offset to point to another memory location
        let pc_offset = 10;
        let intermediate_address = 0x3050;
        let final_value = 2020;

        // At PC + offset, store the intermediate address
        vm.mem.write(vm.reg.pc + pc_offset, intermediate_address);

        // At the intermediate address, store the final value
        vm.mem.write(intermediate_address, final_value);

        // LDI instruction: LDI r0, #offset
        // Opcode (LDI): 1010 (0xA), DR: r0 (000), PCoffset: 000001010 (10)
        let instr = 0b1010_000_000001010;

        // Execute LDI instruction
        vm.op_ldi(instr).unwrap();

        // Check that r0 now contains the final value
        assert_eq!(vm.reg.get(0).unwrap(), final_value);
    }

    #[test]
    fn op_ldi_negative_offset() {
        let mut vm = set_up_vm();

        // Set memory at PC + offset to point to another memory location
        let pc_offset = -5i16 as u16; // Use a negative offset (in two's complement)
        let intermediate_address = 0x300A;
        let final_value = 0x5678;

        // At PC - 5, store the intermediate address
        vm.mem.write(
            (vm.reg.pc as i16 + pc_offset as i16) as u16,
            intermediate_address,
        );

        // At the intermediate address, store the final value
        vm.mem.write(intermediate_address, final_value);

        // LDI instruction: LDI r0, #-5 (two's complement of 5 is 111111011 in 9 bits)
        let instr = 0b1010_000_111111011;

        // Execute LDI instruction
        vm.op_ldi(instr).unwrap();

        // Check that r0 now contains the final value
        assert_eq!(vm.reg.get(0).unwrap(), final_value);
    }

    #[test]
    fn op_ldi_zero_address() {
        let mut vm = set_up_vm();

        // Set memory at PC + offset to point to address 0
        let pc_offset = 0;
        let final_value = 0x9ABC;

        // At PC + 0, store 0 (so the next read will be from address 0)
        vm.mem.write(vm.reg.pc + pc_offset, 0);

        // At memory address 0, store the final value
        vm.mem.write(0, final_value);

        // LDI instruction: LDI r0, #0
        let instr = 0b1010_000_000000000;

        // Execute LDI instruction
        vm.op_ldi(instr).unwrap();

        // Check that r0 now contains the final value
        assert_eq!(vm.reg.get(0).unwrap(), final_value);
    }

    #[test]
    fn op_ldi_max_offset() {
        let mut vm = set_up_vm();

        // Use maximum positive 9-bit PC offset (511)
        let pc_offset = 255;
        let intermediate_address = 0x3500;
        let final_value = 0xDEAD;

        // At PC + offset, store the intermediate address
        let offset_address = (vm.reg.pc + pc_offset) as u16;

        vm.mem.write(offset_address, intermediate_address);

        // At the intermediate address, store the final value
        vm.mem.write(intermediate_address, final_value);

        // LDI instruction: LDI r0, #511
        let instr = 0b1010_000_011111111; // 255 is the max value for a 9-bit signed offset

        // Execute LDI instruction
        vm.op_ldi(instr).unwrap();

        // Check that r0 now contains the final value
        assert_eq!(vm.reg.get(0).unwrap(), final_value);
    }
}
