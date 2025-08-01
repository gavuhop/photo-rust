use actix_web::{web, HttpResponse, Result};
use crate::models::*;
use crate::error::TranscodeError;
use crate::filters::*;
use crate::transformations::*;
use crate::image_processing::ImageProcessor;
use crate::services::*;
use crate::ai::*;
use crate::watermark::*;
use crate::quality::*;
use crate::optimization::*;
use crate::metadata::*;
use crate::batch::*;
use log::{debug, info, error};
use serde_json::json;
use std::path::Path;
use uuid::Uuid;

#[actix_web::get("/health")]
pub async fn health_check() -> Result<HttpResponse> {
    info!("Health check requested");
    Ok(HttpResponse::Ok().json(json!({
        "status": "healthy",
        "service": "photo-go-media-processing",
        "version": "1.0.0",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
} 