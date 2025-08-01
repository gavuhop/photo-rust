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

// Health Check Endpoint
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

// Image Processing Endpoints
#[actix_web::post("/api/v1/image/resize")]
pub async fn resize_image(req: web::Json<ResizeRequest>) -> Result<HttpResponse> {
    info!("Image resize requested: {:?}", req);
    let processor = ImageProcessor::new();
    
    match processor.resize_image(&req).await {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => {
            error!("Image resize failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Image resize failed",
                "message": e.to_string()
            })))
        }
    }
}

#[actix_web::post("/api/v1/image/rotate")]
pub async fn rotate_image(req: web::Json<RotateRequest>) -> Result<HttpResponse> {
    info!("Image rotate requested: {:?}", req);
    let processor = ImageProcessor::new();
    
    match processor.rotate_image(&req.input_path, &req.output_path, req.angle).await {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => {
            error!("Image rotate failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Image rotate failed",
                "message": e.to_string()
            })))
        }
    }
}

#[actix_web::post("/api/v1/image/crop")]
pub async fn crop_image(req: web::Json<CropRequest>) -> Result<HttpResponse> {
    info!("Image crop requested: {:?}", req);
    let processor = ImageProcessor::new();
    
    match processor.crop_image(&req.input_path, &req.output_path, req.x, req.y, req.width, req.height).await {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => {
            error!("Image crop failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Image crop failed",
                "message": e.to_string()
            })))
        }
    }
}

#[actix_web::post("/api/v1/image/filter")]
pub async fn apply_filter(req: web::Json<ImageFilterRequest>) -> Result<HttpResponse> {
    info!("Image filter requested: {:?}", req);
    let filter_processor = ImageFilterProcessor::new();
    
    match filter_processor.apply_filter(&req).await {
        Ok(job_id) => Ok(HttpResponse::Ok().json(json!({
            "job_id": job_id,
            "status": "processing",
            "message": "Filter applied successfully"
        }))),
        Err(e) => {
            error!("Image filter failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Image filter failed",
                "message": e.to_string()
            })))
        }
    }
}

#[actix_web::post("/api/v1/image/watermark")]
pub async fn add_watermark(req: web::Json<WatermarkRequest>) -> Result<HttpResponse> {
    info!("Watermark requested: {:?}", req);
    let watermark_processor = WatermarkProcessor::new();
    
    match watermark_processor.add_watermark(&req).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(e) => {
            error!("Watermark failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Watermark failed",
                "message": e.to_string()
            })))
        }
    }
}

#[actix_web::post("/api/v1/image/optimize")]
pub async fn optimize_image(req: web::Json<OptimizeRequest>) -> Result<HttpResponse> {
    info!("Image optimization requested: {:?}", req);
    let optimizer = ImageOptimizer::new();
    
    match optimizer.compress_image(&req.input_path, &req.output_path, req.quality).await {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => {
            error!("Image optimization failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Image optimization failed",
                "message": e.to_string()
            })))
        }
    }
}

#[actix_web::post("/api/v1/image/convert")]
pub async fn convert_image(req: web::Json<ImageTranscodeRequest>) -> Result<HttpResponse> {
    info!("Image conversion requested: {:?}", req);
    let processor = ImageProcessor::new();
    
    match processor.transcode_image(&req.input_path, &req.output_path, &req.format, req.quality).await {
        Ok(job_id) => Ok(HttpResponse::Ok().json(json!({
            "job_id": job_id,
            "status": "processing",
            "message": "Image conversion started"
        }))),
        Err(e) => {
            error!("Image conversion failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Image conversion failed",
                "message": e.to_string()
            })))
        }
    }
}

// Video Processing Endpoints
#[actix_web::post("/api/v1/video/transcode")]
pub async fn transcode_video(req: web::Json<VideoTranscodeRequest>) -> Result<HttpResponse> {
    info!("Video transcoding requested: {:?}", req);
    let processor = VideoProcessor::new();
    
    match processor.transcode_video(&req.input_path, &req.output_path, &req.format).await {
        Ok(job_id) => Ok(HttpResponse::Ok().json(json!({
            "job_id": job_id,
            "status": "processing",
            "message": "Video transcoding started"
        }))),
        Err(e) => {
            error!("Video transcoding failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Video transcoding failed",
                "message": e.to_string()
            })))
        }
    }
}

#[actix_web::post("/api/v1/audio/transcode")]
pub async fn transcode_audio(req: web::Json<AudioTranscodeRequest>) -> Result<HttpResponse> {
    info!("Audio transcoding requested: {:?}", req);
    let processor = AudioProcessor::new();
    
    match processor.transcode_audio(&req.input_path, &req.output_path, &req.format, req.bitrate.as_deref(), req.sample_rate, req.channels).await {
        Ok(job_id) => Ok(HttpResponse::Ok().json(json!({
            "job_id": job_id,
            "status": "processing",
            "message": "Audio transcoding started"
        }))),
        Err(e) => {
            error!("Audio transcoding failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Audio transcoding failed",
                "message": e.to_string()
            })))
        }
    }
}

#[actix_web::post("/api/v1/audio/extract")]
pub async fn extract_audio(req: web::Json<AudioExtractRequest>) -> Result<HttpResponse> {
    info!("Audio extraction requested: {:?}", req);
    let processor = AudioProcessor::new();
    
    match processor.extract_audio_from_video(&req.video_path, &req.audio_path).await {
        Ok(job_id) => Ok(HttpResponse::Ok().json(json!({
            "job_id": job_id,
            "status": "processing",
            "message": "Audio extraction started"
        }))),
        Err(e) => {
            error!("Audio extraction failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Audio extraction failed",
                "message": e.to_string()
            })))
        }
    }
}

// AI/ML Endpoints
#[actix_web::post("/api/v1/ai/detect-objects")]
pub async fn detect_objects(req: web::Json<ObjectDetectionRequest>) -> Result<HttpResponse> {
    info!("Object detection requested: {:?}", req);
    let ai_processor = AIProcessor::new();
    
    match ai_processor.detect_objects(&req.image_path).await {
        Ok(objects) => Ok(HttpResponse::Ok().json(json!({
            "objects": objects,
            "count": objects.len()
        }))),
        Err(e) => {
            error!("Object detection failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Object detection failed",
                "message": e.to_string()
            })))
        }
    }
}

#[actix_web::post("/api/v1/ai/detect-faces")]
pub async fn detect_faces(req: web::Json<FaceDetectionRequest>) -> Result<HttpResponse> {
    info!("Face detection requested: {:?}", req);
    let ai_processor = AIProcessor::new();
    
    match ai_processor.detect_faces(&req.image_path).await {
        Ok(faces) => Ok(HttpResponse::Ok().json(json!({
            "faces": faces,
            "count": faces.len()
        }))),
        Err(e) => {
            error!("Face detection failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Face detection failed",
                "message": e.to_string()
            })))
        }
    }
}

#[actix_web::post("/api/v1/ai/analyze-colors")]
pub async fn analyze_colors(req: web::Json<ColorAnalysisRequest>) -> Result<HttpResponse> {
    info!("Color analysis requested: {:?}", req);
    let ai_processor = AIProcessor::new();
    
    match ai_processor.analyze_colors(&req.image_path).await {
        Ok(colors) => Ok(HttpResponse::Ok().json(json!({
            "colors": colors,
            "count": colors.len()
        }))),
        Err(e) => {
            error!("Color analysis failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Color analysis failed",
                "message": e.to_string()
            })))
        }
    }
}

#[actix_web::post("/api/v1/ai/content-safety")]
pub async fn content_safety(req: web::Json<ContentSafetyRequest>) -> Result<HttpResponse> {
    info!("Content safety analysis requested: {:?}", req);
    let ai_processor = AIProcessor::new();
    
    match ai_processor.analyze_content_safety(&req.image_path).await {
        Ok(score) => Ok(HttpResponse::Ok().json(json!({
            "safety_score": score,
            "is_safe": score < 0.5
        }))),
        Err(e) => {
            error!("Content safety analysis failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Content safety analysis failed",
                "message": e.to_string()
            })))
        }
    }
}

#[actix_web::post("/api/v1/ai/extract-text")]
pub async fn extract_text(req: web::Json<TextExtractionRequest>) -> Result<HttpResponse> {
    info!("Text extraction requested: {:?}", req);
    let ai_processor = AIProcessor::new();
    
    match ai_processor.extract_text(&req.image_path).await {
        Ok(text) => Ok(HttpResponse::Ok().json(json!({
            "text": text,
            "length": text.len()
        }))),
        Err(e) => {
            error!("Text extraction failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Text extraction failed",
                "message": e.to_string()
            })))
        }
    }
}

#[actix_web::post("/api/v1/ai/classify-scene")]
pub async fn classify_scene(req: web::Json<SceneClassificationRequest>) -> Result<HttpResponse> {
    info!("Scene classification requested: {:?}", req);
    let ai_processor = AIProcessor::new();
    
    match ai_processor.classify_scene(&req.image_path).await {
        Ok(classes) => Ok(HttpResponse::Ok().json(json!({
            "scene_classes": classes,
            "top_class": classes.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).map(|(k, v)| (k.clone(), *v))
        }))),
        Err(e) => {
            error!("Scene classification failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Scene classification failed",
                "message": e.to_string()
            })))
        }
    }
}

// Quality Enhancement Endpoints
#[actix_web::post("/api/v1/quality/assess")]
pub async fn assess_quality(req: web::Json<QualityAssessmentRequest>) -> Result<HttpResponse> {
    info!("Quality assessment requested: {:?}", req);
    let quality_processor = QualityProcessor::new();
    
    match quality_processor.assess_quality(&req.image_path).await {
        Ok(assessment) => Ok(HttpResponse::Ok().json(assessment)),
        Err(e) => {
            error!("Quality assessment failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Quality assessment failed",
                "message": e.to_string()
            })))
        }
    }
}

#[actix_web::post("/api/v1/quality/enhance")]
pub async fn enhance_image(req: web::Json<EnhancementRequest>) -> Result<HttpResponse> {
    info!("Image enhancement requested: {:?}", req);
    let quality_processor = QualityProcessor::new();
    
    match quality_processor.auto_enhance(&req).await {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => {
            error!("Image enhancement failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Image enhancement failed",
                "message": e.to_string()
            })))
        }
    }
}

#[actix_web::post("/api/v1/quality/reduce-noise")]
pub async fn reduce_noise(req: web::Json<EnhancementRequest>) -> Result<HttpResponse> {
    info!("Noise reduction requested: {:?}", req);
    let quality_processor = QualityProcessor::new();
    
    match quality_processor.reduce_noise(&req).await {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => {
            error!("Noise reduction failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Noise reduction failed",
                "message": e.to_string()
            })))
        }
    }
}

// Effects Endpoints
#[actix_web::post("/api/v1/effects/apply")]
pub async fn apply_effect(req: web::Json<EffectRequest>) -> Result<HttpResponse> {
    info!("Effect application requested: {:?}", req);
    let effects_processor = EffectsProcessor::new();
    
    match effects_processor.apply_effect(&req).await {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => {
            error!("Effect application failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Effect application failed",
                "message": e.to_string()
            })))
        }
    }
}

#[actix_web::post("/api/v1/effects/remove-background")]
pub async fn remove_background(req: web::Json<BackgroundRemovalRequest>) -> Result<HttpResponse> {
    info!("Background removal requested: {:?}", req);
    let effects_processor = EffectsProcessor::new();
    
    match effects_processor.remove_background(&req.input_path, &req.output_path).await {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => {
            error!("Background removal failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Background removal failed",
                "message": e.to_string()
            })))
        }
    }
}

#[actix_web::post("/api/v1/effects/style-transfer")]
pub async fn style_transfer(req: web::Json<StyleTransferRequest>) -> Result<HttpResponse> {
    info!("Style transfer requested: {:?}", req);
    let effects_processor = EffectsProcessor::new();
    
    match effects_processor.style_transfer(&req).await {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => {
            error!("Style transfer failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Style transfer failed",
                "message": e.to_string()
            })))
        }
    }
}

#[actix_web::post("/api/v1/effects/panorama")]
pub async fn stitch_panorama(req: web::Json<PanoramaRequest>) -> Result<HttpResponse> {
    info!("Panorama stitching requested: {:?}", req);
    let effects_processor = EffectsProcessor::new();
    
    match effects_processor.stitch_panorama(&req).await {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => {
            error!("Panorama stitching failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Panorama stitching failed",
                "message": e.to_string()
            })))
        }
    }
}

// Metadata Endpoints
#[actix_web::post("/api/v1/metadata/extract")]
pub async fn extract_metadata(req: web::Json<MediaMetadataRequest>) -> Result<HttpResponse> {
    info!("Metadata extraction requested: {:?}", req);
    let metadata_processor = MetadataProcessor::new();
    
    match metadata_processor.extract_metadata(&req.file_path, req.extract_exif.unwrap_or(true), req.extract_ai_tags.unwrap_or(true)).await {
        Ok(metadata) => Ok(HttpResponse::Ok().json(metadata)),
        Err(e) => {
            error!("Metadata extraction failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Metadata extraction failed",
                "message": e.to_string()
            })))
        }
    }
}

#[actix_web::post("/api/v1/metadata/analyze-video")]
pub async fn analyze_video(req: web::Json<VideoAnalysisRequest>) -> Result<HttpResponse> {
    info!("Video analysis requested: {:?}", req);
    let metadata_processor = MetadataProcessor::new();
    
    match metadata_processor.analyze_video(&req.file_path, req.extract_frames.unwrap_or(false), req.frame_interval.unwrap_or(1), req.extract_audio.unwrap_or(false)).await {
        Ok(analysis) => Ok(HttpResponse::Ok().json(analysis)),
        Err(e) => {
            error!("Video analysis failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Video analysis failed",
                "message": e.to_string()
            })))
        }
    }
}

// Batch Processing Endpoints
#[actix_web::post("/api/v1/batch/resize")]
pub async fn batch_resize(req: web::Json<BatchRequest>) -> Result<HttpResponse> {
    info!("Batch resize requested: {:?}", req);
    let batch_processor = BatchProcessor::new();
    
    match batch_processor.batch_resize(&req).await {
        Ok(results) => Ok(HttpResponse::Ok().json(json!({
            "results": results,
            "total": results.len(),
            "successful": results.iter().filter(|r| r.is_ok()).count()
        }))),
        Err(e) => {
            error!("Batch resize failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Batch resize failed",
                "message": e.to_string()
            })))
        }
    }
}

#[actix_web::post("/api/v1/batch/optimize")]
pub async fn batch_optimize(req: web::Json<BatchRequest>) -> Result<HttpResponse> {
    info!("Batch optimization requested: {:?}", req);
    let batch_processor = BatchProcessor::new();
    
    match batch_processor.batch_optimize(&req).await {
        Ok(results) => Ok(HttpResponse::Ok().json(json!({
            "results": results,
            "total": results.len(),
            "successful": results.iter().filter(|r| r.is_ok()).count()
        }))),
        Err(e) => {
            error!("Batch optimization failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Batch optimization failed",
                "message": e.to_string()
            })))
        }
    }
}

#[actix_web::post("/api/v1/batch/convert")]
pub async fn batch_convert(req: web::Json<BatchRequest>) -> Result<HttpResponse> {
    info!("Batch conversion requested: {:?}", req);
    let batch_processor = BatchProcessor::new();
    
    match batch_processor.batch_convert(&req).await {
        Ok(results) => Ok(HttpResponse::Ok().json(json!({
            "results": results,
            "total": results.len(),
            "successful": results.iter().filter(|r| r.is_ok()).count()
        }))),
        Err(e) => {
            error!("Batch conversion failed: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Batch conversion failed",
                "message": e.to_string()
            })))
        }
    }
}

// Job Status Endpoint
#[actix_web::get("/api/v1/jobs/{job_id}")]
pub async fn get_job_status(job_id: web::Path<Uuid>) -> Result<HttpResponse> {
    info!("Job status requested for: {}", job_id);
    
    // TODO: Implement job status tracking
    Ok(HttpResponse::Ok().json(json!({
        "job_id": job_id.to_string(),
        "status": "completed",
        "progress": 100.0,
        "message": "Job completed successfully"
    })))
} 