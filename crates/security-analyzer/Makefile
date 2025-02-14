.PHONY: build test check doc bench profile help

# Build the crate
build:
	cargo build -p security-analyzer

# Run tests
test:
	cargo test -p security-analyzer

# Run cargo check
check:
	cargo check -p security-analyzer

# Generate documentation
doc:
	cargo doc -p security-analyzer --no-deps --open

# Run benchmarks
bench:
	cargo bench -p security-analyzer

# Run with flamegraph profiling
profile:
	cargo flamegraph -p security-analyzer

# Run clippy
lint:
	cargo clippy -p security-analyzer -- -D warnings

help:
	@echo "Available targets:"
	@echo "  build   - Build the crate"
	@echo "  test    - Run tests"
	@echo "  check   - Check for compilation"
	@echo "  doc     - Generate documentation"
	@echo "  bench   - Run benchmarks"
	@echo "  profile - Generate flamegraph"
	@echo "  lint    - Run clippy lints"