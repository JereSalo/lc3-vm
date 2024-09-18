use lc3_vm::hardware::vm::VM;

fn main() {
    let mut vm = VM::new();

    // Load Arguments
    vm.load_arguments();

    // Run VM
    vm.run();
}
