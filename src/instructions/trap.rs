use crate::hardware::vm::VM;

impl VM {
    pub fn op_trap(&mut self, instr: u16) {
        // Save the program counter to general-purpose register 7
        self.reg.general[7] = self.reg.pc;

        // Check the lower byte of the instruction and execute the corresponding trap routine
        match instr & 0xFF {
            0x20 => { trap_getc(self); },
            0x21 => { trap_out(self); },
            0x22 => { trap_puts(self); },
            0x23 => { trap_in(self); },
            0x24 => { trap_putsp(self); },
            0x25 => { trap_halt(self); },
            _ => {}
        }
    }
}

fn trap_getc(vm: &mut VM) {
    // Implementation of trap_getc
}

fn trap_out(vm: &mut VM) {
    // Implementation of trap_out
}

fn trap_puts(vm: &mut VM) {
    // Implementation of trap_puts
    // Write to stdout characters until I reach 0x0000 and flush the output buffer.
    
}

fn trap_in(vm: &mut VM) {
    // Implementation of trap_in
}

fn trap_putsp(vm: &mut VM) {
    // Implementation of trap_putsp
}

fn trap_halt(vm: &mut VM) {
    // Implementation of trap_halt
}

#[cfg(test)]
mod tests {
    use super::*;
    // Add tests here
}
