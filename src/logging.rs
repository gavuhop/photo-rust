use log::{LevelFilter, Log, Metadata, Record};
use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
    path::Path,
    sync::{Arc, Mutex},
};

/// Simple text logger implementation with file rotation
pub struct Logger {
    file: Arc<Mutex<File>>,
    level: LevelFilter,
    log_dir: String,
    max_file_size: u64,
    max_files: usize,
}

impl Logger {
    /// Create a new logger instance
    pub fn new(log_dir: &str, level: LevelFilter) -> io::Result<Self> {
        // Create log directory if it doesn't exist
        std::fs::create_dir_all(log_dir)?;
        
        let log_file_path = format!("{}/app.log", log_dir);
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file_path)?;
        
        Ok(Self {
            file: Arc::new(Mutex::new(file)),
            level,
            log_dir: log_dir.to_string(),
            max_file_size: 10 * 1024 * 1024, // 10MB
            max_files: 5,
        })
    }
    
    /// Rotate log files if current file is too large
    fn rotate_if_needed(&self) -> io::Result<()> {
        let log_file_path = format!("{}/app.log", self.log_dir);
        
        if let Ok(metadata) = std::fs::metadata(&log_file_path) {
            if metadata.len() > self.max_file_size {
                self.rotate_log_files()?;
            }
        }
        
        Ok(())
    }
    
    /// Rotate log files by moving existing files
    fn rotate_log_files(&self) -> io::Result<()> {
        let log_file_path = format!("{}/app.log", self.log_dir);
        
        // Remove oldest log file if we have too many
        let oldest_log = format!("{}/app.log.{}", self.log_dir, self.max_files - 1);
        if Path::new(&oldest_log).exists() {
            std::fs::remove_file(&oldest_log)?;
        }
        
        // Shift existing log files
        for i in (1..self.max_files).rev() {
            let src = format!("{}/app.log.{}", self.log_dir, i - 1);
            let dst = format!("{}/app.log.{}", self.log_dir, i);
            if Path::new(&src).exists() {
                std::fs::rename(&src, &dst)?;
            }
        }
        
        // Rename current log file
        let rotated_log = format!("{}/app.log.0", self.log_dir);
        if Path::new(&log_file_path).exists() {
            std::fs::rename(&log_file_path, &rotated_log)?;
        }
        
        // Create new log file
        let new_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file_path)?;
        
        *self.file.lock().unwrap() = new_file;
        
        Ok(())
    }
    
    /// Format log record as simple text
    fn format_log(&self, record: &Record) -> String {
        let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f");
        
        // Simplify target name
        let target = if record.target() == "media_processing_service" {
            "main"
        } else if record.target().contains("::") {
            record.target().split("::").last().unwrap_or(record.target())
        } else {
            record.target()
        };
        
        // Simplify module path
        let module_path = record.module_path().unwrap_or("unknown");
        let short_module = if module_path.contains("::") {
            module_path.split("::").skip(1).collect::<Vec<_>>().join("::")
        } else {
            module_path.to_string()
        };

        format!(
            "[{}] {} [{}] {} - {}\n",
            timestamp,
            record.level(),
            target,
            short_module,
            record.args()
        )
    }
    
    /// Write log to file
    fn write_log(&self, record: &Record) -> io::Result<()> {
        let log_line = self.format_log(record);
        
        let mut file = self.file.lock().unwrap();
        file.write_all(log_line.as_bytes())?;
        file.flush()?;
        
        Ok(())
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }
    
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            // Rotate logs if needed
            if let Err(e) = self.rotate_if_needed() {
                eprintln!("Failed to rotate log files: {}", e);
            }
            
            // Write to file
            if let Err(e) = self.write_log(record) {
                eprintln!("Failed to write log: {}", e);
            }
            
            // Also print to stderr for development
            if cfg!(debug_assertions) {
                eprintln!("{}", self.format_log(record).trim());
            }
        }
    }
    
    fn flush(&self) {
        if let Ok(mut file) = self.file.lock() {
            let _ = file.flush();
        }
    }
}

/// Initialize the custom logger
pub fn init_logger(log_dir: &str, level: LevelFilter) -> io::Result<()> {
    let logger = Logger::new(log_dir, level)?;
    log::set_boxed_logger(Box::new(logger))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    log::set_max_level(level);
    Ok(())
}

/// Log levels for different environments
pub mod levels {
    use log::LevelFilter;
    
    pub const DEVELOPMENT: LevelFilter = LevelFilter::Debug;
    pub const PRODUCTION: LevelFilter = LevelFilter::Info;
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::Level;
    use tempfile::tempdir;
    
    #[test]
    fn test_logger_creation() {
        let temp_dir = tempdir().unwrap();
        let logger = Logger::new(temp_dir.path().to_str().unwrap(), LevelFilter::Debug);
        assert!(logger.is_ok());
    }
    
    #[test]
    fn test_log_formatting() {
        let temp_dir = tempdir().unwrap();
        let logger = Logger::new(temp_dir.path().to_str().unwrap(), LevelFilter::Debug).unwrap();
        
        let record = log::Record::builder()
            .level(Level::Info)
            .target("test_target")
            .args(format_args!("Test message"))
            .file(Some("test.rs"))
            .line(Some(42))
            .build();
        
        let formatted = logger.format_log(&record);
        assert!(formatted.contains("Test message"));
        assert!(formatted.contains("INFO"));
    }
} 