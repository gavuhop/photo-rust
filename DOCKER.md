# Photo-Rust Docker Setup

This document describes how to run the Photo-Rust media processing service using Docker.

## Overview

Photo-Rust is a Rust-based media processing service that handles:
- 🎥 Video transcoding and processing
- 🖼️ Image processing and manipulation
- 📊 Metadata extraction
- 🔄 Audio processing

## Prerequisites

- Docker Engine 20.10+
- Docker Compose 2.0+
- At least 2GB RAM available for Docker
- 5GB+ free disk space

## Quick Start

### Development Environment

For development with hot reloading:

```bash
# Start development environment
make dev

# Or manually:
docker-compose -f docker-compose.dev.yml up -d
```

### Production Environment

For production deployment:

```bash
# Start production environment
make prod

# Or manually:
docker-compose up -d
```

## Docker Files

### `Dockerfile`
Production-ready multi-stage build that:
- Uses Rust 1.75 slim image for building
- Includes FFmpeg, ImageMagick, and other dependencies
- Creates optimized binary
- Runs as non-root user
- Includes health checks

### `Dockerfile.dev`
Development environment that:
- Includes all development tools
- Allows hot reloading
- Mounts source code for live development
- Includes debugging capabilities

### `docker-compose.yml`
Production orchestration with:
- Photo-Rust service
- Proper networking and volumes

### `docker-compose.dev.yml`
Development orchestration with:
- Development service with hot reloading
- Source code mounting

## Available Commands

### Using Makefile

```bash
# Show all available commands
make help

# Build images
make build          # Production build
make build-dev      # Development build

# Run services
make run            # Production
make run-dev        # Development
make stop           # Stop all services

# Utilities
make logs           # View logs
make shell          # Production shell
make shell-dev      # Development shell
make clean          # Clean up everything
make test           # Run tests
make health         # Health check
```

### Using Docker Compose Directly

```bash
# Production
docker-compose up -d
docker-compose logs -f
docker-compose down

# Development
docker-compose -f docker-compose.dev.yml up -d
docker-compose -f docker-compose.dev.yml logs -f
docker-compose -f docker-compose.dev.yml down
```

## Service Endpoints

Once running, the service is available at:

- **Main Service**: http://localhost:8081
- **Health Check**: http://localhost:8081/api/v1/health

### API Endpoints

```
📺 Video Processing:
  • POST /api/v1/transcode/video
  • POST /api/v1/transcode/audio
  • POST /api/v1/metadata/extract

🖼️ Image Processing:
  • POST /api/v1/image/resize
  • POST /api/v1/image/filter
  • POST /api/v1/image/effect
  • POST /api/v1/image/compress
```

## Environment Variables

### Service Configuration

| Variable | Default | Description |
|----------|---------|-------------|
| `RUST_PORT` | 8081 | Service port |
| `RUST_LOG` | info | Log level (debug/info/warn/error) |
| `RUST_BACKTRACE` | 1 | Enable backtraces |
| `PORT` | 8081 | Alternative port setting |

## Volumes and Data

### Persistent Volumes

- `./tmp` → `/app/tmp` - Media processing files
- `./logs` → `/app/logs` - Application logs
- `./config` → `/app/config` - Configuration files (read-only)

## Development Workflow

1. **Start Development Environment**:
   ```bash
   make dev
   ```

2. **Make Code Changes**:
   - Edit files in `internal/` directory
   - Changes are automatically detected and rebuilt

3. **View Logs**:
   ```bash
   make logs
   ```

4. **Access Container**:
   ```bash
   make shell-dev
   ```

5. **Run Tests**:
   ```bash
   make test
   ```

## Production Deployment

1. **Build Production Image**:
   ```bash
   make build
   ```

2. **Start Production Services**:
   ```bash
   make run
   ```

3. **Monitor Health**:
   ```bash
   make health
   ```

## Troubleshooting

### Common Issues

1. **Port Already in Use**:
   ```bash
   # Check what's using the port
   lsof -i :8081
   
   # Change port in docker-compose.yml
   ports:
     - "8082:8081"
   ```

2. **Build Failures**:
   ```bash
   # Clean and rebuild
   make clean
   make build
   ```

3. **Permission Issues**:
   ```bash
   # Fix volume permissions
   sudo chown -R $USER:$USER ./tmp ./logs
   ```

4. **Memory Issues**:
   ```bash
   # Increase Docker memory limit
   # In Docker Desktop: Settings → Resources → Memory
   ```

### Logs and Debugging

```bash
# View all logs
docker-compose logs

# View specific service logs
docker-compose logs photo-rust

# Follow logs in real-time
docker-compose logs -f photo-rust

# Access container for debugging
docker-compose exec photo-rust /bin/bash
```

## Performance Optimization

### Resource Limits

Add to `docker-compose.yml`:

```yaml
services:
  photo-rust:
    deploy:
      resources:
        limits:
          memory: 2G
          cpus: '2.0'
        reservations:
          memory: 1G
          cpus: '1.0'
```

### Build Optimization

- Use `.dockerignore` to exclude unnecessary files
- Multi-stage builds reduce final image size
- Layer caching improves build times

## Security Considerations

- Service runs as non-root user (`appuser`)
- Minimal runtime dependencies
- No sensitive data in image layers
- Health checks for monitoring
- Proper volume permissions

## Monitoring

### Health Checks

The service includes built-in health checks:

```bash
# Manual health check
curl http://localhost:8081/api/v1/health

# Docker health check
docker ps  # Shows health status
```

### Metrics

Monitor resource usage:

```bash
# Container stats
docker stats photo-rust-service

# Volume usage
docker system df
```

## Cleanup

```bash
# Stop and remove containers
make stop

# Clean everything (containers, volumes, images)
make clean
``` 