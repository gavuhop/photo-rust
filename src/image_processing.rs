// Re-export image processing modules for organization
pub use crate::transformations::TransformationService;
pub use crate::effects::EffectService;
pub use crate::watermark::WatermarkService;
pub use crate::batch::BatchService;
pub use crate::quality::QualityService;
pub use crate::optimization::OptimizationService;

// Common image processing utilities
use anyhow::Result;
use image::{DynamicImage, GenericImageView};
use std::path::Path;

/// Core image processing utilities
pub struct ImageProcessor;

impl ImageProcessor {
    pub fn new() -> Self {
        Self
    }

    /// Load image from file path
    pub fn load_image(path: &str) -> Result<DynamicImage> {
        let img = image::open(path)?;
        Ok(img)
    }

    /// Save image to file path with quality settings
    pub fn save_image(img: &DynamicImage, path: &str, quality: Option<u8>) -> Result<()> {
        let path = Path::new(path);
        
        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        match path.extension().and_then(|ext| ext.to_str()) {
            Some("jpg") | Some("jpeg") => {
                let quality = quality.unwrap_or(85);
                let mut output = std::fs::File::create(path)?;
                img.write_to(&mut output, image::ImageOutputFormat::Jpeg(quality))?;
            }
            Some("png") => {
                img.save(path)?;
            }
            Some("webp") => {
                // WebP encoding would go here
                img.save(path)?;
            }
            Some("gif") => {
                img.save(path)?;
            }
            _ => {
                img.save(path)?;
            }
        }
        
        Ok(())
    }

    /// Get image dimensions
    pub fn get_dimensions(img: &DynamicImage) -> (u32, u32) {
        img.dimensions()
    }

    /// Check if image format is supported
    pub fn is_supported_format(path: &str) -> bool {
        let path = Path::new(path);
        match path.extension().and_then(|ext| ext.to_str()) {
            Some(ext) => matches!(ext.to_lowercase().as_str(), 
                "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "webp"),
            None => false,
        }
    }

    /// Calculate file size
    pub fn calculate_file_size(path: &str) -> Result<u64> {
        let metadata = std::fs::metadata(path)?;
        Ok(metadata.len())
    }
}

/// Image validation utilities
pub struct ImageValidator;

impl ImageValidator {
    /// Validate image file exists and is readable
    pub fn validate_input(path: &str) -> Result<()> {
        if !Path::new(path).exists() {
            return Err(anyhow::anyhow!("Input file does not exist: {}", path));
        }

        if !ImageProcessor::is_supported_format(path) {
            return Err(anyhow::anyhow!("Unsupported image format: {}", path));
        }

        Ok(())
    }

    /// Validate output path is writable
    pub fn validate_output(path: &str) -> Result<()> {
        let path = Path::new(path);
        
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }

        Ok(())
    }

    /// Validate image dimensions
    pub fn validate_dimensions(width: u32, height: u32) -> Result<()> {
        if width == 0 || height == 0 {
            return Err(anyhow::anyhow!("Invalid dimensions: {}x{}", width, height));
        }

        if width > 50000 || height > 50000 {
            return Err(anyhow::anyhow!("Dimensions too large: {}x{}", width, height));
        }

        Ok(())
    }
}

/// Performance monitoring
pub struct PerformanceMonitor {
    start_time: std::time::Instant,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
        }
    }

    pub fn elapsed_ms(&self) -> u64 {
        self.start_time.elapsed().as_millis() as u64
    }
}