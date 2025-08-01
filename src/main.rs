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
            .service(handlers::health_check)
            // Image Processing Endpoints
            .service(handlers::resize_image)
            .service(handlers::rotate_image)
            .service(handlers::crop_image)
            .service(handlers::apply_filter)
            .service(handlers::add_watermark)
            .service(handlers::optimize_image)
            .service(handlers::convert_image)
            // Video Processing Endpoints
            .service(handlers::transcode_video)
            .service(handlers::transcode_audio)
            .service(handlers::extract_audio)
            // AI/ML Endpoints
            .service(handlers::detect_objects)
            .service(handlers::detect_faces)
            .service(handlers::analyze_colors)
            .service(handlers::content_safety)
            .service(handlers::extract_text)
            .service(handlers::classify_scene)
            // Quality Enhancement Endpoints
            .service(handlers::assess_quality)
            .service(handlers::enhance_image)
            .service(handlers::reduce_noise)
            // Effects Endpoints
            .service(handlers::apply_effect)
            .service(handlers::remove_background)
            .service(handlers::style_transfer)
            .service(handlers::stitch_panorama)
            // Metadata Endpoints
            .service(handlers::extract_metadata)
            .service(handlers::analyze_video)
            // Batch Processing Endpoints
            .service(handlers::batch_resize)
            .service(handlers::batch_optimize)
            .service(handlers::batch_convert)
            // Job Status Endpoint
            .service(handlers::get_job_status)
    })
    .bind(addr)?
    .run()
    .await
}

mod handlers;
mod models;
mod error;
mod filters;
mod transformations;
mod image_processing;
mod services;
mod ai;
mod watermark;
mod quality;
mod optimization;
mod metadata;
mod batch;
mod effects; 