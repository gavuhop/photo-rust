# Photo-Go Media Processing Service

Một service xử lý media (hình ảnh, video, audio) được viết bằng Rust với Actix-web framework.

## 🚀 Tính năng đã triển khai

### ✅ Core Infrastructure
- **Web Server**: Actix-web framework với async/await
- **Health Check**: Endpoint `/health` để kiểm tra trạng thái service
- **Logging**: Structured logging với log crate
- **Error Handling**: Custom error types và proper error propagation
- **Configuration**: Environment-based configuration (PORT, etc.)

### ✅ Docker Support
- **Production Dockerfile**: Multi-stage build với optimization
- **Development Dockerfile**: Hot reloading cho development
- **Docker Compose**: Orchestration cho development và production
- **Makefile**: Simplified commands cho build, run, cleanup
- **Documentation**: Comprehensive Docker setup guide (DOCKER.md)

### ✅ Image Processing Modules (Core Logic Implemented)
- **Image Loading/Saving**: Support cho multiple formats (JPEG, PNG, WebP, GIF)
- **Image Validation**: Input/output validation và format checking
- **Performance Monitoring**: Processing time tracking
- **File Operations**: Directory creation, file size calculation

### ✅ Image Processing Features (Core Logic Available)
- **Resize Operations**: Multiple resize modes (fit, crop, exact)
- **Filter Applications**: Basic image filters (Sepia, Grayscale, Blur)
- **Watermark Support**: Text và image watermarking
- **Quality Assessment**: Image quality analysis và enhancement
- **Format Conversion**: Cross-format image conversion
- **Optimization**: Image compression và optimization
- **Transformations**: Rotate, crop, flip operations
- **Effects**: Artistic effects, style transfer, panorama stitching
- **Batch Processing**: Parallel processing capabilities

### ✅ Video/Audio Processing (Core Logic Available)
- **FFmpeg Integration**: Video transcoding với FFmpeg
- **Metadata Extraction**: Media file metadata analysis
- **Stream Analysis**: Video/audio stream detection
- **Format Support**: Multiple video/audio formats
- **Audio Processing**: Audio extraction, normalization, analysis

### ✅ AI/ML Features (Core Logic Available)
- **Object Detection**: Computer vision object detection
- **Face Detection**: Facial recognition capabilities
- **Color Analysis**: Advanced color analysis algorithms
- **Image Enhancement**: AI-powered image enhancement
- **Content Safety**: Content moderation analysis
- **Text Extraction**: OCR capabilities
- **Scene Classification**: Image scene classification
- **Face Recognition**: Face embedding và comparison

## 🏗️ Architecture

```
photo-rust/
├── src/                         # Main service source code
│   ├── main.rs                 # Web server entry point
│   ├── handlers.rs             # HTTP request handlers
│   ├── models.rs               # Data structures và request/response types
│   ├── services.rs             # Core business logic (transcoding, processing)
│   ├── image_processing.rs     # Image processing utilities
│   ├── filters.rs              # Image filter implementations
│   ├── transformations.rs      # Image transformation logic
│   ├── effects.rs              # Special effects và AI-powered features
│   ├── watermark.rs            # Watermark functionality
│   ├── quality.rs              # Quality assessment và enhancement
│   ├── optimization.rs         # Image optimization và compression
│   ├── metadata.rs             # Media metadata handling
│   ├── batch.rs                # Batch processing capabilities
│   ├── ai.rs                   # AI/ML features (object detection, face recognition)
│   └── error.rs                # Error handling
├── Cargo.toml                  # Rust dependencies
├── Dockerfile                  # Production Docker image
├── Dockerfile.dev              # Development Docker image
├── docker-compose.yml          # Production orchestration
├── docker-compose.dev.yml      # Development orchestration
├── Makefile                    # Build automation
├── .dockerignore               # Docker build optimization
└── DOCKER.md                   # Docker documentation
```

## 🛠️ Technology Stack

### Backend
- **Language**: Rust 1.70+
- **Framework**: Actix-web 4.11
- **Async Runtime**: Tokio
- **Serialization**: Serde + Serde JSON
- **Logging**: log + env_logger
- **Image Processing**: image + imageproc + photon-rs
- **Video Processing**: FFmpeg integration
- **UUID Generation**: uuid crate
- **AI/ML**: OpenCV integration cho computer vision

### Infrastructure
- **Containerization**: Docker
- **Orchestration**: Docker Compose
- **Build System**: Cargo (Rust)
- **Development**: Hot reloading support

## 🚀 Quick Start

### Development (Local)
```bash
# Build and run from root directory
cargo run
```

### Development (Docker with Hot Reload)
```bash
# Using the new development script (recommended)
./dev.sh start

# Or using Makefile
make dev-hot-reload

# Or traditional way
make dev
```

### Development Commands
```bash
# Start with hot reload
./dev.sh start

# View logs
./dev.sh logs

# Open shell in container
./dev.sh shell

# Check health
./dev.sh health

# Stop development
./dev.sh stop

# Restart development
./dev.sh restart
```

### Production (Docker)
```bash
make prod
```

## 📡 API Endpoints

### ✅ Implemented Endpoints

#### Health Check
- `GET /health` - Service health status
  ```json
  {
    "status": "healthy",
    "service": "photo-go-media-processing",
    "version": "1.0.0",
    "timestamp": "2024-01-01T00:00:00Z"
  }
  ```

#### Image Processing Endpoints
- `POST /api/v1/image/resize` - Resize images with multiple modes
- `POST /api/v1/image/rotate` - Rotate images by specified angle
- `POST /api/v1/image/crop` - Crop images to specified dimensions
- `POST /api/v1/image/filter` - Apply various image filters
- `POST /api/v1/image/watermark` - Add text or image watermarks
- `POST /api/v1/image/optimize` - Optimize and compress images
- `POST /api/v1/image/convert` - Convert images between formats

#### Video Processing Endpoints
- `POST /api/v1/video/transcode` - Transcode videos to different formats
- `POST /api/v1/audio/transcode` - Transcode audio files
- `POST /api/v1/audio/extract` - Extract audio from video files

#### AI/ML Endpoints
- `POST /api/v1/ai/detect-objects` - Detect objects in images
- `POST /api/v1/ai/detect-faces` - Detect and analyze faces
- `POST /api/v1/ai/analyze-colors` - Analyze dominant colors
- `POST /api/v1/ai/content-safety` - Content safety analysis
- `POST /api/v1/ai/extract-text` - Extract text using OCR
- `POST /api/v1/ai/classify-scene` - Classify image scenes

#### Quality Enhancement Endpoints
- `POST /api/v1/quality/assess` - Assess image quality
- `POST /api/v1/quality/enhance` - Auto-enhance images
- `POST /api/v1/quality/reduce-noise` - Reduce image noise

#### Effects Endpoints
- `POST /api/v1/effects/apply` - Apply artistic effects
- `POST /api/v1/effects/remove-background` - Remove image backgrounds
- `POST /api/v1/effects/style-transfer` - Apply style transfer
- `POST /api/v1/effects/panorama` - Stitch panorama images

#### Metadata Endpoints
- `POST /api/v1/metadata/extract` - Extract media metadata
- `POST /api/v1/metadata/analyze-video` - Analyze video files

#### Batch Processing Endpoints
- `POST /api/v1/batch/resize` - Batch resize multiple images
- `POST /api/v1/batch/optimize` - Batch optimize multiple images
- `POST /api/v1/batch/convert` - Batch convert multiple images

#### Job Status Endpoint
- `GET /api/v1/jobs/{job_id}` - Get job processing status

### 📋 Request/Response Examples

#### Image Resize
```bash
POST /api/v1/image/resize
Content-Type: application/json

{
  "input_path": "/path/to/input.jpg",
  "output_path": "/path/to/output.jpg",
  "width": 800,
  "height": 600,
  "mode": "fit"
}
```

#### Object Detection
```bash
POST /api/v1/ai/detect-objects
Content-Type: application/json

{
  "image_path": "/path/to/image.jpg"
}
```

Response:
```json
{
  "objects": [
    {
      "name": "person",
      "confidence": 0.95,
      "bounding_box": {
        "x": 100,
        "y": 150,
        "width": 200,
        "height": 400
      }
    }
  ],
  "count": 1
}
```

#### Video Transcoding
```bash
POST /api/v1/video/transcode
Content-Type: application/json

{
  "input_path": "/path/to/input.mp4",
  "output_path": "/path/to/output.webm",
  "format": "webm",
  "codec": "libvpx",
  "bitrate": "1M"
}
```

#### Batch Processing
```bash
POST /api/v1/batch/resize
Content-Type: application/json

{
  "input_paths": [
    "/path/to/image1.jpg",
    "/path/to/image2.jpg",
    "/path/to/image3.jpg"
  ],
  "output_directory": "/path/to/output/",
  "operation": "resize",
  "parameters": {
    "width": 800,
    "height": 600,
    "mode": "fit"
  }
}
```

## 🔧 Configuration

### Environment Variables
- `PORT`: Server port (default: 8081)
- `RUST_LOG`: Log level (default: info)

### Docker Environment
- Development: Hot reloading, volume mounts
- Production: Optimized builds, minimal runtime

## 📦 Dependencies

### Core Dependencies
```toml
actix-web = "4.11"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
log = "0.4"
env_logger = "0.10"
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1.0"
```

### Image Processing
```toml
image = "0.24"
imageproc = "0.23"
photon-rs = "0.3"
```

### AI/ML Processing
```toml
opencv = "0.88"
```

### Development
```toml
actix-web = { version = "4.11", features = ["macros"] }
```

## 🐳 Docker Support

### Development
```bash
# Build development image
make build-dev

# Run development container
make dev

# View logs
make logs-dev

# Stop development
make stop-dev
```

### Production
```bash
# Build production image
make build

# Run production container
make prod

# View logs
make logs

# Stop production
make stop
```

## 📋 Current Status

### ✅ Completed
- [x] Basic web server setup
- [x] Health check endpoint
- [x] Docker configuration
- [x] Comprehensive image processing modules
- [x] Video/audio processing capabilities
- [x] AI/ML features implementation
- [x] Error handling framework
- [x] Logging system
- [x] Development environment
- [x] Batch processing capabilities
- [x] Metadata extraction
- [x] Quality assessment và enhancement
- [x] Advanced effects và transformations
- [x] **HTTP API endpoints for all processing capabilities**

### 🚧 In Progress
- [ ] File upload/download endpoints
- [ ] Progress tracking cho long-running operations
- [ ] Job status tracking system
- [ ] Authentication/Authorization
- [ ] Rate limiting

### 📋 Planned
- [ ] Caching layer
- [ ] Database integration
- [ ] Monitoring/Metrics
- [ ] API documentation (OpenAPI/Swagger)
- [ ] WebSocket support for real-time progress
- [ ] File storage integration

## 🔄 Next Steps

To enhance the current API implementation:

1. **Add file upload/download** capabilities
2. **Implement progress tracking** cho long-running operations
3. **Add job status persistence** với database
4. **Implement authentication/authorization**
5. **Add rate limiting** và request validation
6. **Create comprehensive API documentation**

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License.

## 🆘 Support

For issues and questions:
1. Check the documentation
2. Review existing issues
3. Create a new issue with detailed information

---

**Note**: The service now has comprehensive HTTP API endpoints for all processing capabilities. The core processing logic is fully implemented and exposed via RESTful API endpoints.