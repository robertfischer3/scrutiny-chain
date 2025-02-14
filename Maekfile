.PHONY: build test check lint clean doc run-api help

# Default target
all: check build test

# Build all crates in workspace
build:
	cargo build --workspace

# Run tests for all crates
test:
	cargo test --workspace

# Run cargo check on all crates
check:
	cargo check --workspace

# Run clippy on all crates
lint:
	cargo clippy --workspace -- -D warnings

# Clean build artifacts
clean:
	cargo clean

# Generate documentation
doc:
	cargo doc --workspace --no-deps

# Run the API server
run-api:
	cargo run -p api-server

# Format code
format:
	cargo fmt --all

# Install development dependencies
setup:
	rustup component add clippy rustfmt
	cargo install cargo-audit

# Security audit
audit:
	cargo audit

# Watch tests
watch-test:
	cargo watch -x test

# Help command
help:
	@echo "Available targets:"
	@echo "  build       - Build all crates"
	@echo "  test        - Run all tests"
	@echo "  check      - Check all crates for compilation"
	@echo "  lint       - Run clippy lints"
	@echo "  clean      - Clean build artifacts"
	@echo "  doc        - Generate documentation"
	@echo "  run-api    - Run the API server"
	@echo "  format     - Format code using rustfmt"
	@echo "  setup      - Install development dependencies"
	@echo "  audit      - Run security audit"
	@echo "  watch-test - Run tests in watch mode"