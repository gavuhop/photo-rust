# 🚀 Hướng dẫn chạy Media Processing Service

## 📋 Yêu cầu hệ thống

### Prerequisites
- **Rust**: 1.70+ (kiểm tra với `rustc --version`)
- **FFmpeg**: Cài đặt FFmpeg (cho local development)
- **Docker**: (tùy chọn, cho containerized deployment)

### Kiểm tra FFmpeg
```bash
ffmpeg -version
```

## 🎯 Các cách chạy chương trình

### 1. Development Mode (Khuyến nghị)

#### Chạy trực tiếp với Cargo
```bash
# Build và chạy development server
RUST_LOG=info cargo run

# Hoặc chỉ định port
PORT=8082 RUST_LOG=info cargo run
```

#### Chạy binary đã build
```bash
# Build release version
cargo build --release

# Chạy binary
RUST_LOG=info ./target/release/media-processing-service

# Hoặc chỉ định port
PORT=8082 RUST_LOG=info ./target/release/media-processing-service
```

### 2. Docker Mode

#### Build Docker image
```bash
docker build -t media-processing-service .
```

#### Chạy container
```bash
# Chạy với port mặc định
docker run -p 8081:8081 media-processing-service

# Chạy với port tùy chỉnh
docker run -p 8082:8081 -e PORT=8081 media-processing-service

# Chạy với volume mounts (cho development)
docker run -p 8081:8081 \
  -v $(pwd)/tmp:/app/tmp \
  -v $(pwd)/logs:/app/logs \
  media-processing-service
```

### 3. Production Mode

#### Build production binary
```bash
cargo build --release
```

#### Chạy production server
```bash
# Với environment variables
RUST_LOG=info PORT=8081 ./target/release/media-processing-service

# Hoặc tạo systemd service
sudo cp target/release/media-processing-service /usr/local/bin/
sudo chmod +x /usr/local/bin/media-processing-service
```

## 🔧 Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | 8081 | Port server sẽ bind |
| `RUST_LOG` | info | Log level (debug/info/warn/error) |
| `RUST_BACKTRACE` | 1 | Enable backtraces (development) |

## 🧪 Testing

### Test Health Check
```bash
# Test health endpoint
curl http://localhost:8081/health

# Hoặc với port khác
curl http://localhost:8082/health
```

### Test API Endpoints
```bash
# Chạy test script
./test_api.sh

# Hoặc với port khác
./test_api_8082.sh
```

### Manual Testing
```bash
# Health check
curl http://localhost:8081/health | jq '.'

# Video transcode (với file thực tế)
curl -X POST http://localhost:8081/api/v1/video/transcode \
  -H "Content-Type: application/json" \
  -d '{
    "input_path": "/tmp/input.mp4",
    "output_path": "/tmp/output.webm",
    "format": "webm",
    "codec": "libvpx"
  }' | jq '.'
```

## 📊 Monitoring

### Kiểm tra server status
```bash
# Kiểm tra process
ps aux | grep media-processing

# Kiểm tra port
lsof -i :8081
netstat -tlnp | grep 8081

# Kiểm tra logs
tail -f logs/app.log
```

### Logs
```bash
# View real-time logs
RUST_LOG=debug cargo run

# Hoặc với file logging
RUST_LOG=info cargo run 2>&1 | tee logs/app.log
```

## 🐛 Troubleshooting

### Port đã được sử dụng
```bash
# Kiểm tra process đang sử dụng port
lsof -i :8081

# Kill process
sudo kill -9 <PID>

# Hoặc chạy trên port khác
PORT=8082 cargo run
```

### FFmpeg không tìm thấy
```bash
# Kiểm tra FFmpeg
which ffmpeg

# Cài đặt FFmpeg (Ubuntu/Debian)
sudo apt update
sudo apt install ffmpeg

# Cài đặt FFmpeg (macOS)
brew install ffmpeg
```

### Build errors
```bash
# Clean và rebuild
cargo clean
cargo build

# Update dependencies
cargo update
cargo build
```

## 📁 File Structure

```
project/
├── src/                    # Source code
├── target/                 # Build artifacts
├── logs/                   # Log files
├── tmp/                    # Temporary files
├── Cargo.toml             # Dependencies
├── Dockerfile             # Docker configuration
├── test_api.sh            # Test script
└── HOW_TO_RUN.md          # This file
```

## 🚀 Quick Start Commands

### Development
```bash
# 1. Build project
cargo build

# 2. Run server
RUST_LOG=info cargo run

# 3. Test health check
curl http://localhost:8081/health

# 4. Run full test suite
./test_api.sh
```

### Production
```bash
# 1. Build release
cargo build --release

# 2. Run production server
RUST_LOG=info ./target/release/media-processing-service

# 3. Test endpoints
curl http://localhost:8081/health
```

### Docker
```bash
# 1. Build image
docker build -t media-processing-service .

# 2. Run container
docker run -p 8081:8081 media-processing-service

# 3. Test endpoints
curl http://localhost:8081/health
```

## 📈 Performance Tips

### Development
- Sử dụng `cargo run` cho hot reloading
- Set `RUST_LOG=debug` cho detailed logging
- Sử dụng `cargo check` để kiểm tra syntax

### Production
- Sử dụng `cargo build --release` cho optimization
- Set `RUST_LOG=info` cho production logging
- Monitor memory usage và CPU usage

## 🔒 Security

### Development
- Không expose service ra internet
- Sử dụng localhost binding
- Validate input files

### Production
- Sử dụng reverse proxy (nginx)
- Implement authentication
- Validate file paths và permissions
- Use HTTPS với SSL certificates

## 📚 Additional Resources

- [API Documentation](./API_GUIDE.md)
- [Docker Guide](./DOCKER.md)
- [FFmpeg Documentation](https://ffmpeg.org/documentation.html)
- [Actix-web Documentation](https://actix.rs/docs/) 