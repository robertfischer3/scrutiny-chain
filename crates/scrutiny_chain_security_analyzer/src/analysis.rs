// security-analyzer/src/analysis.rs
use crate::vulnerabilities::VulnerabilityScanner;
use scrutiny_chain_common::{
    error::{Error, Result},
    types::{Address, RiskLevel},
    logging::create_timing_span,
};
use tracing::{debug, error, info, warn, instrument};
use std::collections::HashMap;
use scrutiny_chain_blockchain_core::models::{SecurityAnalysis, SmartContract};

/// Represents a security analyzer that can scan smart contracts for vulnerabilities
/// 
/// The SecurityAnalyzer uses multiple vulnerability scanners to perform comprehensive
/// security analysis of smart contracts. It includes logging, error handling, and
/// detailed reporting capabilities.
/// 
/// # Examples
/// 
/// use security_analyzer::analysis::SecurityAnalyzer;
/// use scrutiny_chain_common::types::Address;
/// 
/// #tokio_test::block_on(async {
/// let analyzer = SecurityAnalyzer::new();
/// 
/// // Analyze a contract
/// let analysis = analyzer.analyze_contract(
///     &Address("0x742d35Cc6634C0532925a3b844Bc454e4438f44e".to_string())
/// ).await;
/// 
/// match analysis {
///     Ok(report) => {
///         println!("Risk Level: {}", report.risk_level);
///         println!("Findings: {:?}", report.findings);
///     }
///     Err(e) => println!("Analysis failed: {}", e),
/// #}
/// # })
pub struct SecurityAnalyzer {
    scanners: Vec<Box<dyn VulnerabilityScanner>>,
}

impl SecurityAnalyzer {
    /// Creates a new SecurityAnalyzer instance
    pub fn new() -> Self {
        info!("Initializing SecurityAnalyzer");
        Self {
            scanners: Vec::new(),
        }
    }

    /// Registers a new vulnerability scanner
    /// 
    /// # Examples
    /// 
    /// ```
    /// use scrutiny_chain_security_analyzer::analysis::SecurityAnalyzer;
    /// use scrutiny_chain_security_analyzer::vulnerabilities::VulnerabilityScanner;
    /// use scrutiny_chain_common::error::Result;
    /// use async_trait::async_trait;
    /// use scrutiny_chain_common::types::Address;
    /// 
    /// struct MockScanner;
    /// 
    /// #[async_trait]
    /// impl VulnerabilityScanner for MockScanner {
    ///     async fn scan(&self, _address: &Address) -> Result<Vec<String>> {
    ///         Ok(vec!["No vulnerabilities found".to_string()])
    ///     }
    /// }
    /// 
    /// let mut analyzer = SecurityAnalyzer::new();
    /// analyzer.register_scanner(Box::new(MockScanner));
    /// ```
    pub fn register_scanner(&mut self, scanner: Box<dyn VulnerabilityScanner>) {
        debug!("Registering new vulnerability scanner");
        self.scanners.push(scanner);
    }

    /// Analyzes a smart contract for security vulnerabilities
    /// 
    /// This method runs all registered vulnerability scanners against the target
    /// contract and aggregates their findings into a comprehensive security report.
    /// 
    /// # Arguments
    /// 
    /// * `address` - The address of the smart contract to analyze
    /// 
    /// # Returns
    /// 
    /// Returns a Result containing a SecurityAnalysis if successful, or an Error
    /// if the analysis fails.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use scrutiny_chain_security_analyzer::analysis::SecurityAnalyzer;
    /// use scrutiny_chain_common::types::Address;
    /// use scrutiny_chain_common::error::Result;
    /// 
    /// async fn analyze_my_contract() -> Result<()> {
    ///     let analyzer = SecurityAnalyzer::new();
    ///     
    ///     let address = Address("0x742d35Cc6634C0532925a3b844Bc454e4438f44e".to_string());
    ///     let analysis = analyzer.analyze_contract(&address).await?;
    ///     
    ///     println!("Analysis complete with risk level: {}", analysis.risk_level);
    ///     Ok(())
    /// }
    /// ```
    #[instrument(skip(self), level = "debug", err)]
    pub async fn analyze_contract(&self, address: &Address) -> Result<SecurityAnalysis> {
        let _timing_span = create_timing_span("security_analysis", "contract_scan");
        info!("Starting security analysis for contract {}", address);

        let mut metadata = HashMap::new();
    
        metadata.insert(
            "scan_timestamp".to_string(),
            scrutiny_chain_common::utils::current_timestamp().to_string()
        );
        metadata.insert("scanner_count".to_string(), self.scanners.len().to_string());
        
        if self.scanners.is_empty() {
            warn!("No vulnerability scanners registered");
            return Ok(SecurityAnalysis {
                risk_level: RiskLevel::None,
                findings: vec!["No security scanners configured".to_string()],
                metadata,
            });
        }

        let mut findings = Vec::new();
        let mut highest_risk = RiskLevel::None;
        let mut metadata = HashMap::new();

        for (i, scanner) in self.scanners.iter().enumerate() {
            match scanner.scan(address).await {
                Ok(scanner_findings) => {
                    debug!(
                        "Scanner {} completed with {} findings",
                        i,
                        scanner_findings.len()
                    );
                    findings.extend(scanner_findings);
                }
                Err(e) => {
                    error!("Scanner {} failed: {}", i, e);
                    return Err(Error::analysis(format!(
                        "Vulnerability scanner {} failed: {}",
                        i, e
                    )));
                }
            }
        }

        // Determine overall risk level based on findings
        highest_risk = self.calculate_risk_level(&findings);
        
        metadata.insert(
            "scan_timestamp".to_string(),
            scrutiny_chain_common::utils::current_timestamp().to_string(),
        );
        metadata.insert(
            "scanner_count".to_string(),
            self.scanners.len().to_string(),
        );

        info!(
            "Analysis complete for contract {}. Risk Level: {}",
            address, highest_risk
        );

        Ok(SecurityAnalysis {
            risk_level: highest_risk,
            findings,
            metadata,
        })
    }

    /// Calculates the overall risk level based on the findings
    fn calculate_risk_level(&self, findings: &[String]) -> RiskLevel {
        let critical_keywords = ["critical", "severe", "high risk"];
        let high_keywords = ["high", "major", "significant"];
        let medium_keywords = ["medium", "moderate"];
        let low_keywords = ["low", "minor", "info"];

        let mut highest_risk = RiskLevel::None;

        for finding in findings {
            let finding_lower = finding.to_lowercase();
            let risk_level = if critical_keywords.iter().any(|&k| finding_lower.contains(k)) {
                RiskLevel::Critical
            } else if high_keywords.iter().any(|&k| finding_lower.contains(k)) {
                RiskLevel::High
            } else if medium_keywords.iter().any(|&k| finding_lower.contains(k)) {
                RiskLevel::Medium
            } else if low_keywords.iter().any(|&k| finding_lower.contains(k)) {
                RiskLevel::Low
            } else {
                RiskLevel::None
            };

            if risk_level > highest_risk {
                highest_risk = risk_level;
            }
        }

        highest_risk
    }
}

impl Default for SecurityAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct MockScanner {
        findings: Vec<String>,
    }

    #[async_trait]
    impl VulnerabilityScanner for MockScanner {
        async fn scan(&self, _address: &Address) -> Result<Vec<String>> {
            Ok(self.findings.clone())
        }
    }

    #[tokio::test]
    async fn test_empty_analyzer() {
        let analyzer = SecurityAnalyzer::new();
        let address = Address("0x123".to_string());

        let analysis = analyzer.analyze_contract(&address).await.unwrap();
        assert_eq!(analysis.risk_level, RiskLevel::None);
        assert!(analysis.findings.contains(&"No security scanners configured".to_string()));
    }

    #[tokio::test]
    async fn test_risk_level_calculation() {
        let mut analyzer = SecurityAnalyzer::new();
        
        // Add mock scanners with different risk levels
        analyzer.register_scanner(Box::new(MockScanner {
            findings: vec![
                "Critical vulnerability found".to_string(),
                "Low risk issue detected".to_string(),
            ],
        }));

        let address = Address("0x123".to_string());
        let analysis = analyzer.analyze_contract(&address).await.unwrap();
        
        assert_eq!(analysis.risk_level, RiskLevel::Critical);
        assert_eq!(analysis.findings.len(), 2);
    }

    #[tokio::test]
    async fn test_metadata_generation() {
        let analyzer = SecurityAnalyzer::new();
        let address = Address("0x123".to_string());

        let analysis = analyzer.analyze_contract(&address).await.unwrap();
        
        assert!(analysis.metadata.contains_key("scan_timestamp"));
        assert!(analysis.metadata.contains_key("scanner_count"));
    }
}