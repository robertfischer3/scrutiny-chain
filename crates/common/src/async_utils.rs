// common/src/async_utils.rs
use crate::error::{Error, Result};
use std::time::Duration;
use tokio::time::sleep;

/// Retry an async operation with exponential backoff
/// 
/// # Examples
/// 
/// ```
/// use common::async_utils::retry_with_backoff;
/// use common::error::Result;
/// 
/// async fn fallible_operation() -> Result<String> {
///     // Simulate an operation that might fail
///     Ok("success".to_string())
/// }
/// 
/// # tokio_test::block_on(async {
/// let result = retry_with_backoff(
///     || async { fallible_operation().await },
///     3,                    // max retries
///     Duration::from_secs(1) // initial delay
/// ).await;
/// assert!(result.is_ok());
/// # })
/// ```
pub async fn retry_with_backoff<F, Fut, T>(f: F, max_retries: u32, initial_delay: Duration) -> Result<T>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let mut current_try = 0;
    let mut current_delay = initial_delay;

    loop {
        match f().await {
            Ok(value) => return Ok(value),
            Err(e) => {
                current_try += 1;
                if current_try > max_retries {
                    return Err(e);
                }
                sleep(current_delay).await;
                current_delay *= 2; // Exponential backoff
            }
        }
    }
}

/// Run multiple async operations with a timeout
/// 
/// # Examples
/// 
/// ```
/// use common::async_utils::with_timeout;
/// use std::time::Duration;
/// 
/// async fn long_operation() -> String {
///     tokio::time::sleep(Duration::from_millis(50)).await;
///     "completed".to_string()
/// }
/// 
/// # tokio_test::block_on(async {
/// // This should complete successfully
/// let result = with_timeout(
///     Duration::from_secs(1),
///     long_operation()
/// ).await;
/// assert!(result.is_ok());
/// 
/// // This should timeout
/// let result = with_timeout(
///     Duration::from_millis(10),
///     long_operation()
/// ).await;
/// assert!(result.is_err());
/// # })
/// ```
pub async fn with_timeout<F: std::future::Future>(
    timeout: Duration,
    future: F,
) -> Result<F::Output> {
    tokio::time::timeout(timeout, future)
        .await
        .map_err(|_| Error::Internal("Operation timed out".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_retry_with_backoff() {
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();

        let operation = move || {
            let counter = counter_clone.clone();
            async move {
                let current = counter.fetch_add(1, Ordering::SeqCst);
                if current < 2 {
                    Err(Error::Internal("Temporary error".to_string()))
                } else {
                    Ok("success")
                }
            }
        };

        let result = retry_with_backoff(
            operation,
            3,
            Duration::from_millis(10),
        ).await;

        assert!(result.is_ok());
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_timeout() {
        // Test successful completion
        let result = with_timeout(
            Duration::from_millis(100),
            async { 
                sleep(Duration::from_millis(50)).await;
                42
            }
        ).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);

        // Test timeout
        let result = with_timeout(
            Duration::from_millis(50),
            async { 
                sleep(Duration::from_millis(100)).await;
                42
            }
        ).await;
        assert!(result.is_err());
    }
}