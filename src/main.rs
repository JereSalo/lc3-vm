pub mod hardware;
pub mod instructions;

use hardware::vm::VM;
fn main() {
    let mut vm = VM::new();

    vm.load_arguments();

    vm.run();
}
