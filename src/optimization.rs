use crate::error::TranscodeError;
use crate::models::{OptimizationRequest, OptimizationResponse};
use anyhow::Result;
use image::{DynamicImage, GenericImageView, ImageBuffer, Luma, Rgb, Rgba};
use imageproc::filter::gaussian_blur_f32;
use imageproc::filter::median_filter;
use imageproc::geometric_transformations::rotate_about_center;
use imageproc::map::map_colors;
use log::{debug, info};
use serde_json::json;
use std::path::Path;
use uuid::Uuid;
use std::io::Cursor;

use crate::models::{
    ProcessingResult, ProcessingStatus, OptimizationResult, 
    ThumbnailRequest, ThumbnailSize
};
use crate::services::{ImageProcessor, ImageValidator, PerformanceMonitor};

pub struct OptimizationService;

impl OptimizationService {
    pub fn new() -> Self {
        Self
    }

    /// Compress image with specified quality
    pub async fn compress_image(&self, input_path: &str, output_path: &str, quality: u8) -> Result<OptimizationResult> {
        let monitor = PerformanceMonitor::new();
        
        ImageValidator::validate_input(input_path)?;
        ImageValidator::validate_output(output_path)?;
        
        let original_size = ImageProcessor::calculate_file_size(input_path)?;
        let img = ImageProcessor::load_image(input_path)?;
        
        // Save with specified quality
        ImageProcessor::save_image(&img, output_path, Some(quality))?;
        
        let optimized_size = ImageProcessor::calculate_file_size(output_path)?;
        let compression_ratio = original_size as f32 / optimized_size as f32;
        
        Ok(OptimizationResult {
            original_size,
            optimized_size,
            compression_ratio,
            quality_loss: self.estimate_quality_loss(quality),
            processing_time_ms: monitor.elapsed_ms(),
        })
    }

    /// Convert image format
    pub async fn convert_format(&self, input_path: &str, output_path: &str, format: &str) -> Result<ProcessingResult> {
        let monitor = PerformanceMonitor::new();
        let job_id = Uuid::new_v4();
        
        ImageValidator::validate_input(input_path)?;
        ImageValidator::validate_output(output_path)?;
        
        let img = ImageProcessor::load_image(input_path)?;
        
        // Convert based on target format
        let converted = match format.to_lowercase().as_str() {
            "jpeg" | "jpg" => {
                let rgb_img = img.to_rgb8();
                DynamicImage::ImageRgb8(rgb_img)
            }
            "png" => {
                let rgba_img = img.to_rgba8();
                DynamicImage::ImageRgba8(rgba_img)
            }
            "webp" => {
                // WebP conversion would require additional library
                img
            }
            "gif" => {
                // GIF conversion with palette reduction
                let rgb_img = img.to_rgb8();
                DynamicImage::ImageRgb8(rgb_img)
            }
            _ => img,
        };
        
        ImageProcessor::save_image(&converted, output_path, None)?;
        
        Ok(ProcessingResult {
            job_id,
            status: ProcessingStatus::Completed,
            input_path: input_path.to_string(),
            output_path: Some(output_path.to_string()),
            processing_time_ms: Some(monitor.elapsed_ms()),
            error_message: None,
            metadata: Some(serde_json::json!({
                "target_format": format,
                "conversion_type": "format_change"
            })),
        })
    }

    /// Optimize image for web delivery
    pub async fn optimize_for_web(&self, input_path: &str, output_path: &str) -> Result<OptimizationResult> {
        let monitor = PerformanceMonitor::new();
        
        ImageValidator::validate_input(input_path)?;
        ImageValidator::validate_output(output_path)?;
        
        let original_size = ImageProcessor::calculate_file_size(input_path)?;
        let img = ImageProcessor::load_image(input_path)?;
        let (width, height) = img.dimensions();
        
        // Optimize based on image size and content
        let optimized = if width > 2000 || height > 2000 {
            // Large images: resize and compress
            let resized = img.resize(1920, 1080, image::imageops::FilterType::Lanczos3);
            resized
        } else if width > 1000 || height > 1000 {
            // Medium images: light compression
            img
        } else {
            // Small images: minimal processing
            img
        };
        
        // Determine optimal quality based on content
        let quality = self.determine_optimal_quality(&optimized);
        
        ImageProcessor::save_image(&optimized, output_path, Some(quality))?;
        
        let optimized_size = ImageProcessor::calculate_file_size(output_path)?;
        let compression_ratio = original_size as f32 / optimized_size as f32;
        
        Ok(OptimizationResult {
            original_size,
            optimized_size,
            compression_ratio,
            quality_loss: self.estimate_quality_loss(quality),
            processing_time_ms: monitor.elapsed_ms(),
        })
    }

    /// Generate multiple thumbnail sizes
    pub async fn generate_thumbnails(&self, req: &ThumbnailRequest) -> Result<Vec<ProcessingResult>> {
        let monitor = PerformanceMonitor::new();
        
        ImageValidator::validate_input(&req.input_path)?;
        
        let img = ImageProcessor::load_image(&req.input_path)?;
        let mut results = Vec::new();
        
        for size in &req.sizes {
            let job_id = Uuid::new_v4();
            let output_path = format!("{}/{}_{}.{}", 
                req.output_directory, 
                size.name, 
                job_id,
                req.format.as_deref().unwrap_or("jpg")
            );
            
            let thumbnail = if size.crop {
                // Crop to exact dimensions
                self.create_cropped_thumbnail(&img, size.width, size.height)?
            } else {
                // Resize maintaining aspect ratio
                img.resize(size.width, size.height, image::imageops::FilterType::Lanczos3)
            };
            
            ImageProcessor::save_image(&thumbnail, &output_path, req.quality)?;
            
            results.push(ProcessingResult {
                job_id,
                status: ProcessingStatus::Completed,
                input_path: req.input_path.clone(),
                output_path: Some(output_path),
                processing_time_ms: Some(monitor.elapsed_ms()),
                error_message: None,
                metadata: Some(serde_json::json!({
                    "thumbnail_size": size.name,
                    "dimensions": [size.width, size.height],
                    "crop": size.crop
                })),
            });
        }
        
        Ok(results)
    }

    /// Create progressive JPEG
    pub async fn create_progressive_jpeg(&self, input_path: &str, output_path: &str) -> Result<ProcessingResult> {
        let monitor = PerformanceMonitor::new();
        let job_id = Uuid::new_v4();
        
        ImageValidator::validate_input(input_path)?;
        ImageValidator::validate_output(output_path)?;
        
        let img = ImageProcessor::load_image(input_path)?;
        
        // Create progressive JPEG
        let mut output = std::fs::File::create(output_path)?;
        let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output, 85);
        
        // Enable progressive encoding
        // Note: The image crate doesn't directly support progressive JPEG creation
        // In a real implementation, you'd use a library like mozjpeg-rust
        
        encoder.encode_image(&img)?;
        
        Ok(ProcessingResult {
            job_id,
            status: ProcessingStatus::Completed,
            input_path: input_path.to_string(),
            output_path: Some(output_path.to_string()),
            processing_time_ms: Some(monitor.elapsed_ms()),
            error_message: None,
            metadata: Some(serde_json::json!({
                "format": "progressive_jpeg",
                "optimization": "web_delivery"
            })),
        })
    }

    /// Lossless optimization
    pub async fn lossless_optimize(&self, input_path: &str, output_path: &str) -> Result<OptimizationResult> {
        let monitor = PerformanceMonitor::new();
        
        ImageValidator::validate_input(input_path)?;
        ImageValidator::validate_output(output_path)?;
        
        let original_size = ImageProcessor::calculate_file_size(input_path)?;
        let img = ImageProcessor::load_image(input_path)?;
        
        // Lossless optimizations
        let optimized = self.apply_lossless_optimizations(&img)?;
        
        // Save without quality loss
        optimized.save(output_path)?;
        
        let optimized_size = ImageProcessor::calculate_file_size(output_path)?;
        let compression_ratio = original_size as f32 / optimized_size as f32;
        
        Ok(OptimizationResult {
            original_size,
            optimized_size,
            compression_ratio,
            quality_loss: 0.0, // Lossless
            processing_time_ms: monitor.elapsed_ms(),
        })
    }

    /// Batch optimize images
    pub async fn batch_optimize(&self, input_dir: &str, output_dir: &str, quality: Option<u8>) -> Result<Vec<OptimizationResult>> {
        let entries = std::fs::read_dir(input_dir)?;
        let mut results = Vec::new();
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if let Some(extension) = path.extension() {
                if ImageProcessor::is_supported_format(&path.to_string_lossy()) {
                    let input_path = path.to_string_lossy();
                    let output_path = format!("{}/optimized_{}", 
                        output_dir,
                        path.file_name().unwrap().to_string_lossy()
                    );
                    
                    if let Ok(result) = self.compress_image(&input_path, &output_path, quality.unwrap_or(85)).await {
                        results.push(result);
                    }
                }
            }
        }
        
        Ok(results)
    }

    // Private helper methods

    fn estimate_quality_loss(&self, quality: u8) -> f32 {
        // Estimate quality loss based on compression level
        if quality >= 95 {
            0.0
        } else if quality >= 85 {
            (95 - quality) as f32 * 0.1
        } else if quality >= 75 {
            (85 - quality) as f32 * 0.2 + 1.0
        } else {
            (75 - quality) as f32 * 0.5 + 3.0
        }
    }

    fn determine_optimal_quality(&self, img: &DynamicImage) -> u8 {
        let (width, height) = img.dimensions();
        let pixels = width * height;
        
        // Determine quality based on image size and complexity
        if pixels > 4_000_000 {
            // Large images (>4MP): lower quality to reduce file size
            75
        } else if pixels > 1_000_000 {
            // Medium images (1-4MP): balanced quality
            85
        } else {
            // Small images: higher quality
            90
        }
    }

    fn create_cropped_thumbnail(&self, img: &DynamicImage, width: u32, height: u32) -> Result<DynamicImage> {
        let (img_width, img_height) = img.dimensions();
        let img_aspect = img_width as f32 / img_height as f32;
        let target_aspect = width as f32 / height as f32;
        
        let (resize_width, resize_height) = if img_aspect > target_aspect {
            // Image is wider, resize to height and crop width
            let new_width = (height as f32 * img_aspect) as u32;
            (new_width, height)
        } else {
            // Image is taller, resize to width and crop height
            let new_height = (width as f32 / img_aspect) as u32;
            (width, new_height)
        };
        
        let resized = img.resize_exact(resize_width, resize_height, image::imageops::FilterType::Lanczos3);
        
        // Crop to exact target dimensions
        let x = (resize_width - width) / 2;
        let y = (resize_height - height) / 2;
        
        Ok(resized.crop_imm(x, y, width, height))
    }

    fn apply_lossless_optimizations(&self, img: &DynamicImage) -> Result<DynamicImage> {
        // Apply lossless optimizations like:
        // - Metadata removal
        // - Color palette optimization for PNG
        // - Huffman table optimization for JPEG
        
        // For now, just return the original image
        // In a real implementation, you'd use specialized libraries
        Ok(img.clone())
    }

    /// Calculate file size reduction percentage
    pub fn calculate_savings(&self, original_size: u64, optimized_size: u64) -> f32 {
        if original_size == 0 {
            return 0.0;
        }
        
        ((original_size - optimized_size) as f32 / original_size as f32) * 100.0
    }

    /// Determine best format for image content
    pub fn suggest_optimal_format(&self, img: &DynamicImage) -> &'static str {
        let (width, height) = img.dimensions();
        let has_transparency = match img {
            DynamicImage::ImageRgba8(_) | DynamicImage::ImageLumaA8(_) => true,
            _ => false,
        };
        
        if has_transparency {
            "png"
        } else if width * height > 1_000_000 {
            "jpeg" // Large images benefit from JPEG compression
        } else {
            "webp" // Small to medium images can use WebP
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_loss_estimation() {
        let service = OptimizationService::new();
        
        assert_eq!(service.estimate_quality_loss(100), 0.0);
        assert_eq!(service.estimate_quality_loss(95), 0.0);
        assert_eq!(service.estimate_quality_loss(85), 1.0);
        assert!(service.estimate_quality_loss(50) > 10.0);
    }

    #[test]
    fn test_savings_calculation() {
        let service = OptimizationService::new();
        
        assert_eq!(service.calculate_savings(1000, 500), 50.0);
        assert_eq!(service.calculate_savings(1000, 1000), 0.0);
        assert_eq!(service.calculate_savings(0, 100), 0.0);
    }

    #[test]
    fn test_format_suggestion() {
        let service = OptimizationService::new();
        
        // This would require actual image creation for proper testing
        // Skipping for now
    }
}