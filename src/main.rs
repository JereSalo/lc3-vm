pub mod hardware;
pub mod instructions;

use hardware::vm::VM;
fn main() {
    let mut vm = VM::new();

    vm.run().unwrap_or_else(|e| eprintln!("{}", e));
}
