use anyhow::Result;
use std::path::Path;
use uuid::Uuid;

use crate::models::{BatchRequest, ProcessingResult, ProcessingStatus, OptimizationResult};
use crate::services::{ImageProcessor, PerformanceMonitor};
use crate::transformations::TransformationService;
use crate::optimization::OptimizationService;

pub struct BatchService;

impl BatchService {
    pub fn new() -> Self {
        Self
    }

    /// Batch resize images
    pub async fn batch_resize(&self, req: &BatchRequest) -> Result<Vec<ProcessingResult>> {
        let monitor = PerformanceMonitor::new();
        let mut results = Vec::new();
        
        let files = self.get_image_files(&req.input_directory, req.file_pattern.as_deref())?;
        let transformation_service = TransformationService::new();
        
        for file_path in files {
            let job_id = Uuid::new_v4();
            let file_name = Path::new(&file_path)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("image");
            
            let output_path = format!("{}/resized_{}.jpg", req.output_directory, file_name);
            
            // Extract resize parameters from operations
            if let Some(operation) = req.operations.first() {
                if operation.operation_type == "resize" {
                    let width = operation.parameters.get("width")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(800) as u32;
                    let height = operation.parameters.get("height")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(600) as u32;
                    
                    let resize_req = crate::models::ResizeRequest {
                        input_path: file_path.clone(),
                        output_path: output_path.clone(),
                        width,
                        height,
                        mode: crate::models::ResizeMode::Fit,
                        quality: Some(85),
                        preserve_aspect_ratio: Some(true),
                    };
                    
                    match transformation_service.resize_image(&resize_req).await {
                        Ok(result) => results.push(result),
                        Err(e) => {
                            results.push(ProcessingResult {
                                job_id,
                                status: ProcessingStatus::Failed,
                                input_path: file_path,
                                output_path: None,
                                processing_time_ms: Some(monitor.elapsed_ms()),
                                error_message: Some(e.to_string()),
                                metadata: None,
                            });
                        }
                    }
                }
            }
        }
        
        Ok(results)
    }

    /// Batch optimize images
    pub async fn batch_optimize(&self, req: &BatchRequest) -> Result<Vec<OptimizationResult>> {
        let files = self.get_image_files(&req.input_directory, req.file_pattern.as_deref())?;
        let optimization_service = OptimizationService::new();
        
        let mut results = Vec::new();
        
        for file_path in files {
            let file_name = Path::new(&file_path)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("image");
            
            let output_path = format!("{}/optimized_{}.jpg", req.output_directory, file_name);
            
            // Extract quality from operations
            let quality = req.operations.first()
                .and_then(|op| op.parameters.get("quality"))
                .and_then(|v| v.as_u64())
                .unwrap_or(85) as u8;
            
            match optimization_service.compress_image(&file_path, &output_path, quality).await {
                Ok(result) => results.push(result),
                Err(e) => {
                    log::error!("Failed to optimize {}: {}", file_path, e);
                }
            }
        }
        
        Ok(results)
    }

    /// Batch convert images
    pub async fn batch_convert(&self, req: &BatchRequest) -> Result<Vec<ProcessingResult>> {
        let files = self.get_image_files(&req.input_directory, req.file_pattern.as_deref())?;
        let optimization_service = OptimizationService::new();
        
        let mut results = Vec::new();
        
        for file_path in files {
            let file_name = Path::new(&file_path)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("image");
            
            // Extract target format from operations
            let format = req.operations.first()
                .and_then(|op| op.parameters.get("format"))
                .and_then(|v| v.as_str())
                .unwrap_or("jpeg");
            
            let output_path = format!("{}/converted_{}.{}", req.output_directory, file_name, format);
            
            match optimization_service.convert_format(&file_path, &output_path, format).await {
                Ok(result) => results.push(result),
                Err(e) => {
                    log::error!("Failed to convert {}: {}", file_path, e);
                }
            }
        }
        
        Ok(results)
    }

    /// Get list of image files from directory
    fn get_image_files(&self, directory: &str, pattern: Option<&str>) -> Result<Vec<String>> {
        let mut files = Vec::new();
        
        for entry in std::fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                let path_str = path.to_string_lossy().to_string();
                
                // Check if it's an image file
                if ImageProcessor::is_supported_format(&path_str) {
                    // Apply pattern filter if specified
                    if let Some(pattern) = pattern {
                        if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                            if filename.contains(pattern) {
                                files.push(path_str);
                            }
                        }
                    } else {
                        files.push(path_str);
                    }
                }
            }
        }
        
        Ok(files)
    }

    /// Process files in parallel
    pub async fn process_parallel<F, T>(&self, files: Vec<String>, max_concurrent: usize, processor: F) -> Vec<T>
    where
        F: Fn(String) -> T + Send + Sync,
        T: Send,
    {
        use futures::future::join_all;
        
        let chunks: Vec<_> = files.chunks(max_concurrent).collect();
        let mut results = Vec::new();
        
        for chunk in chunks {
            let futures: Vec<_> = chunk.iter()
                .map(|file| {
                    let file = file.clone();
                    let processor = &processor;
                    async move { processor(file) }
                })
                .collect();
            
            let chunk_results = join_all(futures).await;
            results.extend(chunk_results);
        }
        
        results
    }
}