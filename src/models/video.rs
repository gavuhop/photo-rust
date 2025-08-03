use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct VideoTranscodeRequest {
    pub input_path: String,
    pub output_path: String,
    pub format: Option<String>,
    pub codec: Option<String>,
    pub bitrate: Option<String>,
    pub resolution: Option<String>,
    pub fps: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct VideoTranscodeResponse {
    pub job_id: String,
    pub status: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct JobStatusResponse {
    pub job_id: String,
    pub status: String,
    pub progress: Option<f32>,
    pub result: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AudioExtractRequest {
    pub input_path: String,
    pub output_path: String,
    pub format: Option<String>,
    pub bitrate: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct VideoInfoRequest {
    pub file_path: String,
} 