# Quick Command Reference

## 🌐 **GitHub Repository**
**URL:** https://github.com/vincent-tg/payment-tracker

### Clone the Repository:
```bash
# HTTPS
git clone https://github.com/vincent-tg/payment-tracker.git
cd payment-tracker

# SSH
git clone git@github.com:vincent-tg/payment-tracker.git
cd payment-tracker
```

### Check Repository Status:
```bash
gh repo view vincent-tg/payment-tracker
gh repo view vincent-tg/payment-tracker --web  # Open in browser
```

## 🛠️ **Development Commands**

### Build and Test:
```bash
make build           # Build application
make test            # Run tests
make check           # Check compilation
make lint            # Format and lint
make clean           # Clean build artifacts
```

### Docker Commands:
```bash
make docker-build    # Build Docker image
make docker-run      # Run with Docker Compose
```

### k3s Deployment:
```bash
make k3s-deploy      # Deploy to k3s cluster
make k3s-clean       # Clean up deployment
```

## 📦 **Application Commands**

### Initialize and Configure:
```bash
# Initialize database
./target/release/payment-tracker init

# Configure email (password from environment)
export EMAIL_APP_PASSWORD="kkwj gnwd gioh xjdj"
./target/release/payment-tracker config
```

### Transaction Operations:
```bash
# Add transactions
./target/release/payment-tracker add --amount 1000 --description "Salary" --type in
./target/release/payment-tracker add --amount 50 --description "Groceries" --type out

# List transactions
./target/release/payment-tracker list
./target/release/payment-tracker list --type in
./target/release/payment-tracker list --limit 5

# Generate reports
./target/release/payment-tracker summary
./target/release/payment-tracker summary --period week

# Fetch emails (simplified)
./target/release/payment-tracker fetch
```

## 🔧 **GitHub CLI Commands**

### Repository Management:
```bash
# View repository
gh repo view

# Open in browser
gh repo view --web

# List issues
gh issue list

# Create issue
gh issue create --title "Bug report" --body "Description"

# List pull requests
gh pr list

# Create pull request
gh pr create --title "Feature" --body "Description"
```

### Workflow Management:
```bash
# List workflows
gh workflow list

# View workflow
gh workflow view "CI/CD Pipeline"

# Run workflow
gh workflow run "CI/CD Pipeline"

# View workflow runs
gh run list
```

## 🐳 **Docker Commands**

### Build and Run:
```bash
# Build image
docker build -t payment-tracker:latest .

# Run container
docker run -it --rm \
  -e EMAIL_APP_PASSWORD="kkwj gnwd gioh xjdj" \
  payment-tracker:latest

# Docker Compose
docker-compose up -d
docker-compose logs -f
docker-compose down
```

## ☸️ **k3s Commands**

### Deployment:
```bash
cd k8s

# Deploy
./setup-k3s.sh

# Manual deployment
kubectl apply -f secrets.yaml
kubectl apply -f pvc.yaml
kubectl apply -f deployment.yaml

# Check status
kubectl get deployment payment-tracker
kubectl get pods -l app=payment-tracker
kubectl get service payment-tracker-service

# Access application
POD_NAME=$(kubectl get pods -l app=payment-tracker -o jsonpath="{.items[0].metadata.name}")
kubectl exec $POD_NAME -- /app/payment-tracker list

# Cleanup
kubectl delete -f .
```

## 🔐 **Environment Variables**

### Required:
```bash
export EMAIL_APP_PASSWORD="kkwj gnwd gioh xjdj"
```

### Optional:
```bash
export RUST_LOG=info
export DATABASE_PATH="/app/data/payments.db"
```

## 📁 **File Locations**

### Configuration:
- `~/.payment-tracker/config.toml` - User config
- `payment-tracker/.env.example` - Environment template

### Database:
- Default: `payment_tracker.db` (current directory)
- Configurable via config file

### Logs:
- Application: Check stdout/stderr
- Docker: `docker-compose logs`
- k3s: `kubectl logs -l app=payment-tracker`

## 🚨 **Troubleshooting**

### Build Issues:
```bash
# Clean and rebuild
cargo clean
cargo build --release

# Check dependencies
cargo check
cargo update
```

### Database Issues:
```bash
# Check SQLite
sqlite3 payment_tracker.db ".tables"

# Reinitialize
./target/release/payment-tracker init
```

### Docker Issues:
```bash
# Check Docker
docker --version
docker-compose --version

# Clean images
docker system prune -a
```

### k3s Issues:
```bash
# Check cluster
kubectl cluster-info
kubectl get nodes

# Check resources
kubectl get all -l app=payment-tracker
kubectl describe pod -l app=payment-tracker
```

## 📞 **Support**

### GitHub:
- Issues: https://github.com/vincent-tg/payment-tracker/issues
- Actions: https://github.com/vincent-tg/payment-tracker/actions
- Settings: https://github.com/vincent-tg/payment-tracker/settings

### Documentation:
- `README.md` - Main guide
- `QUICK_START.md` - Quick start
- `K3S_DEPLOYMENT.md` - k3s deployment
- `EMAIL_SETUP_INSTRUCTIONS.md` - Email setup

### Email Configuration:
- Email: `baotg.fin@gmail.com`
- App Password: `kkwj gnwd gioh xjdj`
- Environment Variable: `EMAIL_APP_PASSWORD`

Your Payment Tracker is ready to use! 🚀