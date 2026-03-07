#!/bin/bash

# GitHub Repository Setup Script for Payment Tracker

set -e

echo "=== GitHub Repository Setup ==="
echo ""

# Check if git is installed
if ! command -v git &> /dev/null; then
    echo "Error: git is not installed"
    exit 1
fi

# Initialize git repository if not already initialized
if [ ! -d ".git" ]; then
    echo "1. Initializing git repository..."
    git init
    echo "✓ Git repository initialized"
else
    echo "1. Git repository already initialized"
fi
echo ""

# Add all files
echo "2. Adding files to git..."
git add .
echo "✓ Files added to staging"
echo ""

# Commit changes
echo "3. Committing changes..."
git commit -m "Initial commit: Payment Tracker application

- Rust application for tracking payment cash in/out
- CLI interface with comprehensive commands
- SQLite database storage
- Simplified email parsing
- Docker containerization
- k3s/Kubernetes deployment manifests
- CI/CD pipeline with GitHub Actions
- Comprehensive documentation"
echo "✓ Changes committed"
echo ""

# Check if remote is configured
REMOTE_URL=$(git remote get-url origin 2>/dev/null || echo "")

if [ -z "$REMOTE_URL" ]; then
    echo "4. No remote repository configured"
    echo ""
    echo "To connect to GitHub:"
    echo "1. Create a new repository on GitHub: https://github.com/new"
    echo "2. Run the following commands:"
    echo "   git remote add origin https://github.com/YOUR_USERNAME/payment-tracker.git"
    echo "   git branch -M main"
    echo "   git push -u origin main"
    echo ""
    echo "Or if you want to use SSH:"
    echo "   git remote add origin git@github.com:YOUR_USERNAME/payment-tracker.git"
    echo "   git branch -M main"
    echo "   git push -u origin main"
else
    echo "4. Remote repository already configured: $REMOTE_URL"
    echo ""
    echo "To push to GitHub:"
    echo "   git push -u origin main"
fi
echo ""

# Create main branch if not exists
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ]; then
    echo "5. Creating main branch..."
    git branch -M main
    echo "✓ Switched to main branch"
else
    echo "5. Already on main branch"
fi
echo ""

echo "=== Repository Structure ==="
echo ""
echo "Files included in repository:"
echo ""
echo "📁 Source Code:"
echo "  src/                    - Rust source code"
echo "  Cargo.toml             - Rust dependencies"
echo "  Cargo.lock             - Dependency versions"
echo ""
echo "🐳 Containerization:"
echo "  Dockerfile             - Multi-stage Docker build"
echo "  docker-compose.yml     - Local development"
echo ""
echo "☸️ Kubernetes/k3s:"
echo "  k8s/                   - Deployment manifests"
echo "    deployment.yaml      - k8s Deployment & Service"
echo "    pvc.yaml             - PersistentVolumeClaim"
echo "    secrets.yaml         - Kubernetes Secrets"
echo "    setup-k3s.sh         - Deployment script"
echo ""
echo "🔧 Development:"
echo "  Makefile               - Build automation"
echo "  .github/workflows/     - CI/CD pipelines"
echo "  .gitignore             - Git ignore rules"
echo ""
echo "📚 Documentation:"
echo "  README.md              - Main documentation"
echo "  QUICK_START.md         - Quick start guide"
echo "  K3S_DEPLOYMENT.md      - k3s deployment guide"
echo "  PROJECT_SUMMARY.md     - Project overview"
echo "  .env.example           - Environment template"
echo ""
echo "🧪 Examples & Tests:"
echo "  example_usage.sh       - Usage examples"
echo "  simple_demo.rs         - Demo application"
echo "  test_app.sh            - Test script"
echo ""

echo "=== Next Steps ==="
echo ""
echo "1. Set up GitHub repository (if not already done)"
echo "2. Configure GitHub Secrets for CI/CD:"
echo "   - KUBECONFIG: Base64 encoded kubeconfig for k3s"
echo "   - DOCKER_USERNAME: Docker registry username"
echo "   - DOCKER_PASSWORD: Docker registry password"
echo ""
echo "3. Test locally:"
echo "   make build            # Build application"
echo "   make docker-build     # Build Docker image"
echo "   make docker-run       # Run with Docker Compose"
echo ""
echo "4. Deploy to k3s:"
echo "   make k3s-deploy       # Deploy to k3s cluster"
echo ""
echo "5. Push to GitHub:"
echo "   git push -u origin main"
echo ""
echo "=== Setup Complete ==="