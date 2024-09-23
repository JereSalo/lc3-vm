# Rusty LC3-VM
Rust implementation of the LC3-VM, following [this guide](https://www.jmeiners.com/lc3-vm/) and [ISA specification](https://www.jmeiners.com/lc3-vm/supplies/lc3-isa.pdf). This VM can run any LC-3 program.

## Usage

### Run Programs
Use the Makefile for easy execution:
```bash
make 2048   # Run 2048.obj
make rogue  # Run rogue.obj
```

### Testing
Run opcode tests:
```bash
make test
```

## Demo
https://github.com/user-attachments/assets/ef91b0b3-a24d-49d4-9460-17e31134fb0d
