// api-server/src/handlers.rs
use actix_web::{web, HttpResponse};
use blockchain_core::models::{Transaction, SmartContract};
use security_analyzer::SecurityAnalyzer;
use transaction_analyzer::processor::TransactionProcessor;
use transaction_analyzer::ml::MLTransactionAnalyzer;
use common::types::{Address, Hash};
use common::error::Result;
use tracing::{info, error, instrument};
use std::sync::Arc;

/// Application state to be shared across handlers
pub struct AppState {
    pub security_analyzer: Arc<SecurityAnalyzer>,
    pub transaction_processor: Arc<TransactionProcessor>,
}

impl AppState {
    /// Create a new application state
    pub fn new() -> Self {
        info!("Initializing application state");
        
        // Create security analyzer
        let security_analyzer = Arc::new(SecurityAnalyzer::new());
        
        // Create and configure transaction processor
        let mut transaction_processor = TransactionProcessor::new();
        transaction_processor.register_analyzer(Arc::new(MLTransactionAnalyzer::new()));
        
        Self {
            security_analyzer,
            transaction_processor: Arc::new(transaction_processor),
        }
    }
}

/// Initialize and return the application state
pub fn initialize_state() -> web::Data<AppState> {
    web::Data::new(AppState::new())
}