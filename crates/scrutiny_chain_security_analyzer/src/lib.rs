pub mod analysis;
pub mod vulnerabilities;

// Re-export main types
pub use analysis::SecurityAnalyzer;
pub use vulnerabilities::VulnerabilityScanner;