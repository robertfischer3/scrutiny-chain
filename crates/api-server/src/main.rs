use actix_web::{web, App, HttpServer};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod routes;
mod handlers;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .pretty()
        .init();

    info!("Starting Blockchain Security Analysis Platform API server...");

    HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            // Routes will be added here
            .route("/health", web::get().to(|| async { "OK" }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}