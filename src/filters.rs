use std::path::Path;
use anyhow::Result;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use imageproc::geometric_transformations::{rotate, Interpolation};
use photon_rs::{PhotonImage, filters, effects, colour_spaces, transform};

use crate::models::*;

pub struct ImageProcessor;

impl ImageProcessor {
    pub fn new() -> Self {
        Self
    }

    pub async fn apply_filter(&self, request: &ImageFilterRequest) -> Result<Uuid> {
        let job_id = uuid::Uuid::new_v4();
        
        log::info!("Starting filter job {}: {} -> {} ({})", 
                   job_id, request.input_path, request.output_path, request.filter_type);
        
        let img = image::open(&request.input_path)?;
        
        let processed = match request.filter_type.as_str() {
            "blur" => self.apply_blur(img, request.intensity.unwrap_or(1.0))?,
            "sharpen" => self.apply_sharpen(img, request.intensity.unwrap_or(1.0))?,
            "sepia" => self.apply_sepia(img)?,
            "grayscale" => self.apply_grayscale(img)?,
            "vintage" => self.apply_vintage(img)?,
            "brightness" => self.adjust_brightness(img, request.intensity.unwrap_or(1.0))?,
            "contrast" => self.adjust_contrast(img, request.intensity.unwrap_or(1.0))?,
            "saturation" => self.adjust_saturation(img, request.intensity.unwrap_or(1.0))?,
            "vignette" => self.apply_vignette(img, request.intensity.unwrap_or(0.5))?,
            "emboss" => self.apply_emboss(img)?,
            "edge_detect" => self.apply_edge_detection(img)?,
            _ => return Err(anyhow::anyhow!("Unknown filter type: {}", request.filter_type)),
        };
        
        processed.save(&request.output_path)?;
        
        log::info!("Filter job {} completed", job_id);
        Ok(job_id)
    }

    pub async fn add_watermark(&self, request: &WatermarkRequest) -> Result<Uuid> {
        let job_id = uuid::Uuid::new_v4();
        
        log::info!("Starting watermark job {}: {} + {} -> {}", 
                   job_id, request.input_path, request.watermark_path, request.output_path);
        
        let mut base_img = image::open(&request.input_path)?;
        let watermark = image::open(&request.watermark_path)?;
        
        let (base_width, base_height) = base_img.dimensions();
        let (wm_width, wm_height) = watermark.dimensions();
        
        // Scale watermark if requested
        let scaled_watermark = if let Some(scale) = request.scale {
            let new_width = (wm_width as f32 * scale) as u32;
            let new_height = (wm_height as f32 * scale) as u32;
            watermark.resize(new_width, new_height, image::imageops::FilterType::Lanczos3)
        } else {
            watermark
        };
        
        let (wm_width, wm_height) = scaled_watermark.dimensions();
        
        // Calculate position
        let (x, y) = self.calculate_watermark_position(
            &request.position, 
            base_width, 
            base_height, 
            wm_width, 
            wm_height
        );
        
        // Apply watermark with opacity
        let opacity = request.opacity.unwrap_or(0.5);
        let watermark_rgba = scaled_watermark.to_rgba8();
        
        // Blend watermark onto base image
        let mut base_rgba = base_img.to_rgba8();
        
        for (dx, dy, pixel) in watermark_rgba.enumerate_pixels() {
            let px = x + dx;
            let py = y + dy;
            
            if px < base_width && py < base_height {
                if let Some(base_pixel) = base_rgba.get_pixel_mut_checked(px, py) {
                    let alpha = (pixel[3] as f32 / 255.0) * opacity;
                    
                    for i in 0..3 {
                        let base_val = base_pixel[i] as f32;
                        let overlay_val = pixel[i] as f32;
                        let blended = base_val * (1.0 - alpha) + overlay_val * alpha;
                        base_pixel[i] = blended as u8;
                    }
                }
            }
        }
        
        let result = DynamicImage::ImageRgba8(base_rgba);
        result.save(&request.output_path)?;
        
        log::info!("Watermark job {} completed", job_id);
        Ok(job_id)
    }

    pub async fn batch_process(&self, request: &BatchProcessRequest) -> Result<Vec<Uuid>> {
        let mut job_ids = Vec::new();
        
        for input_path in &request.input_paths {
            let filename = Path::new(input_path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("processed");
            
            let output_path = format!("{}/{}", request.output_directory, filename);
            
            for operation in &request.operations {
                let job_id = self.process_single_operation(input_path, &output_path, operation).await?;
                job_ids.push(job_id);
            }
        }
        
        Ok(job_ids)
    }

    async fn process_single_operation(&self, input_path: &str, output_path: &str, operation: &ProcessOperation) -> Result<Uuid> {
        match operation.operation_type.as_str() {
            "resize" => {
                let width = operation.parameters.get("width")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(800) as u32;
                let height = operation.parameters.get("height")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(600) as u32;
                
                self.resize_image(input_path, output_path, width, height).await
            },
            "rotate" => {
                let angle = operation.parameters.get("angle")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(90.0) as f32;
                
                self.rotate_image(input_path, output_path, angle).await
            },
            "crop" => {
                let x = operation.parameters.get("x").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                let y = operation.parameters.get("y").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                let width = operation.parameters.get("width").and_then(|v| v.as_u64()).unwrap_or(100) as u32;
                let height = operation.parameters.get("height").and_then(|v| v.as_u64()).unwrap_or(100) as u32;
                
                self.crop_image(input_path, output_path, x, y, width, height).await
            },
            _ => Err(anyhow::anyhow!("Unknown operation: {}", operation.operation_type))
        }
    }

    async fn resize_image(&self, input_path: &str, output_path: &str, width: u32, height: u32) -> Result<Uuid> {
        let job_id = uuid::Uuid::new_v4();
        let img = image::open(input_path)?;
        let resized = img.resize(width, height, image::imageops::FilterType::Lanczos3);
        resized.save(output_path)?;
        Ok(job_id)
    }

    async fn rotate_image(&self, input_path: &str, output_path: &str, angle: f32) -> Result<Uuid> {
        let job_id = uuid::Uuid::new_v4();
        let img = image::open(input_path)?;
        
        // Convert angle to radians
        let radians = angle * std::f32::consts::PI / 180.0;
        
        let (width, height) = img.dimensions();
        let rotated = rotate(&img.to_luma8(), 
                            (width as f32 / 2.0, height as f32 / 2.0), 
                            radians, 
                            Interpolation::Bilinear, 
                            image::Luma([255u8]));
        
        DynamicImage::ImageLuma8(rotated).save(output_path)?;
        Ok(job_id)
    }

    async fn crop_image(&self, input_path: &str, output_path: &str, x: u32, y: u32, width: u32, height: u32) -> Result<Uuid> {
        let job_id = uuid::Uuid::new_v4();
        let img = image::open(input_path)?;
        let cropped = img.crop_imm(x, y, width, height);
        cropped.save(output_path)?;
        Ok(job_id)
    }

    // Filter implementations
    fn apply_blur(&self, img: DynamicImage, intensity: f32) -> Result<DynamicImage> {
        let sigma = intensity * 2.0;
        Ok(img.blur(sigma))
    }

    fn apply_sharpen(&self, img: DynamicImage, intensity: f32) -> Result<DynamicImage> {
        let kernel = [
            0.0, -intensity, 0.0,
            -intensity, 1.0 + 4.0 * intensity, -intensity,
            0.0, -intensity, 0.0,
        ];
        
        let filtered = imageproc::filter::filter3x3(&img.to_luma8(), &kernel);
        Ok(DynamicImage::ImageLuma8(filtered))
    }

    fn apply_sepia(&self, img: DynamicImage) -> Result<DynamicImage> {
        let rgb_img = img.to_rgb8();
        let mut sepia_img = ImageBuffer::new(rgb_img.width(), rgb_img.height());
        
        for (x, y, pixel) in rgb_img.enumerate_pixels() {
            let r = pixel[0] as f32;
            let g = pixel[1] as f32;
            let b = pixel[2] as f32;
            
            let new_r = ((r * 0.393) + (g * 0.769) + (b * 0.189)).min(255.0) as u8;
            let new_g = ((r * 0.349) + (g * 0.686) + (b * 0.168)).min(255.0) as u8;
            let new_b = ((r * 0.272) + (g * 0.534) + (b * 0.131)).min(255.0) as u8;
            
            sepia_img.put_pixel(x, y, image::Rgb([new_r, new_g, new_b]));
        }
        
        Ok(DynamicImage::ImageRgb8(sepia_img))
    }

    fn apply_grayscale(&self, img: DynamicImage) -> Result<DynamicImage> {
        Ok(img.grayscale())
    }

    fn apply_vintage(&self, img: DynamicImage) -> Result<DynamicImage> {
        // Apply sepia + vignette + slight blur for vintage effect
        let sepia = self.apply_sepia(img)?;
        let vignette = self.apply_vignette(sepia, 0.3)?;
        self.apply_blur(vignette, 0.5)
    }

    fn adjust_brightness(&self, img: DynamicImage, factor: f32) -> Result<DynamicImage> {
        let adjustment = ((factor - 1.0) * 100.0) as i32;
        Ok(img.brighten(adjustment))
    }

    fn adjust_contrast(&self, img: DynamicImage, factor: f32) -> Result<DynamicImage> {
        let rgb_img = img.to_rgb8();
        let mut contrast_img = ImageBuffer::new(rgb_img.width(), rgb_img.height());
        
        for (x, y, pixel) in rgb_img.enumerate_pixels() {
            let mut new_pixel = [0u8; 3];
            
            for i in 0..3 {
                let val = pixel[i] as f32;
                let adjusted = ((val - 128.0) * factor + 128.0).max(0.0).min(255.0) as u8;
                new_pixel[i] = adjusted;
            }
            
            contrast_img.put_pixel(x, y, image::Rgb(new_pixel));
        }
        
        Ok(DynamicImage::ImageRgb8(contrast_img))
    }

    fn adjust_saturation(&self, img: DynamicImage, factor: f32) -> Result<DynamicImage> {
        let rgb_img = img.to_rgb8();
        let mut sat_img = ImageBuffer::new(rgb_img.width(), rgb_img.height());
        
        for (x, y, pixel) in rgb_img.enumerate_pixels() {
            let r = pixel[0] as f32 / 255.0;
            let g = pixel[1] as f32 / 255.0;
            let b = pixel[2] as f32 / 255.0;
            
            // Convert to HSV
            let max = r.max(g).max(b);
            let min = r.min(g).min(b);
            let delta = max - min;
            
            let v = max;
            let s = if max == 0.0 { 0.0 } else { delta / max };
            let h = if delta == 0.0 {
                0.0
            } else if max == r {
                60.0 * (((g - b) / delta) % 6.0)
            } else if max == g {
                60.0 * (((b - r) / delta) + 2.0)
            } else {
                60.0 * (((r - g) / delta) + 4.0)
            };
            
            // Adjust saturation
            let new_s = (s * factor).min(1.0);
            
            // Convert back to RGB
            let c = v * new_s;
            let x_val = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
            let m = v - c;
            
            let (r_prime, g_prime, b_prime) = if h < 60.0 {
                (c, x_val, 0.0)
            } else if h < 120.0 {
                (x_val, c, 0.0)
            } else if h < 180.0 {
                (0.0, c, x_val)
            } else if h < 240.0 {
                (0.0, x_val, c)
            } else if h < 300.0 {
                (x_val, 0.0, c)
            } else {
                (c, 0.0, x_val)
            };
            
            let new_r = ((r_prime + m) * 255.0) as u8;
            let new_g = ((g_prime + m) * 255.0) as u8;
            let new_b = ((b_prime + m) * 255.0) as u8;
            
            sat_img.put_pixel(x, y, image::Rgb([new_r, new_g, new_b]));
        }
        
        Ok(DynamicImage::ImageRgb8(sat_img))
    }

    fn apply_vignette(&self, img: DynamicImage, intensity: f32) -> Result<DynamicImage> {
        let rgb_img = img.to_rgb8();
        let (width, height) = rgb_img.dimensions();
        let mut vignette_img = ImageBuffer::new(width, height);
        
        let center_x = width as f32 / 2.0;
        let center_y = height as f32 / 2.0;
        let max_distance = ((center_x * center_x) + (center_y * center_y)).sqrt();
        
        for (x, y, pixel) in rgb_img.enumerate_pixels() {
            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            let vignette_factor = 1.0 - (distance / max_distance * intensity);
            let vignette_factor = vignette_factor.max(0.0).min(1.0);
            
            let new_r = (pixel[0] as f32 * vignette_factor) as u8;
            let new_g = (pixel[1] as f32 * vignette_factor) as u8;
            let new_b = (pixel[2] as f32 * vignette_factor) as u8;
            
            vignette_img.put_pixel(x, y, image::Rgb([new_r, new_g, new_b]));
        }
        
        Ok(DynamicImage::ImageRgb8(vignette_img))
    }

    fn apply_emboss(&self, img: DynamicImage) -> Result<DynamicImage> {
        let kernel = [
            -2.0, -1.0, 0.0,
            -1.0, 1.0, 1.0,
            0.0, 1.0, 2.0,
        ];
        
        let filtered = imageproc::filter::filter3x3(&img.to_luma8(), &kernel);
        Ok(DynamicImage::ImageLuma8(filtered))
    }

    fn apply_edge_detection(&self, img: DynamicImage) -> Result<DynamicImage> {
        let edges = imageproc::edges::canny(&img.to_luma8(), 50.0, 100.0);
        Ok(DynamicImage::ImageLuma8(edges))
    }

    fn calculate_watermark_position(&self, position: &str, base_width: u32, base_height: u32, wm_width: u32, wm_height: u32) -> (u32, u32) {
        match position {
            "top-left" => (10, 10),
            "top-center" => ((base_width - wm_width) / 2, 10),
            "top-right" => (base_width - wm_width - 10, 10),
            "center-left" => (10, (base_height - wm_height) / 2),
            "center" => ((base_width - wm_width) / 2, (base_height - wm_height) / 2),
            "center-right" => (base_width - wm_width - 10, (base_height - wm_height) / 2),
            "bottom-left" => (10, base_height - wm_height - 10),
            "bottom-center" => ((base_width - wm_width) / 2, base_height - wm_height - 10),
            "bottom-right" => (base_width - wm_width - 10, base_height - wm_height - 10),
            _ => ((base_width - wm_width) / 2, (base_height - wm_height) / 2), // default to center
        }
    }
}