// api-server/src/main.rs
use actix_web::{web, App, HttpServer, middleware};
use common::logging;
use tracing::{info, Level};
use std::io;

mod routes;
mod handlers;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Initialize logging
    logging::init_logger_with_level(Level::INFO).await;
    
    // Initialize blockchain-core
    blockchain_core::init().await;
    
    // Initialize transaction-analyzer
    transaction_analyzer::init().await;
    
    info!("Starting Blockchain Security Analysis Platform API server...");
    
    // Host and port configuration
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid port number");
    
    info!("Server will listen on {}:{}", host, port);
    
    // Initialize application state
    let app_state = handlers::initialize_state();
    
    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // Add application state
            .app_data(app_state.clone())
            // Enable logger middleware
            .wrap(middleware::Logger::default())
            // Configure routes
            .service(routes::configure_routes())
            // Simple health check at root
            .route("/", web::get().to(|| async { "Blockchain Security Analysis Platform API" }))
    })
    .bind((host, port))?
    .run()
    .await
}