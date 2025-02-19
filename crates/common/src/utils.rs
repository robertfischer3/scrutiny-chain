// common/src/utils.rs
use std::time::{SystemTime, UNIX_EPOCH};

/// Get current timestamp in seconds since the Unix epoch
/// 
/// # Examples
/// 
/// ```
/// use common::utils::current_timestamp;
/// 
/// let now = current_timestamp();
/// assert!(now > 1609459200); // January 1, 2021
/// ```
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Convert a hex string to bytes
/// 
/// The input string can optionally start with "0x".
/// 
/// # Examples
/// 
/// ```
/// use common::utils::hex_to_bytes;
/// 
/// // Convert a hex string without 0x prefix
/// let bytes = hex_to_bytes("1234").unwrap();
/// assert_eq!(bytes, vec![0x12, 0x34]);
/// 
/// // Convert a hex string with 0x prefix
/// let bytes = hex_to_bytes("0x1234").unwrap();
/// assert_eq!(bytes, vec![0x12, 0x34]);
/// 
/// // Invalid hex strings return an error
/// assert!(hex_to_bytes("12ZZ").is_err());
/// ```
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
    let hex = hex.trim_start_matches("0x");
    (0..hex.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex[i..i + 2], 16)
                .map_err(|e| format!("Invalid hex string: {}", e))
        })
        .collect()
}

/// Convert bytes to hex string
/// 
/// The output string always starts with "0x".
/// 
/// # Examples
/// 
/// ```
/// use common::utils::{hex_to_bytes, bytes_to_hex};
/// 
/// let original = "0x1234ab";
/// let bytes = hex_to_bytes(original).unwrap();
/// let hex = bytes_to_hex(&bytes);
/// assert_eq!(original.to_lowercase(), hex.to_lowercase());
/// 
/// // Works with empty byte arrays
/// assert_eq!(bytes_to_hex(&[]), "0x");
/// ```
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    format!("0x{}", hex::encode(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_conversion() {
        let original = "0x1234ab";
        let bytes = hex_to_bytes(original).unwrap();
        let hex = bytes_to_hex(&bytes);
        assert_eq!(original.to_lowercase(), hex.to_lowercase());
    }

    #[test]
    fn test_timestamp() {
        let ts = current_timestamp();
        assert!(ts > 0);
    }
}