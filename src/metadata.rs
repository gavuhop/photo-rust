use std::path::Path;
use anyhow::Result;
use chrono::{DateTime, Utc};
use image::io::Reader as ImageReader;
use std::fs;

use crate::models::*;

pub struct MetadataExtractor;

impl MetadataExtractor {
    pub fn new() -> Self {
        Self
    }

    pub async fn extract_metadata(&self, file_path: &str, extract_exif: bool, extract_ai: bool) -> Result<MetadataResponse> {
        let path = Path::new(file_path);
        
        // Basic file info
        let file_info = self.extract_file_info(path).await?;
        
        // EXIF data for images
        let exif_data = if extract_exif && self.is_image_file(path) {
            self.extract_exif_data(path).await.ok()
        } else {
            None
        };
        
        // AI analysis
        let ai_analysis = if extract_ai {
            self.perform_ai_analysis(path).await.ok()
        } else {
            None
        };
        
        Ok(MetadataResponse {
            file_info,
            exif_data,
            ai_analysis,
        })
    }

    async fn extract_file_info(&self, path: &Path) -> Result<FileInfo> {
        let metadata = fs::metadata(path)?;
        let size = metadata.len();
        
        let created = metadata.created().ok()
            .and_then(|time| DateTime::from_timestamp(
                time.duration_since(std::time::UNIX_EPOCH).ok()?.as_secs() as i64, 0
            ));
        
        let (width, height, duration, format) = if self.is_image_file(path) {
            let img = ImageReader::open(path)?.decode()?;
            (Some(img.width()), Some(img.height()), None, self.get_image_format(path))
        } else if self.is_video_file(path) {
            // TODO: Extract video metadata using FFmpeg
            (None, None, Some(0.0), self.get_video_format(path))
        } else if self.is_audio_file(path) {
            // TODO: Extract audio metadata
            (None, None, Some(0.0), self.get_audio_format(path))
        } else {
            (None, None, None, "unknown".to_string())
        };
        
        Ok(FileInfo {
            width,
            height,
            duration,
            format,
            size,
            created,
        })
    }

    async fn extract_exif_data(&self, path: &Path) -> Result<ExifData> {
        let file = std::fs::File::open(path)?;
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader)?;

        let mut exif_data = ExifData {
            camera_make: None,
            camera_model: None,
            lens: None,
            focal_length: None,
            aperture: None,
            shutter_speed: None,
            iso: None,
            flash: None,
            date_taken: None,
            gps_latitude: None,
            gps_longitude: None,
            gps_altitude: None,
        };

        // Extract common EXIF fields
        if let Some(field) = exif.get_field(exif::Tag::Make, exif::In::PRIMARY) {
            exif_data.camera_make = Some(field.display_value().to_string());
        }

        if let Some(field) = exif.get_field(exif::Tag::Model, exif::In::PRIMARY) {
            exif_data.camera_model = Some(field.display_value().to_string());
        }

        if let Some(field) = exif.get_field(exif::Tag::FocalLength, exif::In::PRIMARY) {
            if let exif::Value::Rational(ref vals) = field.value {
                if !vals.is_empty() {
                    exif_data.focal_length = Some(vals[0].to_f64());
                }
            }
        }

        if let Some(field) = exif.get_field(exif::Tag::FNumber, exif::In::PRIMARY) {
            if let exif::Value::Rational(ref vals) = field.value {
                if !vals.is_empty() {
                    exif_data.aperture = Some(vals[0].to_f64());
                }
            }
        }

        if let Some(field) = exif.get_field(exif::Tag::ExposureTime, exif::In::PRIMARY) {
            exif_data.shutter_speed = Some(field.display_value().to_string());
        }

        if let Some(field) = exif.get_field(exif::Tag::PhotographicSensitivity, exif::In::PRIMARY) {
            if let exif::Value::Short(ref vals) = field.value {
                if !vals.is_empty() {
                    exif_data.iso = Some(vals[0] as u32);
                }
            }
        }

        // GPS data
        if let Some(field) = exif.get_field(exif::Tag::GPSLatitude, exif::In::PRIMARY) {
            if let exif::Value::Rational(ref vals) = field.value {
                if vals.len() >= 3 {
                    let degrees = vals[0].to_f64();
                    let minutes = vals[1].to_f64();
                    let seconds = vals[2].to_f64();
                    exif_data.gps_latitude = Some(degrees + minutes / 60.0 + seconds / 3600.0);
                }
            }
        }

        if let Some(field) = exif.get_field(exif::Tag::GPSLongitude, exif::In::PRIMARY) {
            if let exif::Value::Rational(ref vals) = field.value {
                if vals.len() >= 3 {
                    let degrees = vals[0].to_f64();
                    let minutes = vals[1].to_f64();
                    let seconds = vals[2].to_f64();
                    exif_data.gps_longitude = Some(degrees + minutes / 60.0 + seconds / 3600.0);
                }
            }
        }

        Ok(exif_data)
    }

    async fn perform_ai_analysis(&self, path: &Path) -> Result<AIAnalysis> {
        // TODO: Implement AI analysis using candle or OpenCV
        // This would include:
        // - Object detection
        // - Face detection
        // - Color analysis
        // - Content classification
        
        let colors = self.extract_dominant_colors(path).await?;
        
        Ok(AIAnalysis {
            objects: Vec::new(), // TODO: Implement object detection
            faces: Vec::new(),   // TODO: Implement face detection
            colors,
            tags: Vec::new(),    // TODO: Generate tags based on analysis
            adult_content_score: None, // TODO: Content safety analysis
        })
    }

    async fn extract_dominant_colors(&self, path: &Path) -> Result<Vec<DominantColor>> {
        let img = image::open(path)?;
        let rgb_img = img.to_rgb8();
        
        // Simple color extraction - find most common colors
        let mut color_counts = std::collections::HashMap::new();
        
        for pixel in rgb_img.pixels() {
            // Reduce color space for grouping similar colors
            let r = (pixel[0] / 32) * 32;
            let g = (pixel[1] / 32) * 32;
            let b = (pixel[2] / 32) * 32;
            
            *color_counts.entry((r, g, b)).or_insert(0) += 1;
        }
        
        let total_pixels = rgb_img.pixels().len();
        let mut colors: Vec<_> = color_counts.into_iter().collect();
        colors.sort_by(|a, b| b.1.cmp(&a.1));
        
        let dominant_colors = colors.into_iter()
            .take(5) // Top 5 colors
            .map(|((r, g, b), count)| {
                let percentage = (count as f32 / total_pixels as f32) * 100.0;
                DominantColor {
                    hex: format!("#{:02x}{:02x}{:02x}", r, g, b),
                    rgb: (r, g, b),
                    percentage,
                }
            })
            .collect();
        
        Ok(dominant_colors)
    }

    fn is_image_file(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            matches!(ext.to_str().unwrap_or("").to_lowercase().as_str(), 
                "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "tiff")
        } else {
            false
        }
    }

    fn is_video_file(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            matches!(ext.to_str().unwrap_or("").to_lowercase().as_str(), 
                "mp4" | "avi" | "mov" | "mkv" | "wmv" | "flv" | "webm")
        } else {
            false
        }
    }

    fn is_audio_file(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            matches!(ext.to_str().unwrap_or("").to_lowercase().as_str(), 
                "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a")
        } else {
            false
        }
    }

    fn get_image_format(&self, path: &Path) -> String {
        path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("unknown")
            .to_uppercase()
    }

    fn get_video_format(&self, path: &Path) -> String {
        path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("unknown")
            .to_uppercase()
    }

    fn get_audio_format(&self, path: &Path) -> String {
        path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("unknown")
            .to_uppercase()
    }
}

pub struct VideoAnalyzer;

impl VideoAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub async fn analyze_video(&self, file_path: &str, extract_frames: bool, frame_interval: u32, extract_audio: bool) -> Result<VideoAnalysisResponse> {
        // TODO: Implement using FFmpeg
        // This would extract:
        // - Video metadata (duration, dimensions, codec, bitrate)
        // - Frames at specified intervals
        // - Audio track extraction
        
        log::info!("Analyzing video: {}", file_path);
        
        // Placeholder implementation
        Ok(VideoAnalysisResponse {
            duration: 120.0,
            width: 1920,
            height: 1080,
            frame_rate: 30.0,
            codec: "H.264".to_string(),
            bitrate: 5000000,
            frames: if extract_frames { Some(Vec::new()) } else { None },
            audio_path: if extract_audio { Some("/tmp/extracted_audio.mp3".to_string()) } else { None },
        })
    }
}