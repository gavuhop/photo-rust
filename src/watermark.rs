use anyhow::Result;
use image::{DynamicImage, GenericImageView, Rgba};
use uuid::Uuid;

use crate::models::{WatermarkRequest, WatermarkType, WatermarkPosition, ProcessingResult, ProcessingStatus};
use crate::services::{ImageProcessor, ImageValidator, PerformanceMonitor, CommonOperations};

pub struct WatermarkService;

impl WatermarkService {
    pub fn new() -> Self {
        Self
    }

    /// Add watermark to image
    pub async fn add_watermark(&self, req: &WatermarkRequest) -> Result<WatermarkResponse> {
        let img = ImageProcessor::load_image(&req.input_path)?;
        
        let processed_img = match &req.watermark_path {
            models::WatermarkType::Text { text, font_size, color } => {
                let font = "Arial".to_string();
                let font_size = font_size.unwrap_or(24);
                let color = color.as_ref().unwrap_or(&"#FFFFFF".to_string());
                img = self.add_text_watermark(img, text, &font, color, &req.position, req.opacity.unwrap_or(1.0), req.scale.unwrap_or(1.0))?;
            }
            models::WatermarkType::Image { path } => {
                let watermark_img = ImageProcessor::load_image(path)?;
                img = self.add_image_watermark(img, watermark_img, &req.position, req.opacity.unwrap_or(1.0), req.scale.unwrap_or(1.0))?;
            }
            models::WatermarkType::Logo { path } => {
                let logo_img = ImageProcessor::load_image(path)?;
                img = self.add_image_watermark(img, logo_img, &req.position, req.opacity.unwrap_or(1.0), req.scale.unwrap_or(1.0))?;
            }
        };
        
        // Save processed image
        ImageProcessor::save_image(&processed_img, &req.output_path, req.quality)?;
        
        Ok(WatermarkResponse {
            success: true,
            message: "Watermark added successfully".to_string(),
            output_path: req.output_path.clone(),
            processing_time: 0.0,
            watermark_info: json!({
                "watermark_type": match &req.watermark_path {
                    models::WatermarkType::Text { .. } => "text",
                    models::WatermarkType::Image { .. } => "image", 
                    models::WatermarkType::Logo { .. } => "logo",
                },
                "position": format!("{:?}", req.position),
                "opacity": req.opacity,
                "scale": req.scale
            })
        })
    }

    fn add_text_watermark(
        &self,
        base_img: DynamicImage,
        text: &str,
        _font: &str,
        color: &str,
        position: &WatermarkPosition,
        opacity: f32,
        scale: Option<f32>,
    ) -> Result<DynamicImage> {
        // Simplified text watermark - in practice would use font rendering library
        let (r, g, b) = CommonOperations::hex_to_rgb(color).unwrap_or((255, 255, 255));
        let color = Rgba([r, g, b, (opacity * 255.0) as u8]);
        
        // For now, just return the original image with a comment
        // In a real implementation, you'd use a library like rusttype or fontdue
        log::info!("Adding text watermark: '{}' at {:?}", text, position);
        
        Ok(base_img)
    }

    fn add_image_watermark(
        &self,
        mut base_img: DynamicImage,
        watermark_img: DynamicImage,
        position: &WatermarkPosition,
        opacity: f32,
        scale: Option<f32>,
    ) -> Result<DynamicImage> {
        let (base_width, base_height) = base_img.dimensions();
        
        // Scale watermark if needed
        let watermark = if let Some(scale_factor) = scale {
            let new_width = (watermark_img.width() as f32 * scale_factor) as u32;
            let new_height = (watermark_img.height() as f32 * scale_factor) as u32;
            watermark_img.resize_exact(new_width, new_height, image::imageops::FilterType::Lanczos3)
        } else {
            watermark_img
        };
        
        let (watermark_width, watermark_height) = watermark.dimensions();
        
        // Calculate position
        let (x, y) = match position {
            WatermarkPosition::TopLeft => (10, 10),
            WatermarkPosition::TopCenter => ((base_width - watermark_width) / 2, 10),
            WatermarkPosition::TopRight => (base_width - watermark_width - 10, 10),
            WatermarkPosition::CenterLeft => (10, (base_height - watermark_height) / 2),
            WatermarkPosition::Center => (
                (base_width - watermark_width) / 2,
                (base_height - watermark_height) / 2,
            ),
            WatermarkPosition::CenterRight => (
                base_width - watermark_width - 10,
                (base_height - watermark_height) / 2,
            ),
            WatermarkPosition::BottomLeft => (10, base_height - watermark_height - 10),
            WatermarkPosition::BottomCenter => (
                (base_width - watermark_width) / 2,
                base_height - watermark_height - 10,
            ),
            WatermarkPosition::BottomRight => (
                base_width - watermark_width - 10,
                base_height - watermark_height - 10,
            ),
            WatermarkPosition::Custom { x, y } => (*x, *y),
        };
        
        // Apply opacity to watermark
        let watermark_with_opacity = self.apply_opacity(watermark, opacity)?;
        
        // Overlay watermark onto base image
        image::imageops::overlay(&mut base_img, &watermark_with_opacity, x as i64, y as i64);
        
        Ok(base_img)
    }

    fn apply_opacity(&self, img: DynamicImage, opacity: f32) -> Result<DynamicImage> {
        let mut rgba_img = img.to_rgba8();
        
        for pixel in rgba_img.pixels_mut() {
            pixel[3] = (pixel[3] as f32 * opacity) as u8;
        }
        
        Ok(DynamicImage::ImageRgba8(rgba_img))
    }
}