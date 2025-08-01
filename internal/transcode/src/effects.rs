use crate::error::TranscodeError;
use crate::models::{EffectRequest, EffectResponse};
use anyhow::Result;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb, Rgba};
use imageproc::filter::gaussian_blur_f32;
use imageproc::filter::median_filter;
use imageproc::geometric_transformations::rotate_about_center;
use imageproc::map::map_colors;
use log::{debug, info};
use serde_json::json;
use std::path::Path;
use uuid::Uuid;

use crate::models::{
    EffectRequest, EffectType, ProcessingResult, ProcessingStatus,
    StyleTransferRequest, PanoramaRequest, StitchMode
};
use crate::services::{ImageProcessor, ImageValidator, PerformanceMonitor};

pub struct EffectService;

impl EffectService {
    pub fn new() -> Self {
        Self
    }

    /// Apply artistic effect to image
    pub async fn apply_effect(&self, req: &EffectRequest) -> Result<ProcessingResult> {
        let monitor = PerformanceMonitor::new();
        let job_id = Uuid::new_v4();

        ImageValidator::validate_input(&req.input_path)?;
        ImageValidator::validate_output(&req.output_path)?;

        let img = ImageProcessor::load_image(&req.input_path)?;
        
        let processed = match req.effect_type {
            EffectType::Glitch => self.apply_glitch_effect(&img, &req.parameters)?,
            EffectType::Pixelate => self.apply_pixelate_effect(&img, &req.parameters)?,
            EffectType::OilPainting => self.apply_oil_painting_effect(&img, &req.parameters)?,
            EffectType::Watercolor => self.apply_watercolor_effect(&img, &req.parameters)?,
            EffectType::Pencil => self.apply_pencil_effect(&img, &req.parameters)?,
            EffectType::Cartoon => self.apply_cartoon_effect(&img, &req.parameters)?,
            EffectType::HDR => self.apply_hdr_effect(&img, &req.parameters)?,
            EffectType::Orton => self.apply_orton_effect(&img, &req.parameters)?,
            EffectType::CrossProcess => self.apply_cross_process_effect(&img, &req.parameters)?,
            EffectType::Lomography => self.apply_lomography_effect(&img, &req.parameters)?,
            EffectType::Infrared => self.apply_infrared_effect(&img, &req.parameters)?,
            EffectType::TiltShift => self.apply_tilt_shift_effect(&img, &req.parameters)?,
        };

        ImageProcessor::save_image(&processed, &req.output_path, None)?;

        Ok(ProcessingResult {
            job_id,
            status: ProcessingStatus::Completed,
            input_path: req.input_path.clone(),
            output_path: Some(req.output_path.clone()),
            processing_time_ms: Some(monitor.elapsed_ms()),
            error_message: None,
            metadata: Some(serde_json::json!({
                "effect_type": format!("{:?}", req.effect_type),
                "parameters": req.parameters
            })),
        })
    }

    /// Apply artistic effect
    pub async fn apply_artistic_effect(&self, req: &EffectRequest) -> Result<ProcessingResult> {
        self.apply_effect(req).await
    }

    /// Remove background (placeholder)
    pub async fn remove_background(&self, input_path: &str, output_path: &str) -> Result<ProcessingResult> {
        let monitor = PerformanceMonitor::new();
        let job_id = Uuid::new_v4();

        ImageValidator::validate_input(input_path)?;
        ImageValidator::validate_output(output_path)?;

        let img = ImageProcessor::load_image(input_path)?;
        
        // Placeholder background removal - in practice would use ML models
        let processed = self.simple_background_removal(&img)?;
        
        ImageProcessor::save_image(&processed, output_path, None)?;

        Ok(ProcessingResult {
            job_id,
            status: ProcessingStatus::Completed,
            input_path: input_path.to_string(),
            output_path: Some(output_path.to_string()),
            processing_time_ms: Some(monitor.elapsed_ms()),
            error_message: None,
            metadata: Some(serde_json::json!({
                "effect": "background_removal",
                "method": "edge_detection"
            })),
        })
    }

    /// Style transfer (placeholder)
    pub async fn style_transfer(&self, req: &StyleTransferRequest) -> Result<ProcessingResult> {
        let monitor = PerformanceMonitor::new();
        let job_id = Uuid::new_v4();

        ImageValidator::validate_input(&req.content_path)?;
        ImageValidator::validate_input(&req.style_path)?;
        ImageValidator::validate_output(&req.output_path)?;

        let content_img = ImageProcessor::load_image(&req.content_path)?;
        let _style_img = ImageProcessor::load_image(&req.style_path)?;
        
        // Placeholder style transfer - in practice would use neural networks
        let result = self.simple_style_transfer(&content_img, req.style_strength)?;
        
        ImageProcessor::save_image(&result, &req.output_path, None)?;

        Ok(ProcessingResult {
            job_id,
            status: ProcessingStatus::Completed,
            input_path: req.content_path.clone(),
            output_path: Some(req.output_path.clone()),
            processing_time_ms: Some(monitor.elapsed_ms()),
            error_message: None,
            metadata: Some(serde_json::json!({
                "effect": "style_transfer",
                "style_strength": req.style_strength,
                "preservation_level": req.preservation_level
            })),
        })
    }

    /// Stitch panorama (placeholder)
    pub async fn stitch_panorama(&self, req: &PanoramaRequest) -> Result<ProcessingResult> {
        let monitor = PerformanceMonitor::new();
        let job_id = Uuid::new_v4();

        for path in &req.input_paths {
            ImageValidator::validate_input(path)?;
        }
        ImageValidator::validate_output(&req.output_path)?;

        let images: Result<Vec<_>> = req.input_paths.iter()
            .map(|path| ImageProcessor::load_image(path))
            .collect();
        let images = images?;
        
        let panorama = self.simple_panorama_stitch(&images, &req.stitch_mode)?;
        
        ImageProcessor::save_image(&panorama, &req.output_path, None)?;

        Ok(ProcessingResult {
            job_id,
            status: ProcessingStatus::Completed,
            input_path: req.input_paths[0].clone(),
            output_path: Some(req.output_path.clone()),
            processing_time_ms: Some(monitor.elapsed_ms()),
            error_message: None,
            metadata: Some(serde_json::json!({
                "effect": "panorama_stitch",
                "input_count": req.input_paths.len(),
                "stitch_mode": format!("{:?}", req.stitch_mode)
            })),
        })
    }

    // Effect implementations (simplified)
    
    fn apply_glitch_effect(&self, img: &DynamicImage, _params: &std::collections::HashMap<String, serde_json::Value>) -> Result<DynamicImage> {
        let mut rgb_img = img.to_rgb8();
        let (width, height) = rgb_img.dimensions();
        
        // Simple glitch effect: channel shifting
        for y in 0..height {
            for x in 0..width {
                if x > 0 && y > 0 {
                    let pixel = rgb_img.get_pixel(x, y);
                    let shifted_pixel = rgb_img.get_pixel(x - 1, y - 1);
                    
                    let new_pixel = image::Rgb([
                        pixel[0],
                        shifted_pixel[1],
                        pixel[2],
                    ]);
                    
                    rgb_img.put_pixel(x, y, new_pixel);
                }
            }
        }
        
        Ok(DynamicImage::ImageRgb8(rgb_img))
    }

    fn apply_pixelate_effect(&self, img: &DynamicImage, params: &std::collections::HashMap<String, serde_json::Value>) -> Result<DynamicImage> {
        let pixel_size = params.get("pixel_size")
            .and_then(|v| v.as_f64())
            .unwrap_or(8.0) as u32;
        
        let (width, height) = img.dimensions();
        let small_width = width / pixel_size;
        let small_height = height / pixel_size;
        
        // Downscale and upscale for pixelation
        let small = img.resize_exact(small_width, small_height, image::imageops::FilterType::Nearest);
        let pixelated = small.resize_exact(width, height, image::imageops::FilterType::Nearest);
        
        Ok(pixelated)
    }

    fn apply_oil_painting_effect(&self, img: &DynamicImage, _params: &std::collections::HashMap<String, serde_json::Value>) -> Result<DynamicImage> {
        // Simplified oil painting effect using blur and color quantization
        let blurred = img.blur(2.0);
        
        // Color quantization
        let rgb_img = blurred.to_rgb8();
        let mut result = image::ImageBuffer::new(rgb_img.width(), rgb_img.height());
        
        for (x, y, pixel) in rgb_img.enumerate_pixels() {
            let quantized = image::Rgb([
                (pixel[0] / 32) * 32,
                (pixel[1] / 32) * 32,
                (pixel[2] / 32) * 32,
            ]);
            result.put_pixel(x, y, quantized);
        }
        
        Ok(DynamicImage::ImageRgb8(result))
    }

    fn apply_watercolor_effect(&self, img: &DynamicImage, _params: &std::collections::HashMap<String, serde_json::Value>) -> Result<DynamicImage> {
        // Watercolor effect: heavy blur + edge preservation
        let blurred = img.blur(5.0);
        let edges = self.detect_edges(img)?;
        
        // Combine blurred image with edge mask
        self.blend_images(&blurred, &edges, 0.8)
    }

    fn apply_pencil_effect(&self, img: &DynamicImage, _params: &std::collections::HashMap<String, serde_json::Value>) -> Result<DynamicImage> {
        // Pencil sketch effect
        let gray = img.grayscale();
        let inverted = self.invert_image(&gray)?;
        let blurred = inverted.blur(25.0);
        
        // Dodge blend mode simulation
        self.dodge_blend(&gray, &blurred)
    }

    fn apply_cartoon_effect(&self, img: &DynamicImage, _params: &std::collections::HashMap<String, serde_json::Value>) -> Result<DynamicImage> {
        // Cartoon effect: bilateral filter + edge detection
        let smooth = img.blur(1.0);
        let edges = self.detect_edges(img)?;
        
        // Color quantization
        let quantized = self.quantize_colors(&smooth, 8)?;
        
        // Combine with edges
        self.blend_images(&quantized, &edges, 0.9)
    }

    fn apply_hdr_effect(&self, img: &DynamicImage, _params: &std::collections::HashMap<String, serde_json::Value>) -> Result<DynamicImage> {
        // HDR tone mapping simulation
        let rgb_img = img.to_rgb8();
        let mut result = image::ImageBuffer::new(rgb_img.width(), rgb_img.height());
        
        for (x, y, pixel) in rgb_img.enumerate_pixels() {
            let r = pixel[0] as f32 / 255.0;
            let g = pixel[1] as f32 / 255.0;
            let b = pixel[2] as f32 / 255.0;
            
            // Simple tone mapping
            let mapped_r = r / (r + 1.0);
            let mapped_g = g / (g + 1.0);
            let mapped_b = b / (b + 1.0);
            
            // Increase contrast
            let final_r = ((mapped_r - 0.5) * 1.5 + 0.5).max(0.0).min(1.0);
            let final_g = ((mapped_g - 0.5) * 1.5 + 0.5).max(0.0).min(1.0);
            let final_b = ((mapped_b - 0.5) * 1.5 + 0.5).max(0.0).min(1.0);
            
            result.put_pixel(x, y, image::Rgb([
                (final_r * 255.0) as u8,
                (final_g * 255.0) as u8,
                (final_b * 255.0) as u8,
            ]));
        }
        
        Ok(DynamicImage::ImageRgb8(result))
    }

    fn apply_orton_effect(&self, img: &DynamicImage, _params: &std::collections::HashMap<String, serde_json::Value>) -> Result<DynamicImage> {
        // Orton effect: dreamy, glowing look
        let blurred = img.blur(20.0);
        let overlay = self.screen_blend(img, &blurred, 0.5)?;
        
        // Increase saturation
        self.adjust_saturation(&overlay, 1.3)
    }

    fn apply_cross_process_effect(&self, img: &DynamicImage, _params: &std::collections::HashMap<String, serde_json::Value>) -> Result<DynamicImage> {
        // Cross processing effect
        let rgb_img = img.to_rgb8();
        let mut result = image::ImageBuffer::new(rgb_img.width(), rgb_img.height());
        
        for (x, y, pixel) in rgb_img.enumerate_pixels() {
            let r = pixel[0] as f32 / 255.0;
            let g = pixel[1] as f32 / 255.0;
            let b = pixel[2] as f32 / 255.0;
            
            // Cross processing curve simulation
            let new_r = (r * 1.1).min(1.0);
            let new_g = (g * 0.9).max(0.0);
            let new_b = (b * 1.2).min(1.0);
            
            result.put_pixel(x, y, image::Rgb([
                (new_r * 255.0) as u8,
                (new_g * 255.0) as u8,
                (new_b * 255.0) as u8,
            ]));
        }
        
        Ok(DynamicImage::ImageRgb8(result))
    }

    fn apply_lomography_effect(&self, img: &DynamicImage, _params: &std::collections::HashMap<String, serde_json::Value>) -> Result<DynamicImage> {
        // Lomography effect: vignette + color cast
        let vignette = self.apply_vignette(img, 0.6)?;
        self.apply_color_cast(&vignette, 1.1, 0.9, 0.8)
    }

    fn apply_infrared_effect(&self, img: &DynamicImage, _params: &std::collections::HashMap<String, serde_json::Value>) -> Result<DynamicImage> {
        // Infrared effect: channel swapping
        let rgb_img = img.to_rgb8();
        let mut result = image::ImageBuffer::new(rgb_img.width(), rgb_img.height());
        
        for (x, y, pixel) in rgb_img.enumerate_pixels() {
            // Simulate infrared by swapping channels
            result.put_pixel(x, y, image::Rgb([
                pixel[1], // Green -> Red
                pixel[0], // Red -> Green
                pixel[2], // Blue stays
            ]));
        }
        
        Ok(DynamicImage::ImageRgb8(result))
    }

    fn apply_tilt_shift_effect(&self, img: &DynamicImage, _params: &std::collections::HashMap<String, serde_json::Value>) -> Result<DynamicImage> {
        // Tilt-shift effect: selective focus
        let (width, height) = img.dimensions();
        let focus_center = height / 2;
        let focus_range = height / 6;
        
        let rgb_img = img.to_rgb8();
        let blurred_img = img.blur(8.0).to_rgb8();
        let mut result = image::ImageBuffer::new(width, height);
        
        for (x, y, _) in rgb_img.enumerate_pixels() {
            let distance_from_center = (y as i32 - focus_center as i32).abs() as u32;
            let blur_strength = if distance_from_center < focus_range {
                0.0
            } else {
                ((distance_from_center - focus_range) as f32 / focus_range as f32).min(1.0)
            };
            
            let original_pixel = rgb_img.get_pixel(x, y);
            let blurred_pixel = blurred_img.get_pixel(x, y);
            
            let final_pixel = image::Rgb([
                (original_pixel[0] as f32 * (1.0 - blur_strength) + blurred_pixel[0] as f32 * blur_strength) as u8,
                (original_pixel[1] as f32 * (1.0 - blur_strength) + blurred_pixel[1] as f32 * blur_strength) as u8,
                (original_pixel[2] as f32 * (1.0 - blur_strength) + blurred_pixel[2] as f32 * blur_strength) as u8,
            ]);
            
            result.put_pixel(x, y, final_pixel);
        }
        
        Ok(DynamicImage::ImageRgb8(result))
    }

    // Helper methods
    
    fn detect_edges(&self, img: &DynamicImage) -> Result<DynamicImage> {
        let gray = img.to_luma8();
        let edges = imageproc::edges::canny(&gray, 50.0, 100.0);
        Ok(DynamicImage::ImageLuma8(edges))
    }

    fn invert_image(&self, img: &DynamicImage) -> Result<DynamicImage> {
        let mut img = img.clone();
        img.invert();
        Ok(img)
    }

    fn blend_images(&self, base: &DynamicImage, overlay: &DynamicImage, opacity: f32) -> Result<DynamicImage> {
        let base_rgb = base.to_rgb8();
        let overlay_rgb = overlay.to_rgb8();
        let (width, height) = base_rgb.dimensions();
        let mut result = image::ImageBuffer::new(width, height);
        
        for (x, y, base_pixel) in base_rgb.enumerate_pixels() {
            let overlay_pixel = overlay_rgb.get_pixel(x, y);
            
            let blended = image::Rgb([
                (base_pixel[0] as f32 * (1.0 - opacity) + overlay_pixel[0] as f32 * opacity) as u8,
                (base_pixel[1] as f32 * (1.0 - opacity) + overlay_pixel[1] as f32 * opacity) as u8,
                (base_pixel[2] as f32 * (1.0 - opacity) + overlay_pixel[2] as f32 * opacity) as u8,
            ]);
            
            result.put_pixel(x, y, blended);
        }
        
        Ok(DynamicImage::ImageRgb8(result))
    }

    fn dodge_blend(&self, base: &DynamicImage, overlay: &DynamicImage) -> Result<DynamicImage> {
        let base_luma = base.to_luma8();
        let overlay_luma = overlay.to_luma8();
        let (width, height) = base_luma.dimensions();
        let mut result = image::ImageBuffer::new(width, height);
        
        for (x, y, base_pixel) in base_luma.enumerate_pixels() {
            let overlay_pixel = overlay_luma.get_pixel(x, y);
            
            let base_val = base_pixel[0] as f32 / 255.0;
            let overlay_val = overlay_pixel[0] as f32 / 255.0;
            
            let dodged = if overlay_val >= 1.0 {
                1.0
            } else {
                (base_val / (1.0 - overlay_val)).min(1.0)
            };
            
            result.put_pixel(x, y, image::Luma([(dodged * 255.0) as u8]));
        }
        
        Ok(DynamicImage::ImageLuma8(result))
    }

    fn screen_blend(&self, base: &DynamicImage, overlay: &DynamicImage, opacity: f32) -> Result<DynamicImage> {
        let base_rgb = base.to_rgb8();
        let overlay_rgb = overlay.to_rgb8();
        let (width, height) = base_rgb.dimensions();
        let mut result = image::ImageBuffer::new(width, height);
        
        for (x, y, base_pixel) in base_rgb.enumerate_pixels() {
            let overlay_pixel = overlay_rgb.get_pixel(x, y);
            
            let screen_blend = |a: u8, b: u8| -> u8 {
                let a_norm = a as f32 / 255.0;
                let b_norm = b as f32 / 255.0;
                let screen = 1.0 - (1.0 - a_norm) * (1.0 - b_norm);
                (screen * 255.0) as u8
            };
            
            let blended_r = screen_blend(base_pixel[0], overlay_pixel[0]);
            let blended_g = screen_blend(base_pixel[1], overlay_pixel[1]);
            let blended_b = screen_blend(base_pixel[2], overlay_pixel[2]);
            
            let final_pixel = image::Rgb([
                (base_pixel[0] as f32 * (1.0 - opacity) + blended_r as f32 * opacity) as u8,
                (base_pixel[1] as f32 * (1.0 - opacity) + blended_g as f32 * opacity) as u8,
                (base_pixel[2] as f32 * (1.0 - opacity) + blended_b as f32 * opacity) as u8,
            ]);
            
            result.put_pixel(x, y, final_pixel);
        }
        
        Ok(DynamicImage::ImageRgb8(result))
    }

    fn adjust_saturation(&self, img: &DynamicImage, factor: f32) -> Result<DynamicImage> {
        let rgb_img = img.to_rgb8();
        let mut result = image::ImageBuffer::new(rgb_img.width(), rgb_img.height());
        
        for (x, y, pixel) in rgb_img.enumerate_pixels() {
            let r = pixel[0] as f32 / 255.0;
            let g = pixel[1] as f32 / 255.0;
            let b = pixel[2] as f32 / 255.0;
            
            // Convert to HSV, adjust saturation, convert back
            let (h, s, v) = self.rgb_to_hsv(r, g, b);
            let new_s = (s * factor).min(1.0);
            let (new_r, new_g, new_b) = self.hsv_to_rgb(h, new_s, v);
            
            result.put_pixel(x, y, image::Rgb([
                (new_r * 255.0) as u8,
                (new_g * 255.0) as u8,
                (new_b * 255.0) as u8,
            ]));
        }
        
        Ok(DynamicImage::ImageRgb8(result))
    }

    fn apply_vignette(&self, img: &DynamicImage, intensity: f32) -> Result<DynamicImage> {
        let rgb_img = img.to_rgb8();
        let (width, height) = rgb_img.dimensions();
        let mut result = image::ImageBuffer::new(width, height);
        
        let center_x = width as f32 / 2.0;
        let center_y = height as f32 / 2.0;
        let max_distance = ((center_x * center_x) + (center_y * center_y)).sqrt();
        
        for (x, y, pixel) in rgb_img.enumerate_pixels() {
            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            let vignette_factor = 1.0 - (distance / max_distance * intensity).min(1.0);
            
            result.put_pixel(x, y, image::Rgb([
                (pixel[0] as f32 * vignette_factor) as u8,
                (pixel[1] as f32 * vignette_factor) as u8,
                (pixel[2] as f32 * vignette_factor) as u8,
            ]));
        }
        
        Ok(DynamicImage::ImageRgb8(result))
    }

    fn apply_color_cast(&self, img: &DynamicImage, r_factor: f32, g_factor: f32, b_factor: f32) -> Result<DynamicImage> {
        let rgb_img = img.to_rgb8();
        let mut result = image::ImageBuffer::new(rgb_img.width(), rgb_img.height());
        
        for (x, y, pixel) in rgb_img.enumerate_pixels() {
            result.put_pixel(x, y, image::Rgb([
                ((pixel[0] as f32 * r_factor).min(255.0)) as u8,
                ((pixel[1] as f32 * g_factor).min(255.0)) as u8,
                ((pixel[2] as f32 * b_factor).min(255.0)) as u8,
            ]));
        }
        
        Ok(DynamicImage::ImageRgb8(result))
    }

    fn quantize_colors(&self, img: &DynamicImage, levels: u8) -> Result<DynamicImage> {
        let rgb_img = img.to_rgb8();
        let mut result = image::ImageBuffer::new(rgb_img.width(), rgb_img.height());
        let step = 255 / levels;
        
        for (x, y, pixel) in rgb_img.enumerate_pixels() {
            result.put_pixel(x, y, image::Rgb([
                (pixel[0] / step) * step,
                (pixel[1] / step) * step,
                (pixel[2] / step) * step,
            ]));
        }
        
        Ok(DynamicImage::ImageRgb8(result))
    }

    fn simple_background_removal(&self, img: &DynamicImage) -> Result<DynamicImage> {
        // Very simple background removal using edge detection
        let edges = self.detect_edges(img)?;
        let dilated = self.dilate_image(&edges)?;
        
        // Create mask from edges
        self.apply_mask(img, &dilated)
    }

    fn simple_style_transfer(&self, img: &DynamicImage, strength: f32) -> Result<DynamicImage> {
        // Simplified style transfer - just apply artistic effects
        let oil_painted = self.apply_oil_painting_effect(img, &std::collections::HashMap::new())?;
        self.blend_images(img, &oil_painted, strength)
    }

    fn simple_panorama_stitch(&self, images: &[DynamicImage], _mode: &StitchMode) -> Result<DynamicImage> {
        if images.is_empty() {
            return Err(anyhow::anyhow!("No images to stitch"));
        }
        
        if images.len() == 1 {
            return Ok(images[0].clone());
        }
        
        // Simple horizontal stitching
        let total_width: u32 = images.iter().map(|img| img.width()).sum();
        let max_height = images.iter().map(|img| img.height()).max().unwrap_or(0);
        
        let mut result = DynamicImage::new_rgb8(total_width, max_height);
        let mut x_offset = 0;
        
        for img in images {
            imageops::overlay(&mut result, img, x_offset as i64, 0);
            x_offset += img.width();
        }
        
        Ok(result)
    }

    fn dilate_image(&self, img: &DynamicImage) -> Result<DynamicImage> {
        // Simple dilation - expand bright areas
        let luma = img.to_luma8();
        let mut result = luma.clone();
        
        for y in 1..luma.height()-1 {
            for x in 1..luma.width()-1 {
                let mut max_val = 0u8;
                
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let pixel = luma.get_pixel((x as i32 + dx) as u32, (y as i32 + dy) as u32);
                        max_val = max_val.max(pixel[0]);
                    }
                }
                
                result.put_pixel(x, y, image::Luma([max_val]));
            }
        }
        
        Ok(DynamicImage::ImageLuma8(result))
    }

    fn apply_mask(&self, img: &DynamicImage, mask: &DynamicImage) -> Result<DynamicImage> {
        let rgba_img = img.to_rgba8();
        let mask_luma = mask.to_luma8();
        let mut result = rgba_img.clone();
        
        for (x, y, pixel) in result.enumerate_pixels_mut() {
            let mask_value = mask_luma.get_pixel(x, y)[0];
            pixel[3] = mask_value; // Set alpha based on mask
        }
        
        Ok(DynamicImage::ImageRgba8(result))
    }

    // Color space conversion helpers
    fn rgb_to_hsv(&self, r: f32, g: f32, b: f32) -> (f32, f32, f32) {
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let h = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if max == g {
            60.0 * (((b - r) / delta) + 2.0)
        } else {
            60.0 * (((r - g) / delta) + 4.0)
        };

        let s = if max == 0.0 { 0.0 } else { delta / max };
        let v = max;

        (h, s, v)
    }

    fn hsv_to_rgb(&self, h: f32, s: f32, v: f32) -> (f32, f32, f32) {
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let (r_prime, g_prime, b_prime) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        (r_prime + m, g_prime + m, b_prime + m)
    }
}