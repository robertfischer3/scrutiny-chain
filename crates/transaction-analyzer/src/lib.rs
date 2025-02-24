// transaction-analyzer/src/lib.rs
//! Transaction analyzer for blockchain transactions
//!
//! This crate provides a pluggable architecture for analyzing blockchain
//! transactions. It supports async operations, comprehensive logging, and
//! a flexible plugin system for different analysis strategies.
//!
//! # Examples
//!
//! ```
//! use transaction_analyzer::processor::TransactionProcessor;
//! use transaction_analyzer::ml::MLTransactionAnalyzer;
//! use blockchain_core::models::Transaction;
//! use common::types::{Address, Hash};
//! use std::sync::Arc;
//!
//! # tokio_test::block_on(async {
//! // Create and configure the transaction processor
//! let mut processor = TransactionProcessor::new();
//! processor.register_analyzer(Arc::new(MLTransactionAnalyzer::new()));
//!
//! // Analyze a transaction
//! let tx = Transaction::new(
//!     Hash("0x123".to_string()),
//!     Address("0xabc".to_string()),
//!     Some(Address("0xdef".to_string())),
//!     1000,
//!     50,
//!     21000,
//!     5,
//!     vec![],
//! );
//!
//! let results = processor.process_transaction(&tx).await.unwrap();
//! println!("Analysis results: {:?}", results);
//! # })
//! ```

pub mod processor;
pub mod ml;

// Re-export main types
pub use processor::{TransactionAnalyzer, TransactionProcessor};
pub use ml::MLTransactionAnalyzer;

/// Initialize the transaction analyzer library
///
/// This function sets up logging and prepares the library for use.
///
/// # Examples
///
/// ```
/// use transaction_analyzer::init;
///
/// # tokio_test::block_on(async {
/// // Initialize the library
/// init().await;
/// # })
/// ```
pub async fn init() {
    common::logging::init_logger().await;
    tracing::info!("Transaction analyzer initialized");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::processor::TransactionProcessor;
    use crate::ml::MLTransactionAnalyzer;
    use blockchain_core::models::Transaction;
    use common::types::{Address, Hash};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_integration() {
        init().await;
        
        let mut processor = TransactionProcessor::new();
        processor.register_analyzer(Arc::new(MLTransactionAnalyzer::new()));
        
        let tx = Transaction::new(
            Hash("0x123".to_string()),
            Address("0xabc".to_string()),
            Some(Address("0xdef".to_string())),
            1000,
            50,
            21000,
            5,
            vec![],
        );
        
        let results = processor.process_transaction(&tx).await.unwrap();
        assert!(!results.is_empty());
        assert!(results.contains_key("ml_analysis"));
    }
}