#!/bin/bash

# Test script for Media Processing Service API
# Make sure the service is running on localhost:8081

echo "🧪 Testing Media Processing Service API"
echo "======================================"

# Test 1: Health Check
echo -e "\n1️⃣ Testing Health Check..."
curl -s http://localhost:8081/health | jq '.'

# Test 2: Video Info (will fail without actual file)
echo -e "\n2️⃣ Testing Video Info (expected to fail without file)..."
curl -s -X POST http://localhost:8081/api/v1/video/info \
  -H "Content-Type: application/json" \
  -d '{
    "file_path": "/tmp/nonexistent.mp4"
  }' | jq '.'

# Test 3: Video Transcode (will fail without actual file)
echo -e "\n3️⃣ Testing Video Transcode (expected to fail without file)..."
curl -s -X POST http://localhost:8081/api/v1/video/transcode \
  -H "Content-Type: application/json" \
  -d '{
    "input_path": "/tmp/input.mp4",
    "output_path": "/tmp/output.webm",
    "format": "webm",
    "codec": "libvpx",
    "bitrate": "1M",
    "resolution": "1280x720",
    "fps": 30
  }' | jq '.'

# Test 4: Audio Extraction (will fail without actual file)
echo -e "\n4️⃣ Testing Audio Extraction (expected to fail without file)..."
curl -s -X POST http://localhost:8081/api/v1/video/extract-audio \
  -H "Content-Type: application/json" \
  -d '{
    "input_path": "/tmp/video.mp4",
    "output_path": "/tmp/audio.mp3",
    "format": "libmp3lame",
    "bitrate": "128k"
  }' | jq '.'

# Test 5: Audio Transcode (will fail without actual file)
echo -e "\n5️⃣ Testing Audio Transcode (expected to fail without file)..."
curl -s -X POST http://localhost:8081/api/v1/audio/transcode \
  -H "Content-Type: application/json" \
  -d '{
    "input_path": "/tmp/input.mp3",
    "output_path": "/tmp/output.wav",
    "format": "wav"
  }' | jq '.'

# Test 6: Metadata Extraction (will fail without actual file)
echo -e "\n6️⃣ Testing Metadata Extraction (expected to fail without file)..."
curl -s -X POST http://localhost:8081/api/v1/metadata/extract \
  -H "Content-Type: application/json" \
  -d '{
    "file_path": "/tmp/video.mp4"
  }' | jq '.'

echo -e "\n✅ All API tests completed!"
echo "Note: Tests that require actual video files will fail as expected."
echo "To test with real files, place video files in /tmp/ and update the paths." 