# Payment Tracker Makefile

.PHONY: help build test clean run docker deploy

# Default target
help:
	@echo "Payment Tracker - Available targets:"
	@echo "  build     - Build the project"
	@echo "  test      - Run tests"
	@echo "  clean     - Clean build artifacts"
	@echo "  run       - Run the application"
	@echo "  docker    - Build Docker image"
	@echo "  deploy    - Deploy to k3s"
	@echo "  fmt       - Format code"
	@echo "  clippy    - Run clippy linter"
	@echo "  docs      - Generate documentation"

# Build targets
build:
	cargo build --release

build-dev:
	cargo build

# Test targets
test:
	cargo test --lib

test-all:
	cargo test

test-verbose:
	cargo test -- --nocapture

# Code quality
fmt:
	cargo fmt --all

clippy:
	cargo clippy -- -D warnings

check:
	cargo check

# Documentation
docs:
	cargo doc --open

# Cleanup
clean:
	cargo clean
	rm -f config.toml

# Run targets
run:
	cargo run -- fetch

run-cli:
	cargo run -- $(ARGS)

serve:
	cargo run -- serve

daily:
	cargo run -- daily

# Docker targets
docker:
	docker build -t payment-tracker .

docker-run:
	docker run -p 8080:8080 --env-file configs/.env payment-tracker

# Deployment targets
deploy:
	./scripts/deploy-to-k3s.sh

# Development setup
setup:
	cp configs/.env.example configs/.env
	cp configs/config_example.toml config.toml
	@echo "Please edit config.toml and configs/.env with your settings"

init-db:
	cargo run -- init

# Examples
examples:
	cargo build --examples

run-example:
	cargo run --example $(EXAMPLE)