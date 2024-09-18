use lc3_vm::hardware::vm::VM;
use lc3_vm::utils::*;
fn main() {
    let mut vm = VM::new();

    // Load Arguments
    load_arguments(&mut vm);

    // Set up
    set_up();

    // Run VM
    vm.run();
}
