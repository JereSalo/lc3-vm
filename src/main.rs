use lc3_vm::hardware::vm::VM;

fn main() {
    let mut vm = VM::new();

    vm.load_arguments();

    vm.run();
}
