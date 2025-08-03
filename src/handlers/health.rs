use actix_web::HttpResponse;
use chrono::Utc;
use log::info;

pub async fn health_check() -> HttpResponse {
    info!("Health check endpoint called at {}", Utc::now().to_rfc3339());
    
    HttpResponse::Ok().json(serde_json::json!({
        "status": "service is running",
        "service": "media-processing-service",
        "version": "1.0.0",
        "timestamp": Utc::now().to_rfc3339()
    }))
} 