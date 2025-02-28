# Scrutiny Chain Common

A shared library containing common utilities and types for the Scrutiny Chain blockchain security analysis platform.

## Overview

The `common` crate provides foundational functionality used across the Scrutiny Chain ecosystem, including:

- Error handling with a unified error type
- Common blockchain data types
- Utility functions for hex conversion and timestamp operations
- Structured logging configuration with tracing
- Asynchronous utilities for retry operations and timeouts

## Features

### Error Handling

The crate provides a unified error handling system through the `Error` enum and a `Result` type alias:

```rust
use common::error::{Error, Result};

fn may_fail() -> Result<()> {
    // Return specific error types
    Err(Error::validation("invalid input"))
}
```

### Common Types

Fundamental blockchain types are defined in the `types` module:

```rust
use common::types::{Address, Hash, RiskLevel, TimeRange};

let address = Address("0x742d35Cc6634C0532925a3b844Bc454e4438f44e".to_string());
let hash = Hash("0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925".to_string());

// Risk levels for security findings
let risk = RiskLevel::High;

// Time ranges for queries
let time_range = TimeRange::new(start_timestamp, end_timestamp);
```

### Hex Utilities

Functions for converting between hex strings and byte arrays:

```rust
use common::utils::{hex_to_bytes, bytes_to_hex};

// Convert hex to bytes
let bytes = hex_to_bytes("0x1234ab").unwrap();

// Convert bytes back to hex
let hex = bytes_to_hex(&bytes);
```

### Logging

Structured logging with different configuration options:

```rust
use common::logging::{init_logger, init_logger_with_level, init_json_logger};
use tracing::{info, Level};

// Initialize default logger (INFO level)
init_logger().await;

// Initialize logger with custom level
init_logger_with_level(Level::DEBUG).await;

// Initialize production JSON logger
init_json_logger().await;

// Log events
info!("Application started");
```

### Async Utilities

Functions for robust async operations:

```rust
use common::async_utils::{retry_with_backoff, with_timeout};
use std::time::Duration;

// Retry an async operation with exponential backoff
let result = retry_with_backoff(
    || async { fallible_operation().await },
    3,                      // max retries
    Duration::from_secs(1)  // initial delay
).await;

// Run an async operation with a timeout
let result = with_timeout(
    Duration::from_secs(5),
    async_operation()
).await;
```

## Usage

Add this crate as a dependency in your `Cargo.toml`:

```toml
[dependencies]
common = { path = "../common" }
```

Then import the needed components:

```rust
use common::{error::Result, types::Address, utils::current_timestamp};
```

## Development

### Building

```bash
cargo build -p common
```

### Testing

```bash
cargo test -p common
```

### Documentation

Generate and view the documentation:

```bash
cargo doc -p common --no-deps --open
```

## License

[MIT](LICENSE)