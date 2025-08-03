# Photo-Rust Docker Makefile

.PHONY: help build build-dev run run-dev stop clean logs shell shell-dev test dev-hot-reload

# Default target
help:
	@echo "Photo-Rust Docker Commands:"
	@echo ""
	@echo "Build commands:"
	@echo "  make build      - Build production Docker image"
	@echo "  make build-dev  - Build development Docker image"
	@echo ""
	@echo "Run commands:"
	@echo "  make run        - Run production service with docker-compose"
	@echo "  make run-dev    - Run development service with hot reloading"
	@echo "  make dev-hot-reload - Run with enhanced hot reload (recommended)"
	@echo "  make stop       - Stop all services"
	@echo ""
	@echo "Utility commands:"
	@echo "  make logs       - Show service logs"
	@echo "  make logs-dev   - Show development service logs"
	@echo "  make shell      - Open shell in production container"
	@echo "  make shell-dev  - Open shell in development container"
	@echo "  make clean      - Clean up containers and volumes"
	@echo "  make test       - Run tests in container"
	@echo "  make health     - Check service health"

# Build commands
build:
	docker build -t photo-rust:latest .

build-dev:
	docker build -f Dockerfile.dev -t photo-rust:dev .

# Run commands
run:
	docker-compose up -d

run-dev:
	docker-compose -f docker-compose.dev.yml up -d

# Enhanced hot reload development
dev-hot-reload:
	@echo "🚀 Starting development with enhanced hot reload..."
	@echo "📁 Source changes will automatically trigger rebuilds"
	@echo "⏱️  Watch delay: 1 second"
	@echo "🔗 Service URL: http://localhost:8081"
	@echo "📊 Health Check: http://localhost:8081/health"
	docker-compose -f docker-compose.dev.yml up --build

stop:
	docker-compose down
	docker-compose -f docker-compose.dev.yml down

# Utility commands
logs:
	docker-compose logs -f photo-rust

logs-dev:
	docker-compose -f docker-compose.dev.yml logs -f photo-rust-dev

shell:
	docker-compose exec photo-rust /bin/bash

shell-dev:
	docker-compose -f docker-compose.dev.yml exec photo-rust-dev /bin/bash

clean:
	docker-compose down -v
	docker-compose -f docker-compose.dev.yml down -v
	docker system prune -f
	docker volume prune -f

test:
	docker-compose exec photo-rust cargo test

# Health check
health:
	@echo "Checking service health..."
	@curl -f http://localhost:8081/health || echo "Service is not healthy"

# Quick start for development
dev: build-dev run-dev
	@echo "Development environment started!"
	@echo "Service URL: http://localhost:8081"
	@echo "For hot reload: make dev-hot-reload"

# Quick start for production
prod: build run
	@echo "Production environment started!"
	@echo "Service URL: http://localhost:8081" 