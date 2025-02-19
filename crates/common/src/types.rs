// common/src/types.rs
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a blockchain address
/// 
/// # Examples
/// 
/// ```
/// use common::types::Address;
/// 
/// let addr = Address("0x742d35Cc6634C0532925a3b844Bc454e4438f44e".to_string());
/// assert_eq!(addr.to_string(), "0x742d35Cc6634C0532925a3b844Bc454e4438f44e");
/// 
/// // Addresses can be cloned and compared
/// let addr2 = addr.clone();
/// assert_eq!(addr, addr2);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address(pub String);

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a transaction hash
/// 
/// # Examples
/// 
/// ```
/// use common::types::Hash;
/// 
/// let hash = Hash("0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925".to_string());
/// assert_eq!(
///     hash.to_string(),
///     "0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925"
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hash(pub String);

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents the risk level of a security finding
/// 
/// Risk levels are ordered from lowest to highest:
/// None < Low < Medium < High < Critical
/// 
/// # Examples
/// 
/// ```
/// use common::types::RiskLevel;
/// use std::cmp::Ordering;
/// 
/// // Risk levels can be compared
/// assert!(RiskLevel::Low < RiskLevel::Medium);
/// assert!(RiskLevel::High > RiskLevel::Low);
/// 
/// // They can be converted to strings
/// assert_eq!(RiskLevel::Critical.to_string(), "Critical");
/// 
/// // They can be used in match statements
/// let risk = RiskLevel::High;
/// let action = match risk {
///     RiskLevel::Critical | RiskLevel::High => "immediate action required",
///     RiskLevel::Medium => "monitor closely",
///     _ => "routine monitoring"
/// };
/// assert_eq!(action, "immediate action required");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RiskLevel {
    None,
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RiskLevel::None => write!(f, "None"),
            RiskLevel::Low => write!(f, "Low"),
            RiskLevel::Medium => write!(f, "Medium"),
            RiskLevel::High => write!(f, "High"),
            RiskLevel::Critical => write!(f, "Critical"),
        }
    }
}

/// Represents a time range for queries
/// 
/// # Examples
/// 
/// ```
/// use common::types::TimeRange;
/// 
/// // Create a time range for the last hour
/// let now = std::time::SystemTime::now()
///     .duration_since(std::time::UNIX_EPOCH)
///     .unwrap()
///     .as_secs();
/// let hour_ago = now - 3600;
/// 
/// let range = TimeRange::new(hour_ago, now);
/// 
/// // Check if a timestamp is within the range
/// let test_time = now - 1800; // 30 minutes ago
/// assert!(range.contains(test_time));
/// 
/// // Timestamps before the start or after the end are not contained
/// assert!(!range.contains(hour_ago - 1));
/// assert!(!range.contains(now + 1));
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: u64,
    pub end: u64,
}

impl TimeRange {
    pub fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }

    pub fn contains(&self, timestamp: u64) -> bool {
        timestamp >= self.start && timestamp <= self.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address() {
        let addr = Address("0x123".to_string());
        assert_eq!(addr.to_string(), "0x123");
    }

    #[test]
    fn test_hash() {
        let hash = Hash("0xabc".to_string());
        assert_eq!(hash.to_string(), "0xabc");
    }

    #[test]
    fn test_risk_level_ordering() {
        assert!(RiskLevel::None < RiskLevel::Low);
        assert!(RiskLevel::Low < RiskLevel::Medium);
        assert!(RiskLevel::Medium < RiskLevel::High);
        assert!(RiskLevel::High < RiskLevel::Critical);
    }

    #[test]
    fn test_time_range() {
        let range = TimeRange::new(100, 200);
        assert!(range.contains(150));
        assert!(!range.contains(99));
        assert!(!range.contains(201));
    }
}