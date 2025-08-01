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

### ✅ Image Processing Modules
- **Image Loading/Saving**: Support cho multiple formats (JPEG, PNG, WebP, GIF)
- **Image Validation**: Input/output validation và format checking
- **Performance Monitoring**: Processing time tracking
- **File Operations**: Directory creation, file size calculation

### ✅ Image Processing Features
- **Resize Operations**: Multiple resize modes (fit, crop, exact)
- **Filter Applications**: Basic image filters (Sepia, Grayscale, Blur)
- **Watermark Support**: Text và image watermarking
- **Quality Assessment**: Image quality analysis và enhancement
- **Format Conversion**: Cross-format image conversion
- **Optimization**: Image compression và optimization

### ✅ Video/Audio Processing
- **FFmpeg Integration**: Video transcoding với FFmpeg
- **Metadata Extraction**: Media file metadata analysis
- **Stream Analysis**: Video/audio stream detection
- **Format Support**: Multiple video/audio formats

### ✅ AI/ML Features (Planned)
- **Object Detection**: Computer vision object detection
- **Face Detection**: Facial recognition capabilities
- **Color Analysis**: Advanced color analysis algorithms
- **Image Enhancement**: AI-powered image enhancement

## 🏗️ Architecture

```
photo-rust/
├── internal/transcode/          # Main service
│   ├── src/
│   │   ├── main.rs             # Web server entry point
│   │   ├── handlers.rs         # HTTP request handlers
│   │   ├── models.rs           # Data structures
│   │   ├── services.rs         # Core business logic
│   │   ├── image_processing.rs # Image processing logic
│   │   ├── filters.rs          # Image filter implementations
│   │   ├── transformations.rs  # Image transformation logic
│   │   ├── effects.rs          # Special effects
│   │   ├── watermark.rs        # Watermark functionality
│   │   ├── quality.rs          # Quality assessment
│   │   ├── optimization.rs     # Image optimization
│   │   ├── metadata.rs         # Media metadata handling
│   │   ├── batch.rs            # Batch processing
│   │   ├── ai.rs              # AI/ML features
│   │   └── error.rs           # Error handling
│   └── Cargo.toml             # Rust dependencies
├── Dockerfile                  # Production Docker image
├── Dockerfile.dev             # Development Docker image
├── docker-compose.yml         # Production orchestration
├── docker-compose.dev.yml     # Development orchestration
├── Makefile                   # Build automation
├── .dockerignore              # Docker build optimization
└── DOCKER.md                  # Docker documentation
```

## 🛠️ Technology Stack

### Backend
- **Language**: Rust 1.70+
- **Framework**: Actix-web 4.11
- **Async Runtime**: Tokio
- **Serialization**: Serde + Serde JSON
- **Logging**: log + env_logger
- **Image Processing**: image + imageproc
- **Video Processing**: FFmpeg integration
- **UUID Generation**: uuid crate

### Infrastructure
- **Containerization**: Docker
- **Orchestration**: Docker Compose
- **Build System**: Cargo (Rust)
- **Development**: Hot reloading support

## 🚀 Quick Start

### Development (Local)
```bash
cd internal/transcode
cargo run
```

### Development (Docker)
```bash
make dev
```

### Production (Docker)
```bash
make prod
```

## 📡 API Endpoints

### Health Check
- `GET /health` - Service health status

### Image Processing (Planned)
- `POST /api/v1/image/resize` - Resize images
- `POST /api/v1/image/filter` - Apply filters
- `POST /api/v1/image/watermark` - Add watermarks
- `POST /api/v1/image/convert` - Convert formats
- `POST /api/v1/image/optimize` - Optimize images

### Video Processing (Planned)
- `POST /api/v1/transcode/video` - Transcode videos
- `POST /api/v1/transcode/audio` - Transcode audio
- `POST /api/v1/metadata/extract` - Extract metadata

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
- [x] Image processing modules structure
- [x] Error handling framework
- [x] Logging system
- [x] Development environment

### 🚧 In Progress
- [ ] Image processing endpoints
- [ ] Video transcoding endpoints
- [ ] AI/ML features
- [ ] Batch processing
- [ ] Advanced filters

### 📋 Planned
- [ ] Authentication/Authorization
- [ ] Rate limiting
- [ ] Caching layer
- [ ] Database integration
- [ ] Monitoring/Metrics
- [ ] API documentation (OpenAPI/Swagger)

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

**Note**: This is a work in progress. The service currently has basic infrastructure in place and is ready for feature development.