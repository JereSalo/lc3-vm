# Rusty LC3-VM
Implementation of LC3-VM in Rust following [this guide](https://www.jmeiners.com/lc3-vm/).  
The Virtual machine will be able to run any LC-3 program.  
The opcodes were implemented according to [this specification](https://www.jmeiners.com/lc3-vm/supplies/lc3-isa.pdf).

## Testing the Implemented Opcodes
You can run the tests for the opcodes using the following command:
```bash
cargo test
```

## Running the Virtual Machine
You can use the Makefile to easily run LC3 programs.

### Example Commands:
To run the `2048.obj` example:
```bash
make 2048
```

To run the `rogue.obj` example:
```bash
make rogue
```

## Code Formatting and Linting
You can ensure the code is properly formatted and linted using `cargo fmt` and `cargo clippy`.

- To format the code:
```bash
make fmt
```

- To run the linter:
```bash
make clippy
```

## Building the Project
To build the project:
```bash
make build
```

## Cleaning the Project
To clean the build artifacts:
```bash
make clean
```

## Full Project Check
To run code formatting, linting, tests, and build all together:
```bash
make check
```
