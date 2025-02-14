.PHONY: run dev docker-build docker-run

# Run the API server
run:
	cargo run

# Run with watch mode for development
dev:
	cargo watch -x run

# Build Docker image
docker-build:
	docker build -t blockchain-security-api .

# Run in Docker
docker-run:
	docker run -p 8080:8080 blockchain-security-api

help:
	@echo "Available targets:"
	@echo "  run          - Run the API server"
	@echo "  dev          - Run with watch mode for development"
	@echo "  docker-build - Build Docker image"
	@echo "  docker-run   - Run in Docker container"