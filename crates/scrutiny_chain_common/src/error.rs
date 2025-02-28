// common/src/error.rs
use thiserror::Error;

/// The main error type for the blockchain security analysis platform
/// 
/// # Examples
/// 
/// ```
/// use scrutiny_chain_common::error::{Error, Result};
/// 
/// // Creating errors using helper methods
/// let db_err = Error::database("connection timeout");
/// let blockchain_err = Error::blockchain("invalid block hash");
/// 
/// // Using the Result type alias
/// fn may_fail() -> Result<()> {
///     Err(Error::validation("invalid input"))
/// }
/// 
/// assert!(may_fail().is_err());
/// ```
#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Blockchain error: {0}")]
    Blockchain(String),

    #[error("Smart contract error: {0}")]
    SmartContract(String),

    #[error("Transaction error: {0}")]
    Transaction(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Authorization error: {0}")]
    Authorization(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Analysis error: {0}")]
    Analysis(String),

    #[error("ML model error: {0}")]
    MLModel(String),

    #[error("Data processing error: {0}")]
    DataProcessing(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

/// Result type alias with the common Error type
/// 
/// # Examples
/// 
/// ```
/// use scrutiny_chain_common::error::{Error, Result};
/// 
/// fn divide(a: i32, b: i32) -> Result<i32> {
///     if b == 0 {
///         return Err(Error::validation("division by zero"));
///     }
///     Ok(a / b)
/// }
/// 
/// assert!(divide(10, 2).is_ok());
/// assert!(divide(10, 0).is_err());
/// ```
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Attempts to downcast the error to a specific error type
    /// 
    /// # Examples
    /// 
    /// ```
    /// use scrutiny_chain_common::error::Error;
    /// use std::io::{Error as IoError, ErrorKind};
    /// 
    /// let io_error = IoError::new(ErrorKind::Other, "io error");
    /// let err: Error = Error::Other(Box::new(io_error) as Box<dyn std::error::Error + Send + Sync>);
    /// 
    /// // Try to downcast to IoError
    /// if let Some(io_err) = err.downcast_ref::<IoError>() {
    ///     assert_eq!(io_err.kind(), ErrorKind::Other);
    /// }
    /// ```
    pub fn downcast_ref<T: std::error::Error + 'static>(&self) -> Option<&T> {
        match self {
            Error::Other(err) => err.downcast_ref::<T>(),
            _ => None,
        }
    }

    /// Helper to create a Database error
    /// 
    /// # Examples
    /// 
    /// ```
    /// use scrutiny_chain_common::error::Error;
    /// 
    /// let err = Error::database("connection failed");
    /// assert!(matches!(err, Error::Database(_)));
    /// ```
    pub fn database<T: ToString>(msg: T) -> Self {
        Error::Database(msg.to_string())
    }

    /// Helper to create a Blockchain error
    /// 
    /// # Examples
    /// 
    /// ```
    /// use scrutiny_chain_common::error::Error;
    /// 
    /// let err = Error::blockchain("invalid block");
    /// assert!(matches!(err, Error::Blockchain(_)));
    /// ```
    pub fn blockchain<T: ToString>(msg: T) -> Self {
        Error::Blockchain(msg.to_string())
    }

    /// Helper to create a Validation error
    /// 
    /// # Examples
    /// 
    /// ```
    /// use scrutiny_chain_common::error::Error;
    /// 
    /// let err = Error::validation("invalid input");
    /// assert!(matches!(err, Error::Validation(_)));
    /// ```
    pub fn validation<T: ToString>(msg: T) -> Self {
        Error::Validation(msg.to_string())
    }

    /// Helper to create an Analysis error
    /// 
    /// # Examples
    /// 
    /// ```
    /// use scrutiny_chain_common::error::Error;
    /// 
    /// let err = Error::analysis("model prediction failed");
    /// assert!(matches!(err, Error::Analysis(_)));
    /// ```
    pub fn analysis<T: ToString>(msg: T) -> Self {
        Error::Analysis(msg.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = Error::database("connection failed");
        assert!(matches!(err, Error::Database(_)));
        
        let err = Error::blockchain("invalid block");
        assert!(matches!(err, Error::Blockchain(_)));
    }

    #[test]
    fn test_error_conversion() {
        let io_error = std::io::Error::new(std::io::ErrorKind::Other, "io error");
        // Convert to Box<dyn Error + Send + Sync>
        let err: Error = Error::Other(Box::new(io_error) as Box<dyn std::error::Error + Send + Sync>);
        assert!(matches!(err, Error::Other(_)));
    }
}