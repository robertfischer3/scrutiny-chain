.PHONY: build test check doc bench profile help

# Build the crate
build:
	cargo build -p transaction-analyzer

# Run tests
test:
	cargo test -p transaction-analyzer

# Run cargo check
check:
	cargo check -p transaction-analyzer

# Generate documentation
doc:
	cargo doc -p transaction-analyzer --no-deps --open

# Run benchmarks
bench:
	cargo bench -p transaction-analyzer

# Run with flamegraph profiling
profile:
	cargo flamegraph -p transaction-analyzer

# Run clippy
lint:
	cargo clippy -p transaction-analyzer -- -D warnings

help:
	@echo "Available targets:"
	@echo "  build   - Build the crate"
	@echo "  test    - Run tests"
	@echo "  check   - Check for compilation"
	@echo "  doc     - Generate documentation"
	@echo "  bench   - Run benchmarks"
	@echo "  profile - Generate flamegraph"
	@echo "  lint    - Run clippy lints"