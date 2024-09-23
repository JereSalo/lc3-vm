pub mod components;
pub mod instructions;

use components::vm::VM;
fn main() {
    let mut vm = VM::new();

    vm.run().unwrap_or_else(|e| eprintln!("{}", e));
}
