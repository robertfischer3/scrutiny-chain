// blockchain-core/src/lib.rs
//! Core blockchain functionality and traits
//! 
//! This crate provides the foundational types and traits for blockchain interaction
//! and analysis. It defines the core interfaces that other crates will implement
//! and build upon.

use tracing::{debug, info};

pub mod models;
pub mod blockchain;

// Re-export main types and traits
pub use blockchain::{BlockchainDataProvider, BlockchainError};
pub use models::{Transaction, SmartContract, SecurityAnalysis};

/// Initialize logging for the blockchain-core crate
pub async fn init() {
    info!("Initializing blockchain-core");
    common::logging::init_logger().await;
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