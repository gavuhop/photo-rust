use anyhow::Result;
use ffmpeg_next as ffmpeg;
use log::{error, info};
use std::process::Command;
use uuid::Uuid;
use crate::models::video::{VideoTranscodeRequest, AudioExtractRequest};

pub struct VideoProcessor;

impl VideoProcessor {
    pub fn new() -> Result<Self> {
        // Initialize FFmpeg
        ffmpeg::init()?;
        info!("FFmpeg initialized successfully");
        Ok(Self)
    }

    pub async fn transcode_video(&self, request: &VideoTranscodeRequest) -> Result<String> {
        let job_id = Uuid::new_v4().to_string();
        
        info!("Starting video transcode job: {}", job_id);
        
        // Validate input file exists
        if !std::path::Path::new(&request.input_path).exists() {
            return Err(anyhow::anyhow!("Input file not found: {}", request.input_path));
        }
        
        // Validate output directory exists
        if let Some(parent) = std::path::Path::new(&request.output_path).parent() {
            if !parent.exists() {
                return Err(anyhow::anyhow!("Output directory does not exist: {}", parent.display()));
            }
        }
        
        // Build FFmpeg command
        let mut command = Command::new("ffmpeg");
        
        // Input file
        command.arg("-i").arg(&request.input_path);
        
        // Output format
        if let Some(format) = &request.format {
            command.arg("-f").arg(format);
        }
        
        // Video codec
        if let Some(codec) = &request.codec {
            command.arg("-c:v").arg(codec);
        }
        
        // Bitrate
        if let Some(bitrate) = &request.bitrate {
            command.arg("-b:v").arg(bitrate);
        }
        
        // Resolution
        if let Some(resolution) = &request.resolution {
            command.arg("-s").arg(resolution);
        }
        
        // FPS
        if let Some(fps) = request.fps {
            command.arg("-r").arg(fps.to_string());
        }
        
        // Output file
        command.arg(&request.output_path);
        
        info!("Executing FFmpeg command: {:?}", command);
        
        // Execute FFmpeg command
        let output = command.output()?;
        
        if output.status.success() {
            info!("Video transcode completed successfully: {}", job_id);
            Ok(job_id)
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            error!("FFmpeg error: {}", error);
            Err(anyhow::anyhow!("FFmpeg processing failed: {}", error))
        }
    }

    pub async fn extract_audio(&self, request: &AudioExtractRequest) -> Result<String> {
        let job_id = Uuid::new_v4().to_string();
        
        info!("Starting audio extraction job: {}", job_id);
        
        // Validate input file exists
        if !std::path::Path::new(&request.input_path).exists() {
            return Err(anyhow::anyhow!("Input file not found: {}", request.input_path));
        }
        
        // Validate output directory exists
        if let Some(parent) = std::path::Path::new(&request.output_path).parent() {
            if !parent.exists() {
                return Err(anyhow::anyhow!("Output directory does not exist: {}", parent.display()));
            }
        }
        
        let mut command = Command::new("ffmpeg");
        
        // Input file
        command.arg("-i").arg(&request.input_path);
        
        // No video
        command.arg("-vn");
        
        // Audio codec (default to mp3)
        let codec = request.format.as_deref().unwrap_or("libmp3lame");
        command.arg("-acodec").arg(codec);
        
        // Bitrate
        if let Some(bitrate) = &request.bitrate {
            command.arg("-b:a").arg(bitrate);
        }
        
        // Output file
        command.arg(&request.output_path);
        
        info!("Executing FFmpeg command for audio extraction: {:?}", command);
        
        let output = command.output()?;
            
        if output.status.success() {
            info!("Audio extraction completed successfully: {}", job_id);
            Ok(job_id)
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            error!("FFmpeg error during audio extraction: {}", error);
            Err(anyhow::anyhow!("Audio extraction failed: {}", error))
        }
    }

    pub async fn get_video_info(&self, file_path: &str) -> Result<serde_json::Value> {
        info!("Getting video info for: {}", file_path);
        
        // Validate file exists
        if !std::path::Path::new(file_path).exists() {
            return Err(anyhow::anyhow!("File not found: {}", file_path));
        }
        
        // Validate file is readable (try to open it)
        if let Err(_) = std::fs::File::open(file_path) {
            return Err(anyhow::anyhow!("File is not readable: {}", file_path));
        }
        
        let output = Command::new("ffprobe")
            .arg("-v").arg("quiet")
            .arg("-print_format").arg("json")
            .arg("-show_format")
            .arg("-show_streams")
            .arg(file_path)
            .output()?;
            
        if output.status.success() {
            let json_str = String::from_utf8_lossy(&output.stdout);
            let info: serde_json::Value = serde_json::from_str(&json_str)?;
            info!("Successfully retrieved video info for: {}", file_path);
            Ok(info)
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            error!("FFprobe error: {}", error);
            Err(anyhow::anyhow!("Failed to get video info: {}", error))
        }
    }

    pub async fn transcode_audio(&self, input_path: &str, output_path: &str, format: Option<&str>) -> Result<String> {
        let job_id = Uuid::new_v4().to_string();
        
        info!("Starting audio transcode job: {}", job_id);
        
        // Validate input file exists
        if !std::path::Path::new(input_path).exists() {
            return Err(anyhow::anyhow!("Input file not found: {}", input_path));
        }
        
        // Validate output directory exists
        if let Some(parent) = std::path::Path::new(output_path).parent() {
            if !parent.exists() {
                return Err(anyhow::anyhow!("Output directory does not exist: {}", parent.display()));
            }
        }
        
        let mut command = Command::new("ffmpeg");
        
        // Input file
        command.arg("-i").arg(input_path);
        
        // Output format
        if let Some(fmt) = format {
            command.arg("-f").arg(fmt);
        }
        
        // Output file
        command.arg(output_path);
        
        info!("Executing FFmpeg command for audio transcode: {:?}", command);
        
        let output = command.output()?;
            
        if output.status.success() {
            info!("Audio transcode completed successfully: {}", job_id);
            Ok(job_id)
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            error!("FFmpeg error during audio transcode: {}", error);
            Err(anyhow::anyhow!("Audio transcode failed: {}", error))
        }
    }
} 