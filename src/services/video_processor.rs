use anyhow::Result;
use ffmpeg_next as ffmpeg;
use log::{error, info, warn};
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::os::unix::process::ExitStatusExt;
use uuid::Uuid;
use crate::models::video::{VideoTranscodeRequest, AudioExtractRequest};

pub struct QualityProfile {
    pub label: &'static str,
    pub resolution: &'static str,
    pub bitrate: &'static str,
}

pub static QUALITY_PROFILES: &[QualityProfile] = &[
    QualityProfile { label: "1080p", resolution: "1920x1080", bitrate: "5M" },
    QualityProfile { label: "720p",  resolution: "1280x720",  bitrate: "2.5M" },
    QualityProfile { label: "480p",  resolution: "854x480",   bitrate: "1M" },
];

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
        
        // Log current working directory
        if let Ok(current_dir) = std::env::current_dir() {
            info!("[{}] Current working directory: {:?}", job_id, current_dir);
        }
        
        // Validate input file exists
        let input_path = std::path::Path::new(&request.input_path);
        info!("[{}] Checking input file: {}", job_id, request.input_path);
        
        // Try to get absolute path
        if let Ok(canonical_path) = input_path.canonicalize() {
            info!("[{}] Input file absolute path: {:?}", job_id, canonical_path);
        } else {
            warn!("[{}] Could not resolve absolute path for: {}", job_id, request.input_path);
        }
        
        if !input_path.exists() {
            error!("[{}] Input file does not exist: {}", job_id, request.input_path);
            return Err(anyhow::anyhow!("Input file not found: {}", request.input_path));
        }
        
        // Check if file is readable
        if let Err(e) = std::fs::File::open(&request.input_path) {
            error!("[{}] Input file is not readable: {} - Error: {}", job_id, request.input_path, e);
            return Err(anyhow::anyhow!("Input file is not readable: {} - {}", request.input_path, e));
        }
        
        info!("[{}] Input file validation passed: {}", job_id, request.input_path);
        
        // Validate output directory exists
        if let Some(parent) = std::path::Path::new(&request.output_path).parent() {
            if !parent.exists() {
                return Err(anyhow::anyhow!("Output directory does not exist: {}", parent.display()));
            }
        }
        
        // Get video duration first
        let duration = self.get_video_duration(&request.input_path).await?;
        info!("[{}] Video duration: {:.2} seconds", job_id, duration);
        
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
        
        // Execute FFmpeg command with real-time output monitoring
        command.stdout(Stdio::piped()).stderr(Stdio::piped());
        
        info!("[{}] Spawning FFmpeg process...", job_id);
        let mut child = command.spawn()?;
        
        // Check if process started successfully
        match child.try_wait() {
            Ok(Some(status)) => {
                let error = format!("FFmpeg process terminated immediately with status: {}", status);
                error!("[{}] {}", job_id, error);
                return Err(anyhow::anyhow!("FFmpeg processing failed: {}", error));
            }
            Ok(None) => {
                info!("[{}] FFmpeg process started successfully", job_id);
            }
            Err(e) => {
                let error = format!("Failed to check FFmpeg process status: {}", e);
                error!("[{}] {}", job_id, error);
                return Err(anyhow::anyhow!("FFmpeg processing failed: {}", error));
            }
        }
        
        let stderr = child.stderr.take().unwrap();
        
        // Monitor FFmpeg progress in real-time
        let reader = BufReader::new(stderr);
        let mut last_progress = 0.0;
        
        for line in reader.lines() {
            if let Ok(line) = line {
                // Parse FFmpeg progress output
                if line.contains("time=") && line.contains("bitrate=") {
                    // Extract time information for progress tracking
                    if let Some(time_str) = line.split("time=").nth(1) {
                        if let Some(time_part) = time_str.split_whitespace().next() {
                            // Parse time format (HH:MM:SS.ms) and calculate percentage
                            if let Some(current_time) = self.parse_ffmpeg_time(time_part) {
                                let progress = (current_time / duration) * 100.0;
                                if progress > last_progress + 5.0 { // Log every 5% progress
                                    info!("[{}] Transcode progress: {:.1}% ({:.1}s/{:.1}s)", 
                                          job_id, progress, current_time, duration);
                                    last_progress = progress;
                                }
                            }
                        }
                    }
                }
                
                // Log important FFmpeg messages
                if line.contains("error") || line.contains("Error") {
                    warn!("[{}] FFmpeg warning: {}", job_id, line);
                }
                
                // Check if process is still running
                if let Ok(Some(_)) = child.try_wait() {
                    break;
                }
            } else {
                // Error reading line, check if process is still running
                if let Ok(Some(_)) = child.try_wait() {
                    break;
                }
            }
        }
        
        // Wait for the process to complete
        let status = child.wait()?;
        
        if status.success() {
            info!("Video transcode completed successfully: {}", job_id);
            Ok(job_id)
        } else {
            let error_msg = if status.code().is_some() {
                format!("FFmpeg process failed with exit code: {}", status)
            } else {
                format!("FFmpeg process terminated by signal: {:?}", status.signal())
            };
            error!("[{}] {}", job_id, error_msg);
            Err(anyhow::anyhow!("FFmpeg processing failed: {}", error_msg))
        }
    }

    /// Parse FFmpeg time format (HH:MM:SS.ms) to seconds
    fn parse_ffmpeg_time(&self, time_str: &str) -> Option<f64> {
        let parts: Vec<&str> = time_str.split(':').collect();
        if parts.len() >= 3 {
            let hours: f64 = parts[0].parse().ok()?;
            let minutes: f64 = parts[1].parse().ok()?;
            let seconds: f64 = parts[2].parse().ok()?;
            
            let total_seconds = hours * 3600.0 + minutes * 60.0 + seconds;
            Some(total_seconds)
        } else {
            None
        }
    }

    /// Get video duration using ffprobe
    async fn get_video_duration(&self, file_path: &str) -> Result<f64> {
        let output = Command::new("ffprobe")
            .arg("-v").arg("quiet")
            .arg("-show_entries").arg("format=duration")
            .arg("-of").arg("csv=p=0")
            .arg(file_path)
            .output()?;
            
        if output.status.success() {
            let duration_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let duration: f64 = duration_str.parse()
                .map_err(|_| anyhow::anyhow!("Failed to parse duration: {}", duration_str))?;
            Ok(duration)
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("Failed to get video duration: {}", error))
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
        
        // Get video duration first
        let duration = self.get_video_duration(&request.input_path).await?;
        info!("[{}] Video duration: {:.2} seconds", job_id, duration);
        
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
        
        // Execute FFmpeg command with real-time output monitoring
        command.stdout(Stdio::piped()).stderr(Stdio::piped());
        
        info!("[{}] Spawning FFmpeg process for audio extraction...", job_id);
        let mut child = command.spawn()?;
        
        // Check if process started successfully
        match child.try_wait() {
            Ok(Some(status)) => {
                let error = format!("FFmpeg process terminated immediately with status: {}", status);
                error!("[{}] {}", job_id, error);
                return Err(anyhow::anyhow!("Audio extraction failed: {}", error));
            }
            Ok(None) => {
                info!("[{}] FFmpeg process for audio extraction started successfully", job_id);
            }
            Err(e) => {
                let error = format!("Failed to check FFmpeg process status: {}", e);
                error!("[{}] {}", job_id, error);
                return Err(anyhow::anyhow!("Audio extraction failed: {}", error));
            }
        }
        
        let stderr = child.stderr.take().unwrap();
        
        // Monitor FFmpeg progress in real-time
        let reader = BufReader::new(stderr);
        let mut last_progress = 0.0;
        
        for line in reader.lines() {
            if let Ok(line) = line {
                // Parse FFmpeg progress output
                if line.contains("time=") && line.contains("bitrate=") {
                    // Extract time information for progress tracking
                    if let Some(time_str) = line.split("time=").nth(1) {
                        if let Some(time_part) = time_str.split_whitespace().next() {
                            // Parse time format (HH:MM:SS.ms) and calculate percentage
                            if let Some(current_time) = self.parse_ffmpeg_time(time_part) {
                                let progress = (current_time / duration) * 100.0;
                                if progress > last_progress + 5.0 { // Log every 5% progress
                                    info!("[{}] Audio extraction progress: {:.1}% ({:.1}s/{:.1}s)", 
                                          job_id, progress, current_time, duration);
                                    last_progress = progress;
                                }
                            }
                        }
                    }
                }
                
                // Log important FFmpeg messages
                if line.contains("error") || line.contains("Error") {
                    warn!("[{}] FFmpeg warning during audio extraction: {}", job_id, line);
                }
                
                // Check if process is still running
                if let Ok(Some(_)) = child.try_wait() {
                    break;
                }
            } else {
                // Error reading line, check if process is still running
                if let Ok(Some(_)) = child.try_wait() {
                    break;
                }
            }
        }
        
        // Wait for the process to complete
        let status = child.wait()?;
        
        if status.success() {
            info!("Audio extraction completed successfully: {}", job_id);
            Ok(job_id)
        } else {
            let error_msg = if status.code().is_some() {
                format!("FFmpeg process failed with exit code: {}", status)
            } else {
                format!("FFmpeg process terminated by signal: {:?}", status.signal())
            };
            error!("[{}] {}", job_id, error_msg);
            Err(anyhow::anyhow!("Audio extraction failed: {}", error_msg))
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
        
        // Get audio duration first
        let duration = self.get_video_duration(input_path).await?;
        info!("[{}] Audio duration: {:.2} seconds", job_id, duration);
        
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
        
        // Execute FFmpeg command with real-time output monitoring
        command.stdout(Stdio::piped()).stderr(Stdio::piped());
        
        info!("[{}] Spawning FFmpeg process for audio transcode...", job_id);
        let mut child = command.spawn()?;
        
        // Check if process started successfully
        match child.try_wait() {
            Ok(Some(status)) => {
                let error = format!("FFmpeg process terminated immediately with status: {}", status);
                error!("[{}] {}", job_id, error);
                return Err(anyhow::anyhow!("Audio transcode failed: {}", error));
            }
            Ok(None) => {
                info!("[{}] FFmpeg process for audio transcode started successfully", job_id);
            }
            Err(e) => {
                let error = format!("Failed to check FFmpeg process status: {}", e);
                error!("[{}] {}", job_id, error);
                return Err(anyhow::anyhow!("Audio transcode failed: {}", error));
            }
        }
        
        let stderr = child.stderr.take().unwrap();
        
        // Monitor FFmpeg progress in real-time
        let reader = BufReader::new(stderr);
        let mut last_progress = 0.0;
        
        for line in reader.lines() {
            if let Ok(line) = line {
                // Parse FFmpeg progress output
                if line.contains("time=") && line.contains("bitrate=") {
                    // Extract time information for progress tracking
                    if let Some(time_str) = line.split("time=").nth(1) {
                        if let Some(time_part) = time_str.split_whitespace().next() {
                            // Parse time format (HH:MM:SS.ms) and calculate percentage
                            if let Some(current_time) = self.parse_ffmpeg_time(time_part) {
                                let progress = (current_time / duration) * 100.0;
                                if progress > last_progress + 5.0 { // Log every 5% progress
                                    info!("[{}] Audio transcode progress: {:.1}% ({:.1}s/{:.1}s)", 
                                          job_id, progress, current_time, duration);
                                    last_progress = progress;
                                }
                            }
                        }
                    }
                }
                
                // Log important FFmpeg messages
                if line.contains("error") || line.contains("Error") {
                    warn!("[{}] FFmpeg warning during audio transcode: {}", job_id, line);
                }
                
                // Check if process is still running
                if let Ok(Some(_)) = child.try_wait() {
                    break;
                }
            } else {
                // Error reading line, check if process is still running
                if let Ok(Some(_)) = child.try_wait() {
                    break;
                }
            }
        }
        
        // Wait for the process to complete
        let status = child.wait()?;
        
        if status.success() {
            info!("Audio transcode completed successfully: {}", job_id);
            Ok(job_id)
        } else {
            let error_msg = if status.code().is_some() {
                format!("FFmpeg process failed with exit code: {}", status)
            } else {
                format!("FFmpeg process terminated by signal: {:?}", status.signal())
            };
            error!("[{}] {}", job_id, error_msg);
            Err(anyhow::anyhow!("Audio transcode failed: {}", error_msg))
        }
    }

    /// Transcode input video to multiple qualities in parallel (for adaptive streaming)
    pub async fn transcode_multi_quality(
        &self,
        input_path: &str,
        output_prefix: &str,
        codec: &str,
        format: &str,
    ) -> Result<Vec<String>> {
        use tokio::task;
        let mut handles = vec![];
        for profile in QUALITY_PROFILES {
            let input = input_path.to_string();
            let output = format!("{output_prefix}_{}.mp4", profile.label);
            let codec = codec.to_string();
            let format = format.to_string();
            let res = profile.resolution.to_string();
            let bitrate = profile.bitrate.to_string();

            handles.push(task::spawn(async move {
                let mut cmd = Command::new("ffmpeg");
                cmd.arg("-y")
                    .arg("-i").arg(&input)
                    .arg("-s").arg(&res)
                    .arg("-b:v").arg(&bitrate)
                    .arg("-c:v").arg(&codec)
                    .arg(&output);
                let status = cmd.status().expect("failed to run ffmpeg");
                if status.success() {
                    Ok(output)
                } else {
                    Err(format!("Transcode failed for {}", output))
                }
            }));
        }
        let mut results = vec![];
        for handle in handles {
            match handle.await {
                Ok(Ok(path)) => results.push(path),
                Ok(Err(e)) => return Err(anyhow::anyhow!(e)),
                Err(e) => return Err(anyhow::anyhow!("Task join error: {e}")),
            }
        }
        Ok(results)
    }

    /// Package multiple quality files into HLS segments and master playlist
    pub async fn package_hls(
        &self,
        outputs: &[String],
        output_dir: &str,
        master_playlist: &str,
    ) -> Result<()> {
        use std::fs::File;
        use std::io::Write;
        let mut variant_playlists = vec![];
        let mut master_content = String::new();
        master_content.push_str("#EXTM3U\n");

        for output in outputs {
            // Tạo tên playlist cho từng chất lượng
            let label = output
                .split('_')
                .last()
                .unwrap_or("unknown").replace(".mp4", "");
            let playlist = format!("{}/{}.m3u8", output_dir, label);
            let segment_pattern = format!("{}/{}_segment_%03d.ts", output_dir, label);

            // Đóng gói từng file thành HLS
            let status = Command::new("ffmpeg")
                .arg("-y")
                .arg("-i").arg(output)
                .arg("-c:v").arg("copy")
                .arg("-c:a").arg("aac")
                .arg("-f").arg("hls")
                .arg("-hls_time").arg("4")
                .arg("-hls_playlist_type").arg("vod")
                .arg("-hls_segment_filename").arg(&segment_pattern)
                .arg(&playlist)
                .status()?;
            if !status.success() {
                return Err(anyhow::anyhow!("Failed to package HLS for {}", output));
            }
            // Thêm vào master playlist
            master_content.push_str(&format!(
                "#EXT-X-STREAM-INF:BANDWIDTH={},RESOLUTION={}\n{}\n",
                match label.as_str() {
                    "1080p" => 5000000,
                    "720p" => 2500000,
                    "480p" => 1000000,
                    _ => 500000,
                },
                match label.as_str() {
                    "1080p" => "1920x1080",
                    "720p" => "1280x720",
                    "480p" => "854x480",
                    _ => "640x360",
                },
                format!("{}.m3u8", label)
            ));
            variant_playlists.push(playlist);
        }
        // Ghi master playlist
        let mut file = File::create(format!("{}/{}", output_dir, master_playlist))?;
        file.write_all(master_content.as_bytes())?;
        Ok(())
    }
} 