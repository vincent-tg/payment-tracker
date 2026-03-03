# Makefile for Payment Tracker

.PHONY: help build test run clean docker-build docker-run k3s-deploy k3s-clean

# Default target
help:
	@echo "Payment Tracker - Available commands:"
	@echo ""
	@echo "Development:"
	@echo "  build        - Build the application"
	@echo "  test         - Run tests"
	@echo "  run          - Run the application locally"
	@echo "  clean        - Clean build artifacts"
	@echo ""
	@echo "Docker:"
	@echo "  docker-build - Build Docker image"
	@echo "  docker-run   - Run with Docker Compose"
	@echo ""
	@echo "Kubernetes/k3s:"
	@echo "  k3s-deploy   - Deploy to k3s cluster"
	@echo "  k3s-clean    - Clean up k3s deployment"
	@echo ""
	@echo "GitHub:"
	@echo "  github-init  - Initialize git repository"
	@echo "  github-push  - Push to GitHub"

# Development commands
build:
	cargo build --release

test:
	cargo test

run:
	cargo run -- --help

clean:
	cargo clean
	rm -rf target/

# Docker commands
docker-build:
	docker build -t payment-tracker:latest .

docker-run:
	docker-compose up -d

# k3s commands
k3s-deploy:
	@echo "Deploying to k3s..."
	@cd k8s && chmod +x setup-k3s.sh && ./setup-k3s.sh

k3s-clean:
	@echo "Cleaning up k3s deployment..."
	kubectl delete -f k8s/

# GitHub commands
github-init:
	@echo "Initializing git repository..."
	git init
	git add .
	git commit -m "Initial commit: Payment Tracker application"
	@echo "Repository initialized. Run 'git remote add origin <your-repo-url>' to add remote"

github-push:
	@echo "Pushing to GitHub..."
	git push -u origin main

# Utility commands
lint:
	cargo fmt --all
	cargo clippy -- -D warnings

check:
	cargo check

bench:
	cargo bench

# Database commands
db-init:
	./target/release/payment-tracker init

db-list:
	./target/release/payment-tracker list

db-add-example:
	@echo "Adding example transactions..."
	./target/release/payment-tracker add --amount 1000 --description "Salary" --type in
	./target/release/payment-tracker add --amount 50 --description "Groceries" --type out
	./target/release/payment-tracker add --amount 25 --description "Coffee" --type out

db-summary:
	./target/release/payment-tracker summary --period month

# Build for different architectures
build-linux:
	cargo build --release --target x86_64-unknown-linux-musl

build-arm:
	cargo build --release --target aarch64-unknown-linux-musl

# Release preparation
release: build-linux build-arm
	@echo "Creating release artifacts..."
	mkdir -p release
	cp target/x86_64-unknown-linux-musl/release/payment-tracker release/payment-tracker-x86_64
	cp target/aarch64-unknown-linux-musl/release/payment-tracker release/payment-tracker-arm64
	@echo "Release artifacts created in release/ directory"