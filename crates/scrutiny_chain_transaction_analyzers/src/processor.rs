// transaction-analyzer/src/processor.rs
use async_trait::async_trait;
use scrutiny_chain_common::{
    error::{Error, Result},
    types::{Address, Hash, TimeRange},
    logging::create_timing_span,
};
use tracing::{debug, error, info, warn, instrument};
use scrutiny_chain_blockchain_core::models::Transaction;
use std::collections::HashMap;
use std::sync::Arc;

/// Trait for transaction analysis strategies
/// 
/// This trait defines the interface for different transaction analysis
/// strategies that can be plugged into the transaction processor.
/// 
/// # Examples
/// 
/// ```
/// use scrutiny_chain_transaction_analyzer::processor::TransactionAnalyzer;
/// use scrutiny_chain_blockchain_core::models::Transaction;
/// use scrutiny_chain_common::error::Result;
/// use scrutiny_chain_common::types::Hash;
/// use async_trait::async_trait;
/// use std::collections::HashMap;
/// 
/// struct SimpleAnalyzer;
/// 
/// #[async_trait]
/// impl TransactionAnalyzer for SimpleAnalyzer {
///     async fn analyze_transaction(&self, tx: &Transaction) -> Result<HashMap<String, String>> {
///         let mut results = HashMap::new();
///         results.insert("status".to_string(), "analyzed".to_string());
///         results.insert("gas_efficiency".to_string(), "good".to_string());
///         Ok(results)
///     }
/// }
/// ```
#[async_trait]
pub trait TransactionAnalyzer: Send + Sync {
    /// Analyzes a single transaction and returns analysis results
    /// 
    /// # Arguments
    /// 
    /// * `tx` - The transaction to analyze
    /// 
    /// # Returns
    /// 
    /// Returns a Result containing a HashMap of analysis results,
    /// or an Error if the analysis fails.
    async fn analyze_transaction(&self, tx: &Transaction) -> Result<HashMap<String, String>>;
}

/// Main transaction processor that coordinates analysis strategies
/// 
/// The TransactionProcessor manages multiple analyzers and coordinates
/// the analysis of blockchain transactions.
/// 
/// # Examples
/// 
/// 
/// use scrutiny_chain_transaction_analyzer::processor::{TransactionProcessor, TransactionAnalyzer};
/// use scrutiny_chain_blockchain_core::models::Transaction;
/// use scrutiny_chain_common::error::Result;
/// use scrutiny_chain_common::types::{Address, Hash};
/// use async_trait::async_trait;
/// use std::collections::HashMap;
/// use std::sync::Arc;
/// 
/// struct GasAnalyzer;
/// 
/// #[async_trait]
/// impl TransactionAnalyzer for GasAnalyzer {
///     async fn analyze_transaction(&self, tx: &Transaction) -> Result<HashMap<String, String>> {
///         let mut results = HashMap::new();
///         results.insert("gas_efficiency".to_string(), 
///             if tx.gas_price > 100 { "poor".to_string() } else { "good".to_string() });
///         Ok(results)
///     }
/// }
/// 
/// # tokio_test::block_on(async {
/// // Create processor and register analyzer
/// let mut processor = TransactionProcessor::new();
/// processor.register_analyzer(Arc::new(GasAnalyzer));
/// 
/// // Create a test transaction
/// let tx = Transaction::new(
///     Hash("0x123".to_string()),
///     Address("0xabc".to_string()),
///     Some(Address("0xdef".to_string())),
///     1000,
///     50,
///     21000,
///     5,
///     vec![],
/// );
/// 
/// // Process the transaction
/// let results = processor.process_transaction(&tx).await?;
/// assert!(results.contains_key("gas_efficiency"));
/// # Ok::<(), common::error::Error>(())
/// # })
/// 
pub struct TransactionProcessor {
    analyzers: Vec<Arc<dyn TransactionAnalyzer>>,
}

impl TransactionProcessor {
    /// Creates a new TransactionProcessor instance
    pub fn new() -> Self {
        info!("Initializing TransactionProcessor");
        Self {
            analyzers: Vec::new(),
        }
    }

    /// Registers a new transaction analyzer
    pub fn register_analyzer(&mut self, analyzer: Arc<dyn TransactionAnalyzer>) {
        debug!("Registering new transaction analyzer");
        self.analyzers.push(analyzer);
    }

    /// Processes a single transaction through all registered analyzers
    /// 
    /// # Arguments
    /// 
    /// * `tx` - The transaction to process
    /// 
    /// # Returns
    /// 
    /// Returns a Result containing a HashMap of analysis results from all analyzers,
    /// or an Error if the processing fails.
    #[instrument(skip(self, tx), fields(tx_hash = %tx.hash), level = "debug")]
    pub async fn process_transaction(&self, tx: &Transaction) -> Result<HashMap<String, String>> {
        let _timing_span = create_timing_span("transaction_analysis", "process");
        info!("Processing transaction {}", tx.hash);

        if self.analyzers.is_empty() {
            warn!("No transaction analyzers registered");
            let mut results = HashMap::new();
            results.insert("status".to_string(), "No analyzers configured".to_string());
            return Ok(results);
        }

        let mut combined_results = HashMap::new();
        
        for (i, analyzer) in self.analyzers.iter().enumerate() {
            match analyzer.analyze_transaction(tx).await {
                Ok(results) => {
                    debug!(
                        "Analyzer {} completed successfully for transaction {}",
                        i,
                        tx.hash
                    );
                    combined_results.extend(results);
                }
                Err(e) => {
                    error!("Analyzer {} failed for transaction {}: {}", i, tx.hash, e);
                    combined_results.insert(
                        format!("analyzer_{}_error", i),
                        format!("Analysis failed: {}", e),
                    );
                }
            }
        }

        debug!(
            "Completed processing transaction {} with {} result fields",
            tx.hash,
            combined_results.len()
        );
        
        Ok(combined_results)
    }

    /// Processes multiple transactions in batch
    /// 
    /// # Arguments
    /// 
    /// * `transactions` - A vector of transactions to process
    /// 
    /// # Returns
    /// 
    /// Returns a Result containing a HashMap mapping transaction hashes to analysis results,
    /// or an Error if the batch processing fails.
    #[instrument(skip(self, transactions), level = "debug")]
    pub async fn process_batch(
        &self,
        transactions: &[Transaction],
    ) -> Result<HashMap<String, HashMap<String, String>>> {
        let _timing_span = create_timing_span("transaction_analysis", "batch_process");
        info!("Processing batch of {} transactions", transactions.len());

        let mut batch_results = HashMap::new();

        for tx in transactions {
            match self.process_transaction(tx).await {
                Ok(results) => {
                    batch_results.insert(tx.hash.to_string(), results);
                }
                Err(e) => {
                    error!("Failed to process transaction {}: {}", tx.hash, e);
                    let mut error_result = HashMap::new();
                    error_result.insert("error".to_string(), format!("Processing failed: {}", e));
                    batch_results.insert(tx.hash.to_string(), error_result);
                }
            }
        }

        info!("Completed batch processing of {} transactions", transactions.len());
        Ok(batch_results)
    }
}

impl Default for TransactionProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockAnalyzer {
        key: String,
        value: String,
    }

    #[async_trait]
    impl TransactionAnalyzer for MockAnalyzer {
        async fn analyze_transaction(&self, _tx: &Transaction) -> Result<HashMap<String, String>> {
            let mut results = HashMap::new();
            results.insert(self.key.clone(), self.value.clone());
            Ok(results)
        }
    }

    #[tokio::test]
    async fn test_empty_processor() {
        let processor = TransactionProcessor::new();
        
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
        assert!(results.contains_key("status"));
        assert_eq!(results.get("status").unwrap(), "No analyzers configured");
    }

    #[tokio::test]
    async fn test_multiple_analyzers() {
        let mut processor = TransactionProcessor::new();
        
        processor.register_analyzer(Arc::new(MockAnalyzer {
            key: "gas_analysis".to_string(),
            value: "efficient".to_string(),
        }));
        
        processor.register_analyzer(Arc::new(MockAnalyzer {
            key: "security".to_string(),
            value: "safe".to_string(),
        }));
        
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
        assert_eq!(results.get("gas_analysis").unwrap(), "efficient");
        assert_eq!(results.get("security").unwrap(), "safe");
    }

    #[tokio::test]
    async fn test_batch_processing() {
        let mut processor = TransactionProcessor::new();
        
        processor.register_analyzer(Arc::new(MockAnalyzer {
            key: "analysis".to_string(),
            value: "complete".to_string(),
        }));
        
        let transactions = vec![
            Transaction::new(
                Hash("0x123".to_string()),
                Address("0xabc".to_string()),
                Some(Address("0xdef".to_string())),
                1000,
                50,
                21000,
                5,
                vec![],
            ),
            Transaction::new(
                Hash("0x456".to_string()),
                Address("0xabc".to_string()),
                Some(Address("0xdef".to_string())),
                2000,
                60,
                21000,
                6,
                vec![],
            ),
        ];

        let batch_results = processor.process_batch(&transactions).await.unwrap();
        assert_eq!(batch_results.len(), 2);
        assert!(batch_results.contains_key("0x123"));
        assert!(batch_results.contains_key("0x456"));
        
        for (_, results) in batch_results {
            assert_eq!(results.get("analysis").unwrap(), "complete");
        }
    }
}