use blockchain_core::init;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging for the application
    init().await;
    
    info!("Blockchain Core service starting up...");

    // Add any standalone functionality or service initialization here
    
    info!("Blockchain Core service ready");

    // Keep the service running
    tokio::signal::ctrl_c().await?;
    info!("Shutting down Blockchain Core service");
    
    Ok(())
}