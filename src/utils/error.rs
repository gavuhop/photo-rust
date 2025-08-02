use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;

#[allow(dead_code)]
#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalError,
    
    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),
    
    #[display(fmt = "FFmpeg Error: {}", _0)]
    FFmpegError(String),
    
    #[display(fmt = "File Not Found: {}", _0)]
    FileNotFound(String),
    
    #[display(fmt = "Invalid Format: {}", _0)]
    InvalidFormat(String),
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalError => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Internal Server Error"
                }))
            }
            ServiceError::BadRequest(ref message) => {
                HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Bad Request",
                    "message": message
                }))
            }
            ServiceError::FFmpegError(ref message) => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "FFmpeg Processing Error",
                    "message": message
                }))
            }
            ServiceError::FileNotFound(ref message) => {
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": "File Not Found",
                    "message": message
                }))
            }
            ServiceError::InvalidFormat(ref message) => {
                HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Invalid Format",
                    "message": message
                }))
            }
        }
    }
} 