# VIB Bank Payment Tracker - Deployment Guide

## 🚀 Quick Deployment to k3s

### Prerequisites
- k3s cluster running
- kubectl configured to access k3s
- Docker installed (for local builds)

### 1. Deploy PostgreSQL

```bash
# Deploy PostgreSQL to k3s
sudo k3s kubectl apply -f k8s/postgresql-existing.yaml

# Verify PostgreSQL is running
sudo k3s kubectl get pods | grep postgres
```

### 2. Set Up GitHub Actions Secrets

In your GitHub repository, go to Settings → Secrets and variables → Actions, add:

1. **K3S_KUBECONFIG**: Base64-encoded kubeconfig for k3s
   ```bash
   cat ~/.kube/config | base64 -w0
   ```

### 3. CI/CD Pipeline

The GitHub Actions workflow (`/.github/workflows/ci-cd.yml`) will automatically:

1. **Test**: Run Rust tests and checks on every push/PR
2. **Build & Push**: Build Docker image and push to GitHub Container Registry
3. **Deploy**: Deploy to k3s cluster on main branch pushes
4. **Daily Tracking**: Run daily at 8 AM via cronjob

### 4. Manual Deployment (Alternative)

If you want to deploy manually:

```bash
# Build the Docker image
docker build -t ghcr.io/your-username/payment-tracker:latest .

# Push to registry (if using remote registry)
docker push ghcr.io/your-username/payment-tracker:latest

# Deploy to k3s
sudo k3s kubectl apply -f k8s/payment-tracker.yaml
```

### 5. Verify Deployment

```bash
# Check all components
sudo k3s kubectl get all

# Check payment tracker pods
sudo k3s kubectl get pods -l app=payment-tracker

# Check cronjob
sudo k3s kubectl get cronjobs

# View logs
sudo k3s kubectl logs -f deployment/payment-tracker
```

### 6. Access the Application

```bash
# Port forward to access locally
sudo k3s kubectl port-forward svc/payment-tracker 8080:8080

# Then open: http://localhost:8080
# Health endpoint: http://localhost:8080/health
```

### 7. Run Daily Tracking Manually

```bash
# Trigger the daily tracking job
sudo k3s kubectl create job --from=cronjob/payment-tracker-daily daily-manual-$(date +%s)

# Check job status
sudo k3s kubectl get jobs

# View job logs
sudo k3s kubectl logs -f job/daily-manual-<timestamp>
```

## 📊 Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Gmail/IMAP    │    │  Payment Tracker │    │   PostgreSQL    │
│                 │◄──►│                 │◄──►│                 │
│  • Fetch emails │    │  • Parse VIB    │    │  • Store trans  │
│  • VIB filter   │    │  • Convert VND  │    │  • Upsert by ID │
│  • Date range   │    │  • Track IDs    │    │  • Daily reports│
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                        │                        │
         │                        │                        │
         ▼                        ▼                        ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│  Daily CronJob  │    │  Health Server  │    │   k3s Cluster   │
│  • 8 AM daily   │    │  • Port 8080    │    │  • Deployment   │
│  • Auto-run     │    │  • /health      │    │  • Service      │
│  • Email fetch  │    │  • Monitoring   │    │  • ConfigMap    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## 🔧 Configuration

### Environment Variables
- `DATABASE_URL`: PostgreSQL connection string
- `RUST_LOG`: Log level (info, debug, etc.)
- Email credentials from ConfigMap

### ConfigMap
The configuration is stored in a ConfigMap with:
- Email server settings
- Database connection
- Application settings

### Secrets
Sensitive data stored in Kubernetes Secrets:
- Email password
- PostgreSQL password

## 🚨 Troubleshooting

### Common Issues

1. **PostgreSQL connection failed**
   ```bash
   # Test PostgreSQL connection
   sudo k3s kubectl exec deployment/postgres -- pg_isready -h localhost
   
   # Check PostgreSQL logs
   sudo k3s kubectl logs deployment/postgres
   ```

2. **Email fetching failed**
   ```bash
   # Check email configuration
   sudo k3s kubectl get configmap payment-tracker-config -o yaml
   
   # Test IMAP connection manually
   sudo k3s kubectl run imap-test --image=alpine --rm -it -- sh
   apk add curl
   # Test IMAP connection
   ```

3. **CronJob not running**
   ```bash
   # Check cronjob status
   sudo k3s kubectl get cronjobs
   
   # Check cronjob logs
   sudo k3s kubectl describe cronjob payment-tracker-daily
   ```

4. **Health endpoint not responding**
   ```bash
   # Check pod status
   sudo k3s kubectl get pods
   
   # Check service
   sudo k3s kubectl get svc payment-tracker
   
   # Port forward and test
   sudo k3s kubectl port-forward svc/payment-tracker 8080:8080 &
   curl http://localhost:8080/health
   ```

## 📈 Monitoring

### Logs
```bash
# Application logs
sudo k3s kubectl logs -f deployment/payment-tracker

# CronJob logs
sudo k3s kubectl logs -f job/<job-name>

# PostgreSQL logs
sudo k3s kubectl logs -f deployment/postgres
```

### Metrics
```bash
# Resource usage
sudo k3s kubectl top pods

# Pod status
sudo k3s kubectl describe pod <pod-name>
```

## 🔄 Updates

### Update Application
```bash
# Trigger CI/CD pipeline by pushing to main
git push origin main

# Or manually update image
sudo k3s kubectl set image deployment/payment-tracker payment-tracker=ghcr.io/your-username/payment-tracker:latest
```

### Update Configuration
```bash
# Update ConfigMap
sudo k3s kubectl create configmap payment-tracker-config --from-file=config.toml -o yaml --dry-run=client | sudo k3s kubectl apply -f -

# Restart pods to pick up new config
sudo k3s kubectl rollout restart deployment/payment-tracker
```

## 🎯 Success Criteria

Deployment is successful when:
1. ✅ PostgreSQL pod is running
2. ✅ Payment tracker pod is running
3. ✅ Service is available on port 8080
4. ✅ Health endpoint returns 200 OK
5. ✅ CronJob is scheduled for daily runs
6. ✅ Daily tracking successfully processes VIB emails
7. ✅ Transactions are stored in PostgreSQL with proper upsert