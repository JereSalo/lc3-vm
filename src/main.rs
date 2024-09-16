use lc3_vm::utils::*;
use lc3_vm::vm::VM;
fn main() {
    // Load Arguments
    load_arguments();

    // Setup
    setup();

    // Run VM
    let mut vm = VM::new();
    vm.run();
}
