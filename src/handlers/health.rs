use actix_web::HttpResponse;
use chrono::Utc;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "media-processing-service",
        "version": "1.0.0",
        "timestamp": Utc::now().to_rfc3339()
    }))
} 