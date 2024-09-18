# Run the 2048 example
2048:
	cargo run -- ./examples/2048.obj

# Run the Rogue example
rogue:
	cargo run -- ./examples/rogue.obj

# Run Clippy (Rust linter)
clippy:
	cargo clippy

# Run Rustfmt (code formatter)
fmt:
	cargo fmt --all

# Run tests
test:
	cargo test

# Build the project
build:
	cargo build

# Clean the target directory
clean:
	cargo clean

# Full check: run clippy, fmt, test, and build
check: fmt clippy test build
