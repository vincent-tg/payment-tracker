# k3s Deployment Guide for Payment Tracker

This guide explains how to deploy the Payment Tracker application on a k3s Kubernetes cluster.

## Prerequisites

### 1. k3s Cluster
- A running k3s cluster
- `kubectl` configured to connect to your cluster
- Docker or container runtime installed

### 2. Local Development Tools
- Docker
- `kubectl` command-line tool
- (Optional) `k3d` for local k3s cluster testing

## Quick Deployment

### Option 1: Using the Setup Script (Recommended)

```bash
# 1. Build the Docker image
docker build -t payment-tracker:latest .

# 2. Run the setup script
cd k8s
chmod +x setup-k3s.sh
./setup-k3s.sh
```

### Option 2: Manual Deployment

```bash
# 1. Build the Docker image
docker build -t payment-tracker:latest .

# 2. Apply Kubernetes manifests
kubectl apply -f k8s/secrets.yaml
kubectl apply -f k8s/pvc.yaml
kubectl apply -f k8s/deployment.yaml

# 3. Check deployment status
kubectl get deployment payment-tracker
kubectl get pods -l app=payment-tracker
```

## Configuration

### 1. Update Secrets

Edit `k8s/secrets.yaml` with your actual email credentials:

```bash
# Generate base64 encoded values
echo -n "your-email@gmail.com" | base64
echo -n "your-app-password" | base64

# Update secrets.yaml with the generated values
```

### 2. Customize Deployment

You can modify the following files:
- `k8s/deployment.yaml`: Resource limits, environment variables, replicas
- `k8s/pvc.yaml`: Storage size and class
- `k8s/secrets.yaml`: Application secrets

## Architecture

### Kubernetes Resources

1. **Deployment**: Manages the payment-tracker pods
2. **Service**: ClusterIP service for internal access
3. **PersistentVolumeClaim**: 1GB storage for SQLite database
4. **Secret**: Stores email credentials securely

### Storage

The application uses a PersistentVolumeClaim with:
- 1GB storage request
- `local-path` storage class (k3s default)
- ReadWriteOnce access mode

### Networking

- Service type: ClusterIP (internal only)
- Port: 80 (service) → 8080 (container)
- For external access, use port forwarding or Ingress

## Access Methods

### 1. Port Forwarding (Local Access)

```bash
# Forward service to localhost
kubectl port-forward service/payment-tracker-service 8080:80

# Access application via localhost:8080
```

### 2. Exec into Pod (CLI Access)

```bash
# Get pod name
POD_NAME=$(kubectl get pods -l app=payment-tracker -o jsonpath="{.items[0].metadata.name}")

# Run commands in pod
kubectl exec $POD_NAME -- /app/payment-tracker list
kubectl exec $POD_NAME -- /app/payment-tracker add --amount 100 --description "Test" --type in
kubectl exec $POD_NAME -- /app/payment-tracker summary
```

### 3. View Logs

```bash
kubectl logs -l app=payment-tracker
kubectl logs -l app=payment-tracker --follow  # Stream logs
```

## Monitoring

### Check Deployment Status

```bash
# View all resources
kubectl get all -l app=payment-tracker

# View deployment details
kubectl describe deployment payment-tracker

# View pod details
kubectl describe pod -l app=payment-tracker

# View service details
kubectl describe service payment-tracker-service
```

### Health Checks

The deployment includes:
- **Liveness probe**: Checks if container is running
- **Readiness probe**: Checks if application is ready to serve requests

## Scaling

### Horizontal Scaling

```bash
# Scale to 3 replicas
kubectl scale deployment payment-tracker --replicas=3

# Check pod distribution
kubectl get pods -l app=payment-tracker -o wide
```

**Note**: Since Payment Tracker uses SQLite (single-writer database), horizontal scaling may require database changes.

### Resource Limits

Default resource limits:
- Request: 100m CPU, 64Mi memory
- Limit: 200m CPU, 128Mi memory

Adjust in `k8s/deployment.yaml` based on your needs.

## Updates and Maintenance

### 1. Update Application

```bash
# 1. Build new Docker image
docker build -t payment-tracker:latest .

# 2. Restart deployment
kubectl rollout restart deployment payment-tracker

# 3. Monitor rollout
kubectl rollout status deployment payment-tracker
```

### 2. Backup Database

```bash
# 1. Get pod name
POD_NAME=$(kubectl get pods -l app=payment-tracker -o jsonpath="{.items[0].metadata.name}")

# 2. Copy database from pod
kubectl cp $POD_NAME:/app/data/payments.db ./payments-backup-$(date +%Y%m%d).db
```

### 3. Restore Database

```bash
# 1. Get pod name
POD_NAME=$(kubectl get pods -l app=payment-tracker -o jsonpath="{.items[0].metadata.name}")

# 2. Copy backup to pod
kubectl cp ./payments-backup.db $POD_NAME:/app/data/payments.db

# 3. Restart pod
kubectl delete pod $POD_NAME
```

## Troubleshooting

### Common Issues

1. **Image pull errors**
   ```bash
   # Check if image exists locally
   docker images | grep payment-tracker
   
   # For k3d, load image
   k3d image import payment-tracker:latest
   ```

2. **PVC not bound**
   ```bash
   # Check PVC status
   kubectl get pvc
   
   # Check storage class
   kubectl get storageclass
   ```

3. **Pod crashing**
   ```bash
   # Check pod logs
   kubectl logs -l app=payment-tracker --previous
   
   # Describe pod for events
   kubectl describe pod -l app=payment-tracker
   ```

4. **Database initialization issues**
   ```bash
   # Check if database file exists
   kubectl exec $POD_NAME -- ls -la /app/data/
   
   # Manually initialize
   kubectl exec $POD_NAME -- /app/payment-tracker init
   ```

### Debug Commands

```bash
# Get detailed pod information
kubectl get pods -l app=payment-tracker -o yaml

# Check events in namespace
kubectl get events --sort-by='.lastTimestamp'

# Check resource usage
kubectl top pod -l app=payment-tracker
```

## Cleanup

### Remove Deployment

```bash
# Delete all resources
kubectl delete -f k8s/

# Or delete individually
kubectl delete deployment payment-tracker
kubectl delete service payment-tracker-service
kubectl delete pvc payment-tracker-pvc
kubectl delete secret payment-tracker-secrets
```

### Keep Data (Persistent Volume)

```bash
# Delete deployment but keep PVC
kubectl delete deployment payment-tracker
kubectl delete service payment-tracker-service
kubectl delete secret payment-tracker-secrets

# PVC and data will persist for future deployments
```

## Production Considerations

### 1. Use External Database
For production, consider replacing SQLite with:
- PostgreSQL
- MySQL
- CockroachDB

### 2. Implement Ingress
Add Ingress for external access:
```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: payment-tracker-ingress
spec:
  rules:
  - host: payment-tracker.example.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: payment-tracker-service
            port:
              number: 80
```

### 3. Add Monitoring
- Prometheus for metrics
- Grafana for dashboards
- Loki for logs

### 4. Implement Backup Strategy
- Regular database backups
- Backup to cloud storage (S3, GCS)
- Automated backup jobs using CronJobs

## Support

For issues with k3s deployment:
1. Check k3s logs: `journalctl -u k3s`
2. Check Kubernetes events: `kubectl get events`
3. Review application logs: `kubectl logs -l app=payment-tracker`

For application issues:
1. Check the main README.md
2. Review application documentation
3. Check GitHub issues