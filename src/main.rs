use actix_web::{App, HttpServer, middleware::Logger};
use log::info;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port = env::var("PORT").unwrap_or_else(|_| "8081".to_string());
    let addr = format!("127.0.0.1:{}", port);

    info!("🦀 Starting Photo-Go Media Processing Service on {}", addr);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(health_check)
    })
    .bind(addr)?
    .run()
    .await
}

use actix_web::{HttpResponse, Result};
use serde_json::json;

#[actix_web::get("/health")]
async fn health_check() -> Result<HttpResponse> {
    info!("Health check requested");
    Ok(HttpResponse::Ok().json(json!({
        "status": "healthy",
        "service": "photo-go-media-processing",
        "version": "1.0.0",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
} 