// blockchain-core/src/blockchain.rs
use crate::models::{Transaction, SmartContract, SecurityAnalysis};
use async_trait::async_trait;
use common::{
    error::Result,
    types::{Address, Hash, TimeRange},
};
use tracing::{debug, error, info, warn};

/// Error types specific to blockchain operations
#[derive(Debug, thiserror::Error)]
pub enum BlockchainError {
    #[error("Node connection error: {0}")]
    ConnectionError(String),

    #[error("Invalid block hash: {0}")]
    InvalidBlockHash(String),

    #[error("Invalid transaction hash: {0}")]
    InvalidTransactionHash(String),

    #[error("Contract not found at address: {0}")]
    ContractNotFound(String),

    #[error("RPC error: {0}")]
    RPCError(String),
}

/// Trait defining the interface for blockchain data providers
/// 
/// This trait must be implemented by any concrete blockchain implementation
/// to provide access to blockchain data.
/// 
/// # Examples
/// 
/// ```
/// use blockchain_core::blockchain::{BlockchainDataProvider, BlockchainError};
/// use blockchain_core::models::{Transaction, SmartContract};
/// use common::types::{Address, Hash, TimeRange};
/// use common::error::Result;
/// use async_trait::async_trait;
/// use std::collections::HashMap;
/// 
/// struct MockProvider;
/// 
/// #[async_trait]
/// impl BlockchainDataProvider for MockProvider {
///     async fn get_transaction(&self, hash: &Hash) -> Result<Transaction> {
///         // Mock implementation
///         Ok(Transaction::new(
///             hash.clone(),
///             Address("0xabc".to_string()),
///             Some(Address("0xdef".to_string())),
///             1000,
///             50,
///             21000,
///             5,
///             vec![],
///         ))
///     }
///     
///     async fn get_contract(&self, address: &Address) -> Result<SmartContract> {
///         // Mock implementation
///         Ok(SmartContract {
///             address: address.clone(),
///             bytecode: vec![],
///             creator: Address("0xabc".to_string()),
///             creation_tx: "0x123".to_string(),
///             storage: HashMap::new(),
///             timestamp: 0,
///         })
///     }
///     
///     // Other required methods would be implemented here...
/// }
/// ```
#[async_trait]
pub trait BlockchainDataProvider: Send + Sync {
    /// Retrieves a transaction by its hash
    async fn get_transaction(&self, hash: &Hash) -> Result<Transaction>;

    /// Retrieves a smart contract by its address
    async fn get_contract(&self, address: &Address) -> Result<SmartContract>;

    /// Retrieves all transactions within a given time range
    async fn get_transactions_in_range(&self, range: TimeRange) -> Result<Vec<Transaction>>;

    /// Retrieves all transactions for a specific address
    async fn get_address_transactions(&self, address: &Address) -> Result<Vec<Transaction>>;

    /// Retrieves the current balance of an address
    async fn get_balance(&self, address: &Address) -> Result<u64>;

    /// Retrieves the nonce (transaction count) for an address
    async fn get_nonce(&self, address: &Address) -> Result<u64>;

    /// Performs a security analysis on a smart contract
    async fn analyze_contract(&self, address: &Address) -> Result<SecurityAnalysis>;

    /// Checks if an address is a contract
    async fn is_contract(&self, address: &Address) -> Result<bool> {
        debug!("Checking if address {} is a contract", address);
        
        match self.get_contract(address).await {
            Ok(_) => {
                info!("Address {} is a contract", address);
                Ok(true)
            }
            Err(e) => {
                if let Some(blockchain_err) = e.downcast_ref::<BlockchainError>() {
                    match blockchain_err {
                        BlockchainError::ContractNotFound(_) => {
                            debug!("Address {} is not a contract", address);
                            Ok(false)
                        }
                        _ => {
                            error!("Error checking contract status: {}", e);
                            Err(e)
                        }
                    }
                } else {
                    warn!("Unexpected error type checking contract status: {}", e);
                    Err(e)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::types::RiskLevel;
    use std::collections::HashMap;

    struct MockProvider;

    #[async_trait]
    impl BlockchainDataProvider for MockProvider {
        async fn get_transaction(&self, hash: &Hash) -> Result<Transaction> {
            Ok(Transaction::new(
                hash.clone(),
                Address("0xabc".to_string()),
                Some(Address("0xdef".to_string())),
                1000,
                50,
                21000,
                5,
                vec![],
            ))
        }

        async fn get_contract(&self, address: &Address) -> Result<SmartContract> {
            if address.0 == "0xcontract" {
                Ok(SmartContract {
                    address: address.clone(),
                    bytecode: vec![0, 1, 2],
                    creator: Address("0xabc".to_string()),
                    creation_tx: "0x123".to_string(),
                    storage: HashMap::new(),
                    timestamp: 0,
                })
            } else {
                Err(Box::<dyn std::error::Error + Send + Sync>::from(Box::new(BlockchainError::ContractNotFound(address.0.clone()))).into())
            }
        }

        async fn get_transactions_in_range(&self, _range: TimeRange) -> Result<Vec<Transaction>> {
            Ok(vec![])
        }

        async fn get_address_transactions(&self, _address: &Address) -> Result<Vec<Transaction>> {
            Ok(vec![])
        }

        async fn get_balance(&self, _address: &Address) -> Result<u64> {
            Ok(1000)
        }

        async fn get_nonce(&self, _address: &Address) -> Result<u64> {
            Ok(5)
        }

        async fn analyze_contract(&self, _address: &Address) -> Result<SecurityAnalysis> {
            Ok(SecurityAnalysis {
                risk_level: RiskLevel::Low,
                findings: vec![],
                metadata: HashMap::new(),
            })
        }
    }

    #[tokio::test]
    async fn test_mock_provider() {
        let provider = MockProvider;
        
        // Test get_transaction
        let tx = provider.get_transaction(&Hash("0x123".to_string())).await.unwrap();
        assert_eq!(tx.hash.0, "0x123");
        
        // Test is_contract
        assert!(provider.is_contract(&Address("0xcontract".to_string())).await.unwrap());
        assert!(!provider.is_contract(&Address("0xnotcontract".to_string())).await.unwrap());
    }
}