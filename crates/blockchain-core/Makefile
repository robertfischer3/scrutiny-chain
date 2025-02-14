.PHONY: build test check doc watch-test help

# Build the crate
build:
	cargo build -p blockchain-core

# Run tests
test:
	cargo test -p blockchain-core

# Run cargo check
check:
	cargo check -p blockchain-core

# Generate documentation
doc:
	cargo doc -p blockchain-core --no-deps --open

# Watch tests
watch-test:
	cargo watch -x "test -p blockchain-core"

# Run clippy
lint:
	cargo clippy -p blockchain-core -- -D warnings

help:
	@echo "Available targets:"
	@echo "  build      - Build the crate"
	@echo "  test       - Run tests"
	@echo "  check      - Check for compilation"
	@echo "  doc        - Generate documentation"
	@echo "  watch-test - Run tests in watch mode"
	@echo "  lint       - Run clippy lints"