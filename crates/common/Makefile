.PHONY: build test check doc lint help

# Build the crate
build:
	cargo build -p common

# Run tests
test:
	cargo test -p common

# Run cargo check
check:
	cargo check -p common

# Generate documentation
doc:
	cargo doc -p common --no-deps --open

# Run clippy
lint:
	cargo clippy -p common -- -D warnings

help:
	@echo "Available targets:"
	@echo "  build  - Build the crate"
	@echo "  test   - Run tests"
	@echo "  check  - Check for compilation"
	@echo "  doc    - Generate documentation"
	@echo "  lint   - Run clippy lints"