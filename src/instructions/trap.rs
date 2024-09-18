use crate::hardware::vm::VM;

impl VM {
    /// Trap
    /// Executes a trap instruction, which handles I/O operations and system calls.
    pub fn op_trap(&mut self, instr: u16) {}
}
