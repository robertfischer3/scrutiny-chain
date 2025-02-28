// blockchain-core/src/models.rs
use scrutiny_chain_common::types::{Address, Hash};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a blockchain transaction with its metadata
/// 
/// # Examples
/// 
/// ```
/// use scrutiny_chain_blockchain_core::models::Transaction;
/// use scrutiny_chain_common::types::{Address, Hash};
/// 
/// let tx = Transaction {
///     hash: Hash("0x123".to_string()),
///     from: Address("0xabc".to_string()),
///     to: Some(Address("0xdef".to_string())),
///     value: 1000,
///     gas_price: 50,
///     gas_limit: 21000,
///     nonce: 5,
///     data: vec![1, 2, 3],
///     timestamp: 1645484400,
/// };
/// 
/// assert_eq!(tx.gas_price, 50);
/// assert!(tx.to.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Transaction hash
    pub hash: Hash,
    /// Sender address
    pub from: Address,
    /// Recipient address (None for contract creation)
    pub to: Option<Address>,
    /// Transaction value in wei
    pub value: u64,
    /// Gas price in wei
    pub gas_price: u64,
    /// Gas limit
    pub gas_limit: u64,
    /// Transaction nonce
    pub nonce: u64,
    /// Transaction data
    pub data: Vec<u8>,
    /// Transaction timestamp
    pub timestamp: u64,
}

impl SmartContract {
    /// Creates a new smart contract instance
    /// 
    /// # Examples
    /// 
    /// ```
    /// use scrutiny_chain_blockchain_core::models::SmartContract;
    /// use scrutiny_chain_common::types::Address;
    /// 
    /// let contract = SmartContract::new(
    ///     Address("0x789".to_string()),
    ///     vec![1, 2, 3],
    ///     Address("0xabc".to_string()),
    ///     "0x123".to_string(),
    /// );
    /// 
    /// assert_eq!(contract.bytecode, vec![1, 2, 3]);
    /// ```
    pub fn new(
        address: Address,
        bytecode: Vec<u8>,
        creator: Address,
        creation_tx: String,
    ) -> Self {
        Self {
            address,
            bytecode,
            creator,
            creation_tx,
            storage: HashMap::new(),
            timestamp: scrutiny_chain_common::utils::current_timestamp(),
        }
    }

    /// Checks if the contract has a specific storage key
    /// 
    /// # Examples
    /// 
    /// ```
    /// use scrutiny_chain_blockchain_core::models::SmartContract;
    /// use scrutiny_chain_common::types::Address;
    /// 
    /// let mut contract = SmartContract::new(
    ///     Address("0x789".to_string()),
    ///     vec![],
    ///     Address("0xabc".to_string()),
    ///     "0x123".to_string(),
    /// );
    /// 
    /// contract.storage.insert("balance".to_string(), vec![0, 0, 0, 42]);
    /// assert!(contract.has_storage("balance"));
    /// assert!(!contract.has_storage("nonexistent"));
    /// ```
    pub fn has_storage(&self, key: &str) -> bool {
        self.storage.contains_key(key)
    }

    /// Returns the contract age in seconds
    /// 
    /// # Examples
    /// 
    /// ```
    /// use scrutiny_chain_blockchain_core::models::SmartContract;
    /// use scrutiny_chain_common::types::Address;
    /// use std::thread::sleep;
    /// use std::time::Duration;
    /// 
    /// let contract = SmartContract::new(
    ///     Address("0x789".to_string()),
    ///     vec![],
    ///     Address("0xabc".to_string()),
    ///     "0x123".to_string(),
    /// );
    /// 
    /// sleep(Duration::from_secs(1));
    /// assert!(contract.age_in_seconds() >= 1);
    /// ```
    pub fn age_in_seconds(&self) -> u64 {
        scrutiny_chain_common::utils::current_timestamp().saturating_sub(self.timestamp)
    }

    /// Returns the size of the contract bytecode in bytes
    /// 
    /// # Examples
    /// 
    /// ```
    /// use scrutiny_chain_blockchain_core::models::SmartContract;
    /// use scrutiny_chain_common::types::Address;
    /// 
    /// let contract = SmartContract::new(
    ///     Address("0x789".to_string()),
    ///     vec![1, 2, 3, 4, 5],
    ///     Address("0xabc".to_string()),
    ///     "0x123".to_string(),
    /// );
    /// 
    /// assert_eq!(contract.bytecode_size(), 5);
    /// ```
    pub fn bytecode_size(&self) -> usize {
        self.bytecode.len()
    }
}

/// Represents a smart contract on the blockchain
/// 
/// # Examples
/// 
/// ```
/// use scrutiny_chain_blockchain_core::models::SmartContract;
/// use scrutiny_chain_common::types::Address;
/// use std::collections::HashMap;
/// 
/// let mut contract = SmartContract {
///     address: Address("0x789".to_string()),
///     bytecode: vec![0, 1, 2],
///     creator: Address("0xabc".to_string()),
///     creation_tx: "0x123".to_string(),
///     storage: HashMap::new(),
///     timestamp: 1645484400,
/// };
/// 
/// // Add some storage values
/// contract.storage.insert("balance".to_string(), vec![0, 0, 0, 42]);
/// 
/// assert!(contract.storage.contains_key("balance"));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartContract {
    /// Contract address
    pub address: Address,
    /// Contract bytecode
    pub bytecode: Vec<u8>,
    /// Contract creator address
    pub creator: Address,
    /// Creation transaction hash
    pub creation_tx: String,
    /// Contract storage
    pub storage: HashMap<String, Vec<u8>>,
    /// Contract creation timestamp
    pub timestamp: u64,
}

/// Result of a security analysis
/// 
/// # Examples
/// 
/// ```
/// use scrutiny_chain_blockchain_core::models::SecurityAnalysis;
/// use scrutiny_chain_common::types::RiskLevel;
/// 
/// let analysis = SecurityAnalysis {
///     risk_level: RiskLevel::High,
///     findings: vec!["Reentrancy vulnerability detected".to_string()],
///     metadata: Default::default(),
/// };
/// 
/// assert_eq!(analysis.risk_level, RiskLevel::High);
/// assert!(!analysis.findings.is_empty());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAnalysis {
    /// Overall risk level
    pub risk_level: scrutiny_chain_common::types::RiskLevel,
    /// List of security findings
    pub findings: Vec<String>,
    /// Additional metadata about the analysis
    pub metadata: HashMap<String, String>,
}

impl Transaction {
    /// Calculates the total transaction cost (value + gas cost)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use scrutiny_chain_blockchain_core::models::Transaction;
    /// use scrutiny_chain_common::types::{Address, Hash};
    /// 
    /// let tx = Transaction::new(
    ///     Hash("0x123".to_string()),
    ///     Address("0xabc".to_string()),
    ///     Some(Address("0xdef".to_string())),
    ///     1000,  // value
    ///     50,    // gas price
    ///     21000, // gas limit
    ///     5,
    ///     vec![],
    /// );
    /// 
    /// // Total cost = value + (gas_price * gas_limit)
    /// assert_eq!(tx.total_cost(), 1000 + (50 * 21000));
    /// ```
    pub fn total_cost(&self) -> u64 {
        self.value + (self.gas_price * self.gas_limit)
    }

    /// Checks if the transaction is a contract creation
    /// 
    /// # Examples
    /// 
    /// ```
    /// use scrutiny_chain_blockchain_core::models::Transaction;
    /// use scrutiny_chain_common::types::{Address, Hash};
    /// 
    /// // Contract creation transaction (no 'to' address)
    /// let contract_tx = Transaction::new(
    ///     Hash("0x123".to_string()),
    ///     Address("0xabc".to_string()),
    ///     None,
    ///     0,
    ///     50,
    ///     21000,
    ///     5,
    ///     vec![1, 2, 3], // Contract bytecode
    /// );
    /// 
    /// assert!(contract_tx.is_contract_creation());
    /// ```
    pub fn is_contract_creation(&self) -> bool {
        self.to.is_none() && !self.data.is_empty()
    }

    /// Returns the age of the transaction in seconds
    /// 
    /// # Examples
    /// 
    /// ```
    /// use scrutiny_chain_blockchain_core::models::Transaction;
    /// use scrutiny_chain_common::types::{Address, Hash};
    /// use std::thread::sleep;
    /// use std::time::Duration;
    /// 
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
    /// sleep(Duration::from_secs(1));
    /// assert!(tx.age_in_seconds() >= 1);
    /// ```
    pub fn age_in_seconds(&self) -> u64 {
        scrutiny_chain_common::utils::current_timestamp().saturating_sub(self.timestamp)
    }

    /// Creates a new transaction
    /// 
    /// # Examples
    /// 
    /// ```
    /// use scrutiny_chain_blockchain_core::models::Transaction;
    /// use scrutiny_chain_common::types::{Address, Hash};
    /// 
    /// let tx = Transaction::new(
    ///     Hash("0x123".to_string()),
    ///     Address("0xabc".to_string()),
    ///     Some(Address("0xdef".to_string())),
    ///     1000,
    ///     50,
    ///     21000,
    ///     5,
    ///     vec![1, 2, 3],
    /// );
    /// 
    /// assert_eq!(tx.value, 1000);
    /// ```
    pub fn new(
        hash: Hash,
        from: Address,
        to: Option<Address>,
        value: u64,
        gas_price: u64,
        gas_limit: u64,
        nonce: u64,
        data: Vec<u8>,
    ) -> Self {
        Self {
            hash,
            from,
            to,
            value,
            gas_price,
            gas_limit,
            nonce,
            data,
            timestamp: scrutiny_chain_common::utils::current_timestamp(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use scrutiny_chain_common::types::RiskLevel;

    #[test]
    fn test_transaction_creation() {
        let tx = Transaction::new(
            Hash("0x123".to_string()),
            Address("0xabc".to_string()),
            Some(Address("0xdef".to_string())),
            1000,
            50,
            21000,
            5,
            vec![1, 2, 3],
        );

        assert_eq!(tx.hash.0, "0x123");
        assert_eq!(tx.from.0, "0xabc");
        assert_eq!(tx.to.unwrap().0, "0xdef");
        assert_eq!(tx.value, 1000);
        assert_eq!(tx.gas_price, 50);
        assert_eq!(tx.gas_limit, 21000);
        assert_eq!(tx.nonce, 5);
        assert_eq!(tx.data, vec![1, 2, 3]);
        assert!(tx.timestamp > 0);
    }

    #[test]
    fn test_smart_contract() {
        let contract = SmartContract {
            address: Address("0x789".to_string()),
            bytecode: vec![0, 1, 2],
            creator: Address("0xabc".to_string()),
            creation_tx: "0x123".to_string(),
            storage: HashMap::new(),
            timestamp: scrutiny_chain_common::utils::current_timestamp(),
        };

        assert_eq!(contract.address.0, "0x789");
        assert_eq!(contract.bytecode, vec![0, 1, 2]);
        assert_eq!(contract.creator.0, "0xabc");
    }

    #[test]
    fn test_security_analysis() {
        let findings = vec!["Vulnerability found".to_string()];
        let mut metadata = HashMap::new();
        metadata.insert("scanner".to_string(), "test".to_string());

        let analysis = SecurityAnalysis {
            risk_level: RiskLevel::High,
            findings: findings.clone(),
            metadata,
        };

        assert_eq!(analysis.risk_level, RiskLevel::High);
        assert_eq!(analysis.findings, findings);
        assert_eq!(analysis.metadata.get("scanner").unwrap(), "test");
    }
}