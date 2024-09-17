use lc3_vm::utils::*;
use lc3_vm::hardware::vm::VM;
fn main() {
    // Load Arguments
    load_arguments();

    // Set up
    set_up();

    // Run VM
    let mut vm = VM::new();
    vm.run();
}
