# 🚀 Quick Start - Media Processing Service

## ⚡ Chạy nhanh

### 1. Chạy Server
```bash
# Development mode
PORT=8082 RUST_LOG=info cargo run

# Hoặc production mode
cargo build --release
PORT=8082 RUST_LOG=info ./target/release/media-processing-service
```

### 2. Test Health Check
```bash
curl http://localhost:8082/health
```

### 3. Test API Endpoints
```bash
# Video info
curl -X POST http://localhost:8082/api/v1/video/info \
  -H "Content-Type: application/json" \
  -d '{"file_path": "/tmp/test.mp4"}'

# Video transcode
curl -X POST http://localhost:8082/api/v1/video/transcode \
  -H "Content-Type: application/json" \
  -d '{
    "input_path": "/tmp/input.mp4",
    "output_path": "/tmp/output.webm",
    "format": "webm",
    "codec": "libvpx"
  }'

# Audio extraction
curl -X POST http://localhost:8082/api/v1/video/extract-audio \
  -H "Content-Type: application/json" \
  -d '{
    "input_path": "/tmp/video.mp4",
    "output_path": "/tmp/audio.mp3"
  }'
```

## 📋 Các lệnh cơ bản

### Build và Run
```bash
# Build project
cargo build

# Run development
cargo run

# Run với port tùy chỉnh
PORT=8082 cargo run

# Build release
cargo build --release

# Run release
./target/release/media-processing-service
```

### Docker
```bash
# Build image
docker build -t media-processing-service .

# Run container
docker run -p 8082:8081 media-processing-service
```

### Testing
```bash
# Health check
curl http://localhost:8082/health

# Test script
./test_api.sh
```

## 🔧 Environment Variables

| Variable | Default | Usage |
|----------|---------|-------|
| `PORT` | 8081 | Port server |
| `RUST_LOG` | info | Log level |

## 📡 API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/health` | Health check |
| POST | `/api/v1/video/transcode` | Video transcoding |
| POST | `/api/v1/video/extract-audio` | Audio extraction |
| POST | `/api/v1/video/info` | Video metadata |
| POST | `/api/v1/audio/transcode` | Audio transcoding |
| POST | `/api/v1/metadata/extract` | Metadata extraction |

## 🐛 Troubleshooting

### Port đã được sử dụng
```bash
# Chạy trên port khác
PORT=8082 cargo run
```

### FFmpeg không tìm thấy
```bash
# Cài đặt FFmpeg
sudo apt update && sudo apt install ffmpeg
```

### Build errors
```bash
# Clean và rebuild
cargo clean && cargo build
```

## 📊 Monitoring

```bash
# Kiểm tra process
ps aux | grep media-processing

# Kiểm tra port
lsof -i :8082

# View logs
RUST_LOG=debug cargo run
```

## 🎯 Ví dụ sử dụng

### 1. Chuyển đổi video MP4 sang WebM
```bash
curl -X POST http://localhost:8082/api/v1/video/transcode \
  -H "Content-Type: application/json" \
  -d '{
    "input_path": "/tmp/input.mp4",
    "output_path": "/tmp/output.webm",
    "format": "webm",
    "codec": "libvpx",
    "bitrate": "1M",
    "resolution": "1280x720"
  }'
```

### 2. Trích xuất audio từ video
```bash
curl -X POST http://localhost:8082/api/v1/video/extract-audio \
  -H "Content-Type: application/json" \
  -d '{
    "input_path": "/tmp/video.mp4",
    "output_path": "/tmp/audio.mp3",
    "format": "libmp3lame",
    "bitrate": "128k"
  }'
```

### 3. Lấy thông tin video
```bash
curl -X POST http://localhost:8082/api/v1/video/info \
  -H "Content-Type: application/json" \
  -d '{
    "file_path": "/tmp/video.mp4"
  }'
```

## ✅ Server đang chạy!

Nếu bạn thấy output như này, server đã chạy thành công:
```
[INFO] Starting Media Processing Service...
[INFO] FFmpeg initialized successfully
[INFO] Server starting on 127.0.0.1:8082
```

Bây giờ bạn có thể sử dụng các API endpoints để xử lý video và audio! 