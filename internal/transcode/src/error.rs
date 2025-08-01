use thiserror::Error;

#[derive(Error, Debug)]
pub enum TranscodeError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
    
    #[error("FFmpeg error: {0}")]
    FFmpegError(String),
    
    #[error("Image processing error: {0}")]
    ImageProcessingError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<image::ImageError> for TranscodeError {
    fn from(err: image::ImageError) -> Self {
        TranscodeError::ImageProcessingError(err.to_string())
    }
}

impl From<anyhow::Error> for TranscodeError {
    fn from(err: anyhow::Error) -> Self {
        TranscodeError::Unknown(err.to_string())
    }
} 