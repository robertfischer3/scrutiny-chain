pub mod error;
pub mod utils;
pub mod calculator;

// Re-export common types
pub use error::Error;
pub use calculator::add;