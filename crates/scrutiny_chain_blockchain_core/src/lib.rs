// blockchain-core/src/lib.rs
//! Core blockchain functionality and traits
//! 
//! This crate provides the foundational types and traits for blockchain interaction
//! and analysis. It defines the core interfaces that other crates will implement
//! and build upon.

use tracing::{debug, info};

pub mod models;
pub mod blockchain;

// Conditionally include the Ethereum provider
#[cfg(feature = "ethereum")]
pub mod ethereum;

// Re-export main types and traits
pub use blockchain::{BlockchainDataProvider, BlockchainError};
pub use models::{Transaction, SmartContract, SecurityAnalysis};

// Conditionally re-export the Ethereum provider
#[cfg(feature = "ethereum")]
pub use ethereum::EthereumProvider;

/// Initialize logging for the blockchain-core crate
pub async fn init() {
    info!("Initializing blockchain-core");
    scrutiny_chain_common::logging::init_logger().await;
    debug!("Blockchain core initialization complete");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_initialization() {
        init().await;
        info!("Test log message");
    }
}