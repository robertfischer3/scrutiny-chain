# Scrutiny Chain

AI-Enhanced Blockchain Security Analysis Platform

## Overview

Scrutiny Chain is a comprehensive blockchain security analysis toolkit in development that helps developers and auditors identify vulnerabilities and security issues in smart contracts and blockchain transactions. The platform combines static analysis, pattern matching, and machine learning to provide thorough security insights.

## Features

- **Smart Contract Analysis**: Scan smart contracts for common security vulnerabilities
- **Transaction Pattern Analysis**: Identify suspicious transaction patterns and behaviors
- **Blockchain Data Provider**: Abstract interface for connecting to different blockchain networks
- **API Server**: REST API exposing security analysis capabilities
- **Extensible Architecture**: Modular design allows for custom analyzers and scanners

## Project Structure

The project is organized as a Rust workspace with multiple crates:

- **blockchain-core**: Core functionality and traits for blockchain interactions
- **security-analyzer**: Tools for analyzing smart contract security
- **transaction-analyzer**: Components for analyzing transaction patterns
- **api-server**: REST API server for exposing analysis capabilities
- **common**: Shared utilities, error handling, and data types

## Getting Started

### Prerequisites

- Rust 1.60 or higher
- Cargo

### Installation

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/scrutiny-chain.git
   cd scrutiny-chain
   ```

2. Build the project:
   ```
   cargo build --workspace
   ```

3. Run tests:
   ```
   cargo test --workspace
   ```

### Running the API Server

```
cargo run -p api-server
```

The API server will be available at `http://localhost:8080`.

## Documentation

Generate and view the documentation:

```
cargo doc --workspace --open
```

## Development

### Project Commands

Scrutiny Chain uses makefiles for common development tasks:

```
# Build all crates
make build

# Run tests
make test

# Check code for errors without building
make check

# Run Clippy lints
make lint

# Format code
make format

# Generate documentation
make doc

# Run the API server
make run-api
```

### Adding a New Vulnerability Scanner

1. Implement the `VulnerabilityScanner` trait in the security-analyzer crate:

```rust
use security_analyzer::vulnerabilities::VulnerabilityScanner;
use common::types::Address;
use common::error::Result;
use async_trait::async_trait;

struct MyCustomScanner;

#[async_trait]
impl VulnerabilityScanner for MyCustomScanner {
    async fn scan(&self, address: &Address) -> Result<Vec<String>> {
        // Your scanning logic here
        Ok(vec![
            "Finding 1: Description".to_string(),
            "Finding 2: Description".to_string(),
        ])
    }
}
```

2. Register your scanner with the main `SecurityAnalyzer`.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments
- Inspired by existing blockchain security tools and best practices