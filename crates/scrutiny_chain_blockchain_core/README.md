# Scrutiny Chain Blockchain Core

Core blockchain functionality and traits for the Scrutiny Chain blockchain security analysis platform.

## Overview

The `scrutiny_chain_blockchain_core` crate provides the foundational abstractions and models for interacting with blockchain data in the Scrutiny Chain ecosystem. It defines key interfaces that other crates implement to ensure consistent interaction with various blockchain data sources.

## Features

### Blockchain Data Provider

The core of this crate is the `BlockchainDataProvider` trait, which defines the standard interface for accessing blockchain data:

```rust
use scrutiny_chain_blockchain_core::blockchain::BlockchainDataProvider;
use scrutiny_chain_common::types::{Address, Hash};

// Example usage with any provider implementing the trait
async fn check_contract(provider: &impl BlockchainDataProvider, address: &Address) {
    let is_contract = provider.is_contract(address).await.unwrap();
    
    if is_contract {
        let contract = provider.get_contract(address).await.unwrap();
        println!("Contract bytecode size: {}", contract.bytecode_size());
    }
}
```

### Blockchain Models

The crate defines standard models for blockchain data:

#### Transaction

```rust
use scrutiny_chain_blockchain_core::models::Transaction;
use scrutiny_chain_common::types::{Address, Hash};

// Create a new transaction
let tx = Transaction::new(
    Hash("0x123".to_string()),
    Address("0xabc".to_string()),
    Some(Address("0xdef".to_string())),
    1000,   // value
    50,     // gas price
    21000,  // gas limit
    5,      // nonce
    vec![], // data
);

// Calculate total cost
let cost = tx.total_cost(); // value + (gas_price * gas_limit)

// Check if it's a contract creation
let is_creation = tx.is_contract_creation();
```

#### Smart Contract

```rust
use scrutiny_chain_blockchain_core::models::SmartContract;
use scrutiny_chain_common::types::Address;

// Create a new smart contract
let contract = SmartContract::new(
    Address("0x789".to_string()),
    vec![1, 2, 3],  // bytecode
    Address("0xabc".to_string()), // creator
    "0x123".to_string(), // creation tx
);

// Get contract details
let bytecode_size = contract.bytecode_size();
let age = contract.age_in_seconds();
```

#### Security Analysis

```rust
use scrutiny_chain_blockchain_core::models::SecurityAnalysis;
use scrutiny_chain_common::types::RiskLevel;
use std::collections::HashMap;

// Create a security analysis result
let analysis = SecurityAnalysis {
    risk_level: RiskLevel::High,
    findings: vec!["Reentrancy vulnerability detected".to_string()],
    metadata: HashMap::new(),
};
```

### Blockchain Errors

The crate defines specific error types for blockchain operations:

```rust
use scrutiny_chain_blockchain_core::blockchain::BlockchainError;

// Handle specific blockchain errors
match result {
    Err(e) if e.downcast_ref::<BlockchainError>() == Some(&BlockchainError::ContractNotFound(_)) => {
        println!("Not a contract address");
    }
    _ => { /* Handle other cases */ }
}
```

## Requirements

- Rust 2021 edition or later
- The `scrutiny_chain_common` crate from the Scrutiny Chain workspace

## Usage

Add this crate as a dependency in your `Cargo.toml`:

```toml
[dependencies]
scrutiny_chain_blockchain-core = { path = "../scrutiny_chain_blockchain-core" }
```

## Development

### Building

```bash
cargo build -p scrutiny_chain_blockchain-core
```

### Testing

```bash
cargo test -p scrutiny_chain_blockchain-core
```

### Watch Mode Testing

```bash
cargo watch -x "test -p scrutiny_chain_blockchain-core"
```

### Documentation

Generate and view the documentation:

```bash
cargo doc -p scrutiny_chain_blockchain-core --no-deps --open
```

## Implementation Example

Here's a simplified example of implementing the `BlockchainDataProvider` trait:

```rust
use scrutiny_chain_blockchain_core::blockchain::{BlockchainDataProvider, BlockchainError};
use scrutiny_chain_blockchain_core::models::{Transaction, SmartContract, SecurityAnalysis};
use scrutiny_chain_common::types::{Address, Hash, TimeRange};
use scrutiny_chain_common::error::Result;
use async_trait::async_trait;
use std::collections::HashMap;

struct EthereumProvider {
    // Configuration and client details
}

#[async_trait]
impl BlockchainDataProvider for EthereumProvider {
    async fn get_transaction(&self, hash: &Hash) -> Result<Transaction> {
        // Implementation to fetch transaction from Ethereum
    }
    
    async fn get_contract(&self, address: &Address) -> Result<SmartContract> {
        // Implementation to fetch contract from Ethereum
    }
    
    // Implement other required methods...
}
```

## License

[MIT](LICENSE)