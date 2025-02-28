// common/src/logging.rs
use std::sync::Once;
use tokio::sync::OnceCell;
use tracing::Level;
use tracing_subscriber::{
    fmt::format::FmtSpan,
    EnvFilter,
};

static ASYNC_LOGGER: OnceCell<()> = OnceCell::const_new();

/// Initialize the global logger with sensible defaults asynchronously
/// 
/// This sets up logging with:
/// - INFO level by default
/// - Console output
/// - Thread IDs
/// - File and line numbers
/// - Full span events
/// 
/// # Examples
/// 
/// ```
/// use scrutiny_chain_common::logging::init_logger;
/// use tracing::info;
/// 
/// # tokio_test::block_on(async {
/// // Initialize the default logger
/// init_logger().await;
/// 
/// // Log some information
/// info!("Application started");
/// # })
/// ```
pub async fn init_logger() {
    init_logger_with_level(Level::INFO).await;
}

/// Initialize the global logger with a specific level asynchronously
/// 
/// # Examples
/// 
/// ```
/// use scrutiny_chain_common::logging::init_logger_with_level;
/// use tracing::{debug, Level};
/// 
/// # tokio_test::block_on(async {
/// // Initialize logger with debug level
/// init_logger_with_level(Level::DEBUG).await;
/// 
/// // Now debug logs will be visible
/// debug!("Detailed debug information");
/// # })
/// ```
pub async fn init_logger_with_level(level: Level) {
    ASYNC_LOGGER.get_or_init(|| async {
        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new(format!("common={}", level.as_str())));

            tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .with_span_events(FmtSpan::FULL)
            .with_target(true)
            .with_thread_ids(true)
            .with_line_number(true)
            .with_file(true)
            .init();
    }).await;
}

/// Initialize a JSON logger for production environments asynchronously
/// 
/// This sets up structured logging in JSON format, which is useful for:
/// - Log aggregation systems
/// - Cloud logging platforms
/// - Production environments where machine-readable logs are needed
/// 
/// # Examples
/// 
/// ```
/// use scrutiny_chain_common::logging::init_json_logger;
/// use tracing::{info, warn};
/// 
/// # tokio_test::block_on(async {
/// // Initialize the JSON logger
/// init_json_logger().await;
/// 
/// // Log events will now be output in JSON format
/// info!("System status nominal");
/// warn!(error_code = 123, "Resource usage high");
/// # })
/// ```
pub async fn init_json_logger() {
    ASYNC_LOGGER.get_or_init(|| async {
        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"));

            tracing_subscriber::fmt()
            .json()
            .with_env_filter(env_filter)
            .with_span_events(FmtSpan::FULL)
            .with_target(true)
            .with_thread_ids(true)
            .with_line_number(true)
            .with_file(true)
            .init();
    }).await;
}

/// Create an async timing span for measuring operation duration
/// 
/// # Examples
/// 
/// ```
/// use scrutiny_chain_common::logging::create_timing_span;
/// use tracing::info;
/// use std::time::Duration;
/// 
/// # tokio_test::block_on(async {
/// // Create a span for timing a database operation
/// let span = create_timing_span("database_operation", "query_users");
/// let _entered = span.entered();
/// 
/// // Simulate some async work
/// tokio::time::sleep(Duration::from_millis(100)).await;
/// info!("Querying users table");
/// 
/// // Span will automatically close when _entered is dropped
/// # })
/// ```
pub fn create_timing_span(category: &str, operation: &str) -> tracing::Span {
    tracing::info_span!(
        "timing",
        category = category,
        operation = operation,
        start_time = format!("{:?}", std::time::Instant::now())
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;
    use std::time::Duration;

    #[tokio::test]
    async fn test_logger_initialization() {
        // Initialize logger with debug level
        init_logger_with_level(Level::DEBUG).await;
        
        // Test logging
        info!("Test log message");
    }

    #[tokio::test]
    async fn test_json_logger() {
        // Initialize JSON logger
        init_json_logger().await;
        
        // Test structured logging
        info!(
            target: "test",
            event = "test_event",
            value = 42,
            "Test JSON logging"
        );
    }

    #[tokio::test]
    async fn test_timing_span() {
        init_logger().await;
        let span = create_timing_span("test", "operation");
        let _entered = span.entered();
        
        // Simulate some async work
        tokio::time::sleep(Duration::from_millis(10)).await;
        info!("Operation in progress");
    }
}