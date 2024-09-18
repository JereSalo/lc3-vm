use crate::hardware::vm::VM;

impl VM {
    pub fn op_trap(&mut self, instr: u16) {
        self.reg.general[7] = self.reg.pc;

        match instr >> 0xFF {
            0x20 => {trap_getc();},
            0x21 => {trap_out();},
            0x22 => {trap_puts();},
            0x23 => {trap_in();},
            0x24 => {trap_putsp();},
            0x25 => {trap_halt();},
            _ => {}
        }
    }
}

fn trap_getc() {

}

fn trap_out() {

}

fn trap_puts() {

}

fn trap_in() {

}

fn trap_putsp() {

}

fn trap_halt() {

}

#[cfg(test)]
mod tests {
    use super::*;

    
}
