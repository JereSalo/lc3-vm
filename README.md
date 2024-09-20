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

### Code Quality
Format and lint the code:
```bash
make fmt       # Format code
make clippy    # Lint with clippy
```

### Build & Clean
Build the project:
```bash
make build
```
Clean build artifacts:
```bash
make clean
```

### Full Project Check
Run formatting, linting, tests, and build:
```bash
make check
```
