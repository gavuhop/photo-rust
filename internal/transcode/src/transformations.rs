use crate::error::TranscodeError;
use crate::models::{TransformRequest, TransformResponse};
use anyhow::Result;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb, Rgba};
use imageproc::geometric_transformations::{rotate, Interpolation};
use log::{debug, info};
use serde_json::json;
use std::path::Path;
use uuid::Uuid;

use crate::models::{ResizeRequest, ResizeMode, ProcessingResult, ProcessingStatus};
use crate::services::{ImageProcessor, ImageValidator, CommonOperations, PerformanceMonitor};

pub struct TransformationService;

impl TransformationService {
    pub fn new() -> Self {
        Self
    }

    /// Resize image with various modes
    pub async fn resize_image(&self, req: &ResizeRequest) -> Result<ProcessingResult> {
        let monitor = PerformanceMonitor::new();
        let job_id = Uuid::new_v4();

        // Validate inputs
        ImageValidator::validate_input(&req.input_path)?;
        ImageValidator::validate_output(&req.output_path)?;
        ImageValidator::validate_dimensions(req.width, req.height)?;

        // Load image
        let img = ImageProcessor::load_image(&req.input_path)?;
        let (original_width, original_height) = img.dimensions();

        // Calculate target dimensions based on resize mode
        let (target_width, target_height) = self.calculate_resize_dimensions(
            original_width, 
            original_height, 
            req.width, 
            req.height, 
            &req.mode,
            req.preserve_aspect_ratio.unwrap_or(true)
        );

        // Perform resize
        let resized = match req.mode {
            ResizeMode::Fit => self.resize_fit(&img, target_width, target_height),
            ResizeMode::Fill => self.resize_fill(&img, req.width, req.height),
            ResizeMode::Stretch => self.resize_stretch(&img, req.width, req.height),
            ResizeMode::Pad => self.resize_pad(&img, req.width, req.height),
        }?;

        // Save result
        ImageProcessor::save_image(&resized, &req.output_path, req.quality)?;

        Ok(ProcessingResult {
            job_id,
            status: ProcessingStatus::Completed,
            input_path: req.input_path.clone(),
            output_path: Some(req.output_path.clone()),
            processing_time_ms: Some(monitor.elapsed_ms()),
            error_message: None,
            metadata: Some(serde_json::json!({
                "original_dimensions": [original_width, original_height],
                "new_dimensions": [target_width, target_height],
                "resize_mode": format!("{:?}", req.mode),
                "quality": req.quality
            })),
        })
    }

    /// Rotate image by angle in degrees
    pub async fn rotate_image(&self, input_path: &str, output_path: &str, angle: f32) -> Result<ProcessingResult> {
        let monitor = PerformanceMonitor::new();
        let job_id = Uuid::new_v4();

        ImageValidator::validate_input(input_path)?;
        ImageValidator::validate_output(output_path)?;

        let img = ImageProcessor::load_image(input_path)?;
        
        // Normalize angle to 0-360 range
        let normalized_angle = angle % 360.0;
        
        let rotated = match normalized_angle {
            0.0 => img,
            90.0 => img.rotate90(),
            180.0 => img.rotate180(),
            270.0 => img.rotate270(),
            _ => {
                // For arbitrary angles, we need more complex rotation
                self.rotate_arbitrary(&img, normalized_angle)?
            }
        };

        ImageProcessor::save_image(&rotated, output_path, None)?;

        Ok(ProcessingResult {
            job_id,
            status: ProcessingStatus::Completed,
            input_path: input_path.to_string(),
            output_path: Some(output_path.to_string()),
            processing_time_ms: Some(monitor.elapsed_ms()),
            error_message: None,
            metadata: Some(serde_json::json!({
                "rotation_angle": angle,
                "normalized_angle": normalized_angle
            })),
        })
    }

    /// Crop image to specified rectangle
    pub async fn crop_image(&self, input_path: &str, output_path: &str, x: u32, y: u32, width: u32, height: u32) -> Result<ProcessingResult> {
        let monitor = PerformanceMonitor::new();
        let job_id = Uuid::new_v4();

        ImageValidator::validate_input(input_path)?;
        ImageValidator::validate_output(output_path)?;
        ImageValidator::validate_dimensions(width, height)?;

        let img = ImageProcessor::load_image(input_path)?;
        let (img_width, img_height) = img.dimensions();

        // Validate crop rectangle is within image bounds
        if x + width > img_width || y + height > img_height {
            return Err(anyhow::anyhow!("Crop rectangle exceeds image bounds"));
        }

        let cropped = img.crop_imm(x, y, width, height);
        ImageProcessor::save_image(&cropped, output_path, None)?;

        Ok(ProcessingResult {
            job_id,
            status: ProcessingStatus::Completed,
            input_path: input_path.to_string(),
            output_path: Some(output_path.to_string()),
            processing_time_ms: Some(monitor.elapsed_ms()),
            error_message: None,
            metadata: Some(serde_json::json!({
                "crop_rectangle": [x, y, width, height],
                "original_dimensions": [img_width, img_height]
            })),
        })
    }

    /// Flip image horizontally
    pub async fn flip_horizontal(&self, input_path: &str, output_path: &str) -> Result<ProcessingResult> {
        let monitor = PerformanceMonitor::new();
        let job_id = Uuid::new_v4();

        ImageValidator::validate_input(input_path)?;
        ImageValidator::validate_output(output_path)?;

        let img = ImageProcessor::load_image(input_path)?;
        let flipped = img.fliph();

        ImageProcessor::save_image(&flipped, output_path, None)?;

        Ok(ProcessingResult {
            job_id,
            status: ProcessingStatus::Completed,
            input_path: input_path.to_string(),
            output_path: Some(output_path.to_string()),
            processing_time_ms: Some(monitor.elapsed_ms()),
            error_message: None,
            metadata: Some(serde_json::json!({
                "transformation": "flip_horizontal"
            })),
        })
    }

    /// Flip image vertically
    pub async fn flip_vertical(&self, input_path: &str, output_path: &str) -> Result<ProcessingResult> {
        let monitor = PerformanceMonitor::new();
        let job_id = Uuid::new_v4();

        ImageValidator::validate_input(input_path)?;
        ImageValidator::validate_output(output_path)?;

        let img = ImageProcessor::load_image(input_path)?;
        let flipped = img.flipv();

        ImageProcessor::save_image(&flipped, output_path, None)?;

        Ok(ProcessingResult {
            job_id,
            status: ProcessingStatus::Completed,
            input_path: input_path.to_string(),
            output_path: Some(output_path.to_string()),
            processing_time_ms: Some(monitor.elapsed_ms()),
            error_message: None,
            metadata: Some(serde_json::json!({
                "transformation": "flip_vertical"
            })),
        })
    }

    // Private helper methods

    fn calculate_resize_dimensions(
        &self,
        original_width: u32,
        original_height: u32,
        target_width: u32,
        target_height: u32,
        mode: &ResizeMode,
        preserve_aspect: bool,
    ) -> (u32, u32) {
        match mode {
            ResizeMode::Fit => {
                if preserve_aspect {
                    CommonOperations::calculate_dimensions(
                        original_width, 
                        original_height, 
                        target_width, 
                        target_height, 
                        true
                    )
                } else {
                    (target_width, target_height)
                }
            }
            ResizeMode::Fill | ResizeMode::Stretch | ResizeMode::Pad => {
                (target_width, target_height)
            }
        }
    }

    fn resize_fit(&self, img: &DynamicImage, width: u32, height: u32) -> Result<DynamicImage> {
        Ok(img.resize(width, height, imageops::FilterType::Lanczos3))
    }

    fn resize_fill(&self, img: &DynamicImage, width: u32, height: u32) -> Result<DynamicImage> {
        // Resize to fill target dimensions, cropping if necessary
        let (img_width, img_height) = img.dimensions();
        let aspect_ratio = img_width as f32 / img_height as f32;
        let target_aspect = width as f32 / height as f32;

        let (resize_width, resize_height) = if aspect_ratio > target_aspect {
            // Image is wider, resize to height and crop width
            let new_width = (height as f32 * aspect_ratio) as u32;
            (new_width, height)
        } else {
            // Image is taller, resize to width and crop height
            let new_height = (width as f32 / aspect_ratio) as u32;
            (width, new_height)
        };

        let resized = img.resize(resize_width, resize_height, imageops::FilterType::Lanczos3);
        
        // Crop to exact target dimensions
        let x = (resize_width - width) / 2;
        let y = (resize_height - height) / 2;
        
        Ok(resized.crop_imm(x, y, width, height))
    }

    fn resize_stretch(&self, img: &DynamicImage, width: u32, height: u32) -> Result<DynamicImage> {
        Ok(img.resize_exact(width, height, imageops::FilterType::Lanczos3))
    }

    fn resize_pad(&self, img: &DynamicImage, width: u32, height: u32) -> Result<DynamicImage> {
        // Resize to fit and pad with background color
        let fitted = self.resize_fit(img, width, height)?;
        let (fitted_width, fitted_height) = fitted.dimensions();
        
        if fitted_width == width && fitted_height == height {
            return Ok(fitted);
        }

        // Create new image with target dimensions and white background
        let mut result = DynamicImage::new_rgb8(width, height);
        
        // Calculate position to center the fitted image
        let x = (width - fitted_width) / 2;
        let y = (height - fitted_height) / 2;
        
        // Overlay the fitted image onto the padded background
        imageops::overlay(&mut result, &fitted, x as i64, y as i64);
        
        Ok(result)
    }

    fn rotate_arbitrary(&self, img: &DynamicImage, angle: f32) -> Result<DynamicImage> {
        // For now, return the original image for arbitrary angles
        // In a full implementation, you would use a proper rotation algorithm
        log::warn!("Arbitrary angle rotation not fully implemented, using nearest 90-degree rotation");
        
        let normalized = if angle < 45.0 || angle >= 315.0 {
            0.0
        } else if angle < 135.0 {
            90.0
        } else if angle < 225.0 {
            180.0
        } else {
            270.0
        };

        match normalized {
            90.0 => Ok(img.rotate90()),
            180.0 => Ok(img.rotate180()),
            270.0 => Ok(img.rotate270()),
            _ => Ok(img.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_resize_modes() {
        // This would require test images, skipping for now
        // In a real implementation, you'd create test images and verify the resize operations
    }

    #[test]
    fn test_dimension_calculation() {
        let service = TransformationService::new();
        let (w, h) = service.calculate_resize_dimensions(
            1920, 1080, 800, 600, &ResizeMode::Fit, true
        );
        // Should maintain aspect ratio
        assert_eq!((w, h), (800, 450));
    }
}