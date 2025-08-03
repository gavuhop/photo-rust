mod handlers;
mod services;
mod models;
mod utils;
mod logging;

use actix_web::{web, App, HttpServer};
use log::info;
use services::video_processor::VideoProcessor;
use logging::{init_logger, levels};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize custom logger
    let log_dir = std::env::var("LOG_DIR").unwrap_or_else(|_| "logs".to_string());
    let log_level = if cfg!(debug_assertions) {
        levels::DEVELOPMENT
    } else {
        levels::PRODUCTION
    };
    
    init_logger(&log_dir, log_level)?;
    
    info!("Starting Media Processing Service...");
    
    // Initialize video processor
    let video_processor = VideoProcessor::new()
        .expect("Failed to initialize video processor");
    
    let video_processor_data = web::Data::new(video_processor);
    
    let port = std::env::var("PORT").unwrap_or_else(|_| "8082".to_string());
    let bind_address = format!("127.0.0.1:{}", port);
    
    info!("Server starting on {}", bind_address);
    
    HttpServer::new(move || {
        App::new()
            .app_data(video_processor_data.clone())
            .service(
                web::scope("/api/v1")
                    .service(
                        web::scope("/video")
                            .route("/transcode", web::post().to(handlers::video::transcode_video))
                            .route("/extract-audio", web::post().to(handlers::video::extract_audio))
                            .route("/info", web::post().to(handlers::video::get_video_info))
                    )
                    .service(
                        web::scope("/audio")
                            .route("/transcode", web::post().to(handlers::video::transcode_audio))
                            .route("/extract", web::post().to(handlers::video::extract_audio))
                    )
                    .service(
                        web::scope("/metadata")
                            .route("/extract", web::post().to(handlers::video::get_video_info_from_json))
                    )
            )
            .route("/health", web::get().to(handlers::health::health_check))
    })
    .bind(&bind_address)?
    .run()
    .await
} 