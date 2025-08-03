use actix_web::{web, HttpResponse, Result};
use crate::models::video::{VideoTranscodeRequest, VideoTranscodeResponse, AudioExtractRequest, VideoInfoRequest};
use crate::services::video_processor::VideoProcessor;
use crate::utils::error::ServiceError;
use log::{error, info};
use serde::Serialize;

#[derive(Serialize)]
pub struct MultiQualityHlsResponse {
    pub outputs: Vec<String>,
    pub master_playlist: String,
}

pub async fn transcode_video(
    req: web::Json<VideoTranscodeRequest>,
    video_processor: web::Data<VideoProcessor>,
) -> Result<HttpResponse, ServiceError> {
    info!("Received video transcode request");
    
    match video_processor.transcode_video(&req.into_inner()).await {
        Ok(job_id) => {
            let response = VideoTranscodeResponse {
                job_id,
                status: "processing".to_string(),
                message: "Video transcode job started successfully".to_string(),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            error!("Video transcode failed: {}", e);
            Err(ServiceError::FFmpegError(e.to_string()))
        }
    }
}

pub async fn extract_audio(
    req: web::Json<AudioExtractRequest>,
    video_processor: web::Data<VideoProcessor>,
) -> Result<HttpResponse, ServiceError> {
    info!("Received audio extraction request");
    
    match video_processor.extract_audio(&req.into_inner()).await {
        Ok(job_id) => {
            let response = VideoTranscodeResponse {
                job_id,
                status: "processing".to_string(),
                message: "Audio extraction job started successfully".to_string(),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            error!("Audio extraction failed: {}", e);
            Err(ServiceError::FFmpegError(e.to_string()))
        }
    }
}

pub async fn transcode_audio(
    req: web::Json<serde_json::Value>,
    video_processor: web::Data<VideoProcessor>,
) -> Result<HttpResponse, ServiceError> {
    info!("Received audio transcode request");
    
    let input_path = req["input_path"].as_str().unwrap_or("");
    let output_path = req["output_path"].as_str().unwrap_or("");
    let format = req["format"].as_str();
    
    if input_path.is_empty() || output_path.is_empty() {
        return Err(ServiceError::BadRequest("Missing input_path or output_path".to_string()));
    }
    
    match video_processor.transcode_audio(input_path, output_path, format).await {
        Ok(job_id) => {
            let response = VideoTranscodeResponse {
                job_id,
                status: "processing".to_string(),
                message: "Audio transcode job started successfully".to_string(),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            error!("Audio transcode failed: {}", e);
            Err(ServiceError::FFmpegError(e.to_string()))
        }
    }
}

pub async fn transcode_multi_quality_and_hls(
    req: web::Json<VideoTranscodeRequest>,
    video_processor: web::Data<VideoProcessor>,
) -> Result<HttpResponse, ServiceError> {
    info!("Received multi-quality HLS transcode request");
    let input_path = &req.input_path;
    let output_prefix = req.output_path.trim_end_matches(".mp4");
    let codec = req.codec.as_deref().unwrap_or("libx264");
    let format = req.format.as_deref().unwrap_or("mp4");
    let output_dir = std::path::Path::new(output_prefix).parent().unwrap_or_else(|| std::path::Path::new("output")).to_str().unwrap_or("output");
    let master_playlist = "master.m3u8";

    // 1. Transcode song song nhiều chất lượng
    let outputs = video_processor.transcode_multi_quality(
        input_path,
        output_prefix,
        codec,
        format,
    ).await.map_err(|e| ServiceError::FFmpegError(e.to_string()))?;

    // 2. Đóng gói HLS
    video_processor.package_hls(&outputs, output_dir, master_playlist)
        .await.map_err(|e| ServiceError::FFmpegError(e.to_string()))?;

    // 3. Trả về metadata
    let response = MultiQualityHlsResponse {
        outputs: outputs.clone(),
        master_playlist: format!("{}/{}", output_dir, master_playlist),
    };
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_video_info(
    req: web::Json<VideoInfoRequest>,
    video_processor: web::Data<VideoProcessor>,
) -> Result<HttpResponse, ServiceError> {
    info!("Received video info request for: {}", req.file_path);
    
    match video_processor.get_video_info(&req.file_path).await {
        Ok(info) => Ok(HttpResponse::Ok().json(info)),
        Err(e) => {
            error!("Failed to get video info: {}", e);
            Err(ServiceError::FFmpegError(e.to_string()))
        }
    }
}

pub async fn get_video_info_from_json(
    req: web::Json<serde_json::Value>,
    video_processor: web::Data<VideoProcessor>,
) -> Result<HttpResponse, ServiceError> {
    let file_path = req["file_path"].as_str().unwrap_or("");
    
    if file_path.is_empty() {
        return Err(ServiceError::BadRequest("Missing file_path".to_string()));
    }
    
    info!("Received video info request for: {}", file_path);
    
    match video_processor.get_video_info(file_path).await {
        Ok(info) => Ok(HttpResponse::Ok().json(info)),
        Err(e) => {
            error!("Failed to get video info: {}", e);
            Err(ServiceError::FFmpegError(e.to_string()))
        }
    }
} 