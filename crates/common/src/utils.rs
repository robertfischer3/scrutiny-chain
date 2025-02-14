use std::time::{SystemTime, UNIX_EPOCH};

/// Get current timestamp in seconds
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Convert a hex string to bytes
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