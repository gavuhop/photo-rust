# 🎬🖼️ Photo-Go Media Processing Service

A unified high-performance media processing microservice built in Rust, combining video transcoding and image processing capabilities for the Photo-Go application backend.

## 🚀 Features

### 📺 Video Processing (Original)
- **Video Transcoding**: MP4, AVI, MOV format conversion
- **Audio Processing**: Extraction and format conversion
- **Metadata Extraction**: Video properties and technical information
- **Quality Control**: Bitrate, resolution, and codec optimization

### 🖼️ Image Processing (New)
- **Transformations**: Resize, rotate, crop, flip with various modes
- **Filters**: 20+ filters including artistic, color correction, and enhancement
- **Effects**: Glitch, pixelate, oil painting, watercolor, HDR, and more
- **Watermarking**: Text and image watermarks with positioning options
- **Batch Processing**: Process multiple images concurrently
- **Quality Assessment**: Automated image quality scoring and suggestions
- **Color Analysis**: Dominant colors, histograms, temperature analysis
- **Face Detection**: Computer vision integration (placeholder)
- **Duplicate Detection**: Perceptual hashing for duplicate image identification

### 🔧 Optimization & Storage
- **Smart Compression**: Quality-based compression for both video and images
- **Format Conversion**: Support for JPEG, PNG, WebP, GIF, TIFF
- **Thumbnail Generation**: Multiple sizes with cropping options
- **Web Optimization**: Automatic optimization for web delivery
- **Progressive JPEG**: Create progressive images for faster loading

### 📊 Analysis & AI
- **EXIF Extraction**: Camera settings, GPS, timestamps
- **Color Profiling**: ICC profile analysis
- **File Information**: Size, checksums, creation dates
- **Auto Enhancement**: Automatic levels, saturation, sharpening
- **Noise Reduction**: Remove image noise
- **Super Resolution**: AI-powered upscaling (placeholder)

## 🏗️ Architecture

### Unified Service Structure
```
pkg/transcode/src/
├── main.rs              # HTTP server and routing
├── models.rs            # Data structures and types
├── handlers.rs          # HTTP request handlers (video + image)
├── services.rs          # Core utilities
├── metadata.rs          # Metadata extraction
├── filters.rs           # Image filters
├── ai.rs               # AI/ML features
├── error.rs            # Error handling
│
├── image_processing.rs  # Image processing utilities
├── transformations.rs   # Basic image transformations
├── effects.rs          # Artistic effects
├── watermark.rs        # Watermarking
├── batch.rs            # Batch processing
├── quality.rs          # Quality assessment & enhancement
└── optimization.rs     # Compression & optimization
```

### HTTP API Endpoints

#### Health & Status
- `GET /api/v1/health` - Service health check

#### Video Processing (Original)
- `POST /api/v1/transcode/video` - Transcode video files
- `POST /api/v1/transcode/image` - Basic image transcode
- `POST /api/v1/transcode/audio` - Audio processing
- `GET /api/v1/transcode/status/{job_id}` - Get job status
- `POST /api/v1/metadata/extract` - Extract metadata
- `POST /api/v1/metadata/analyze` - Analyze video content

#### Image Processing (New)
- `POST /api/v1/image/resize` - Resize images
- `POST /api/v1/image/filter` - Apply filters
- `POST /api/v1/image/effect` - Apply effects
- `POST /api/v1/image/rotate` - Rotate images
- `POST /api/v1/image/crop` - Crop images
- `POST /api/v1/image/watermark` - Add watermarks
- `POST /api/v1/image/compress` - Compress images
- `POST /api/v1/image/convert` - Convert formats

#### Batch Operations
- `POST /api/v1/batch/resize-batch` - Batch resize
- `POST /api/v1/batch/optimize-batch` - Batch optimize
- `POST /api/v1/batch/convert-batch` - Batch convert

#### Analysis
- `POST /api/v1/analysis/colors` - Color analysis
- `POST /api/v1/analysis/quality` - Quality assessment
- `POST /api/v1/analysis/features` - Feature extraction
- `POST /api/v1/analysis/duplicates` - Duplicate detection

#### Enhancement
- `POST /api/v1/enhance/auto-enhance` - Auto enhancement
- `POST /api/v1/enhance/denoise` - Noise reduction
- `POST /api/v1/enhance/super-resolution` - Super resolution
- `POST /api/v1/enhance/color-correct` - Color correction

#### Creative Tools
- `POST /api/v1/creative/artistic` - Artistic filters
- `POST /api/v1/creative/remove-bg` - Background removal
- `POST /api/v1/creative/style-transfer` - Style transfer
- `POST /api/v1/creative/panorama` - Panorama stitching

#### Storage Optimization
- `POST /api/v1/storage/web-optimize` - Web optimization
- `POST /api/v1/storage/thumbnails` - Generate thumbnails
- `POST /api/v1/storage/progressive` - Create progressive JPEG

## 🛠️ Technology Stack

### Core Libraries
- **Video Processing**: `ffmpeg-next` for video transcoding
- **Image Processing**: `image`, `imageproc`, `fast_image_resize`, `photon-rs`
- **Audio Processing**: `symphonia`, `hound`
- **Computer Vision**: `opencv` (optional), Edge detection with `imageproc`
- **AI/ML**: `candle-core`, `candle-nn` for machine learning features
- **Color Science**: `palette`, `lab` for color space conversions
- **Async Runtime**: `tokio`, `futures` for concurrent processing
- **Web Framework**: `actix-web` for HTTP API

### Performance Features
- **Parallel Processing**: `rayon` for CPU-intensive operations
- **SIMD Optimizations**: Built-in with image processing libraries
- **Memory Efficiency**: Zero-copy operations where possible
- **Concurrent Processing**: Handle multiple media files simultaneously

### Supported Formats
- **Video**: MP4, AVI, MOV, MKV, WebM
- **Audio**: MP3, WAV, FLAC, AAC
- **Images**: JPEG, PNG, GIF, BMP, TIFF, WebP
- **Color Spaces**: sRGB, Adobe RGB, Display P3 (basic support)

## 🚀 Getting Started

### Prerequisites
- Rust 1.70+
- FFmpeg libraries (for video processing)
- Optional: OpenCV for advanced computer vision
- Optional: ONNX Runtime for AI features

### Installation
```bash
cd pkg/transcode
cargo build --release
```

### Running the Service
```bash
# Development mode
make media-processing-dev

# Or directly
cd pkg/transcode
cargo run

# Production build
make media-processing
```

### Configuration
Environment variables:
- `PORT`: Service port (default: 8081)
- `RUST_LOG`: Logging level (info, debug, trace)

## 📊 Performance Characteristics

### Benchmarks (Estimated)
- **Video Transcoding**: Depends on input format and settings
- **Image Resize (1920x1080 → 800x600)**: ~45ms
- **Filter Application**: ~20-80ms depending on complexity
- **Batch Processing**: ~100 images/minute for resize operations
- **Color Analysis**: ~15ms per image
- **Compression**: ~30ms with 60% size reduction

### Memory Usage
- **Base Memory**: ~50MB (includes video processing capabilities)
- **Processing Peak**: ~150MB for 4K video, ~80MB for 4K images
- **Concurrent Processing**: Scales with available CPU cores

### Scalability
- **Horizontal**: Multiple service instances
- **Vertical**: Multi-core CPU utilization
- **Load Balancing**: Stateless design for easy scaling

## 🔌 Integration with Photo-Go

### Go API Integration
```go
// Example Go client code for image processing
client := &http.Client{}
req, _ := http.NewRequest("POST", "http://localhost:8081/api/v1/image/resize", bytes.NewBuffer(jsonData))
req.Header.Set("Content-Type", "application/json")
resp, err := client.Do(req)

// Example for video transcoding
videoReq, _ := http.NewRequest("POST", "http://localhost:8081/api/v1/transcode/video", bytes.NewBuffer(videoData))
```

### Request/Response Format
```json
// Image resize request
{
  "input_path": "/path/to/input.jpg",
  "output_path": "/path/to/output.jpg",
  "width": 800,
  "height": 600
}

// Video transcode request
{
  "input_path": "/path/to/input.mp4",
  "output_path": "/path/to/output.mp4",
  "output_format": "mp4",
  "quality": "high"
}
```

## 🧪 Testing

### Unit Tests
```bash
cargo test
```

### Integration Tests
```bash
make media-processing-test
```

### API Testing
```bash
# Test both video and image endpoints
./scripts/test_merged_service.sh
```

## 📈 Monitoring & Metrics

### Health Checks
- Service health endpoint returns detailed status
- Performance metrics included in responses
- Processing time tracking for all operations

### Logging
- Structured logging with `env_logger`
- Request/response logging in handlers
- Performance metrics logging

## 🔧 Development

### Adding New Features
1. Add new endpoint in `handlers.rs`
2. Implement logic in appropriate module
3. Update routing in `main.rs`
4. Add tests and documentation

### Performance Optimization
- Use `rayon` for parallel processing
- Profile with `cargo flamegraph`
- Monitor memory usage
- Benchmark with `criterion`

## 🎯 Benefits of Unified Service

### ✅ **Advantages**
- **Single Deployment**: One service to manage instead of two
- **Shared Resources**: Efficient memory and CPU usage
- **Consistent API**: Unified structure for all media processing
- **Simplified Monitoring**: One service to monitor and maintain
- **Reduced Network Overhead**: No inter-service communication
- **Easy Development**: All media processing in one codebase

### 📊 **Performance Comparison**
```
Two Separate Services:
├── Video Service (Port 8081): ~30MB base memory
├── Image Service (Port 8082): ~25MB base memory
├── Network overhead: ~1ms per request
└── Total resources: 55MB + network latency

Unified Service:
├── Media Service (Port 8081): ~50MB base memory
├── No network overhead for internal processing
├── Shared optimization and caching
└── Total resources: 50MB + better performance
```

### 🔄 **Migration Benefits**
- **Backwards Compatible**: All existing endpoints still work
- **New Capabilities**: Image processing capabilities added
- **Easier Scaling**: Scale one service instead of two
- **Simpler Deployment**: One Docker container, one binary

## 🚀 Future Enhancements

### Planned Features
- **Advanced AI**: Real computer vision and style transfer
- **Real-time Processing**: WebSocket support for live processing
- **Cloud Storage**: Direct S3/GCS integration
- **Advanced Video**: 4K/8K video processing optimization
- **Streaming**: Real-time video streaming capabilities

### Performance Improvements
- **GPU Acceleration**: CUDA/OpenCL support for video and image processing
- **Hardware Acceleration**: Hardware-accelerated video encoding
- **Distributed Processing**: Multi-node batch processing
- **Caching Layer**: Redis-based result caching

## 📄 License

Part of the Photo-Go project. See main project LICENSE for details.

## 🤝 Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Add tests for new functionality
4. Commit changes (`git commit -am 'Add amazing feature'`)
5. Push to branch (`git push origin feature/amazing-feature`)
6. Create Pull Request

## 📞 Support

For issues and questions:
- Create GitHub issue in main Photo-Go repository
- Tag with `media-processing` label
- Include sample files and error logs

---

**Note**: This unified media processing service provides Photo-Go with comprehensive video and image processing capabilities while maintaining high performance, scalability, and operational simplicity.

## 🔧 Quick Commands

```bash
# Build and run
make media-processing-dev

# Test all functionality
./scripts/test_merged_service.sh

# Build for production
make media-processing

# Legacy aliases (still work)
make transcode        # Same as media-processing
make image-processing # Same as media-processing
```