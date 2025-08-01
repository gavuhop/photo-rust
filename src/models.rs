use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct TranscodeRequest {
    pub input_path: String,
    pub output_path: String,
    pub format: String,
    pub quality: Option<String>,
    pub resolution: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoTranscodeRequest {
    pub input_path: String,
    pub output_path: String,
    pub format: String,
    pub codec: Option<String>,
    pub bitrate: Option<String>,
    pub resolution: Option<String>,
    pub fps: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageTranscodeRequest {
    pub input_path: String,
    pub output_path: String,
    pub format: String,
    pub quality: Option<u8>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub resize_mode: Option<String>, // "fit", "crop", "fill"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranscodeJob {
    pub id: Uuid,
    pub status: TranscodeStatus,
    pub input_path: String,
    pub output_path: String,
    pub format: String,
    pub progress: f32,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TranscodeStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranscodeResponse {
    pub job_id: Uuid,
    pub status: TranscodeStatus,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioTranscodeRequest {
    pub input_path: String,
    pub output_path: String,
    pub format: String,
    pub bitrate: Option<String>,
    pub sample_rate: Option<u32>,
    pub channels: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaMetadataRequest {
    pub file_path: String,
    pub extract_exif: Option<bool>,
    pub extract_ai_tags: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataResponse {
    pub file_info: FileInfo,
    pub exif_data: Option<ExifData>,
    pub ai_analysis: Option<AIAnalysis>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration: Option<f64>,
    pub format: String,
    pub size: u64,
    pub created: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExifData {
    pub camera_make: Option<String>,
    pub camera_model: Option<String>,
    pub lens: Option<String>,
    pub focal_length: Option<f64>,
    pub aperture: Option<f64>,
    pub shutter_speed: Option<String>,
    pub iso: Option<u32>,
    pub flash: Option<bool>,
    pub date_taken: Option<DateTime<Utc>>,
    pub gps_latitude: Option<f64>,
    pub gps_longitude: Option<f64>,
    pub gps_altitude: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AIAnalysis {
    pub objects: Vec<DetectedObject>,
    pub faces: Vec<DetectedFace>,
    pub colors: Vec<DominantColor>,
    pub tags: Vec<String>,
    pub adult_content_score: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectedObject {
    pub name: String,
    pub confidence: f32,
    pub bounding_box: BoundingBox,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectedFace {
    pub confidence: f32,
    pub bounding_box: BoundingBox,
    pub age_range: Option<(u8, u8)>,
    pub gender: Option<String>,
    pub emotions: HashMap<String, f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BoundingBox {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DominantColor {
    pub hex: String,
    pub rgb: (u8, u8, u8),
    pub percentage: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageFilterRequest {
    pub input_path: String,
    pub output_path: String,
    pub filter_type: String, // "blur", "sharpen", "sepia", "grayscale", etc.
    pub intensity: Option<f32>,
    pub parameters: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WatermarkRequest {
    pub input_path: String,
    pub output_path: String,
    pub watermark_path: String,
    pub position: String, // "top-left", "center", "bottom-right", etc.
    pub opacity: Option<f32>,
    pub scale: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchProcessRequest {
    pub input_paths: Vec<String>,
    pub output_directory: String,
    pub operations: Vec<ProcessOperation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessOperation {
    pub operation_type: String,
    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoAnalysisRequest {
    pub file_path: String,
    pub extract_frames: Option<bool>,
    pub frame_interval: Option<u32>, // seconds
    pub extract_audio: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoAnalysisResponse {
    pub duration: f64,
    pub width: u32,
    pub height: u32,
    pub frame_rate: f64,
    pub codec: String,
    pub bitrate: u64,
    pub frames: Option<Vec<String>>, // paths to extracted frames
    pub audio_path: Option<String>,
}

// New request/response models for the HTTP endpoints

#[derive(Debug, Serialize, Deserialize)]
pub struct ResizeRequest {
    pub input_path: String,
    pub output_path: String,
    pub width: u32,
    pub height: u32,
    pub mode: Option<String>, // "fit", "crop", "fill"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RotateRequest {
    pub input_path: String,
    pub output_path: String,
    pub angle: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CropRequest {
    pub input_path: String,
    pub output_path: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OptimizeRequest {
    pub input_path: String,
    pub output_path: String,
    pub quality: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioExtractRequest {
    pub video_path: String,
    pub audio_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectDetectionRequest {
    pub image_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FaceDetectionRequest {
    pub image_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ColorAnalysisRequest {
    pub image_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentSafetyRequest {
    pub image_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextExtractionRequest {
    pub image_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneClassificationRequest {
    pub image_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityAssessmentRequest {
    pub image_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnhancementRequest {
    pub input_path: String,
    pub output_path: String,
    pub enhancement_type: Option<String>, // "auto", "noise_reduction", "super_resolution", "color_correction"
    pub parameters: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EffectRequest {
    pub input_path: String,
    pub output_path: String,
    pub effect_type: String, // "artistic", "style_transfer", "background_removal"
    pub parameters: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackgroundRemovalRequest {
    pub input_path: String,
    pub output_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StyleTransferRequest {
    pub input_path: String,
    pub output_path: String,
    pub style_path: String,
    pub intensity: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PanoramaRequest {
    pub input_paths: Vec<String>,
    pub output_path: String,
    pub method: Option<String>, // "auto", "manual"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchRequest {
    pub input_paths: Vec<String>,
    pub output_directory: String,
    pub operation: String, // "resize", "optimize", "convert"
    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingResult {
    pub success: bool,
    pub input_path: String,
    pub output_path: String,
    pub processing_time: f64,
    pub error_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OptimizationResult {
    pub original_size: u64,
    pub optimized_size: u64,
    pub compression_ratio: f32,
    pub quality_score: f32,
    pub output_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityAssessment {
    pub sharpness: f32,
    pub brightness: f32,
    pub contrast: f32,
    pub noise_level: f32,
    pub overall_score: f32,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WatermarkResponse {
    pub success: bool,
    pub output_path: String,
    pub processing_time: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioMetadata {
    pub duration: f64,
    pub sample_rate: u32,
    pub channels: u8,
    pub bitrate: u64,
    pub format: String,
    pub codec: String,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub year: Option<u32>,
    pub genre: Option<String>,
} 