#!/bin/bash

# Payment Tracker k3s Setup Script
# This script sets up the payment-tracker application on a k3s cluster

set -e

echo "=== Payment Tracker k3s Setup ==="
echo ""

# Check if kubectl is available
if ! command -v kubectl &> /dev/null; then
    echo "Error: kubectl is not installed or not in PATH"
    echo "Please install kubectl first"
    exit 1
fi

# Check k3s/k8s connection
echo "1. Checking k3s/k8s connection..."
kubectl cluster-info
if [ $? -ne 0 ]; then
    echo "Error: Cannot connect to k3s/k8s cluster"
    exit 1
fi
echo "✓ Connected to cluster"
echo ""

# Build Docker image
echo "2. Building Docker image..."
docker build -t payment-tracker:latest .
if [ $? -ne 0 ]; then
    echo "Error: Docker build failed"
    exit 1
fi
echo "✓ Docker image built successfully"
echo ""

# Load image into k3s (if using k3s with local registry)
echo "3. Loading Docker image into k3s..."
if command -v k3d &> /dev/null; then
    echo "   Detected k3d, loading image..."
    k3d image import payment-tracker:latest
elif [ -f "/etc/rancher/k3s/k3s.yaml" ]; then
    echo "   Detected k3s, copying image..."
    # For k3s, we need to use the local registry or push to a registry
    echo "   Note: For production, push to a container registry"
    echo "   Using local image (make sure imagePullPolicy is IfNotPresent)"
else
    echo "   Note: Make sure the Docker image is available in your cluster's registry"
fi
echo ""

# Create namespace (if not using default)
echo "4. Creating/updating Kubernetes resources..."
echo "   Applying secrets..."
kubectl apply -f secrets.yaml
echo ""

echo "   Applying PersistentVolumeClaim..."
kubectl apply -f pvc.yaml
echo ""

echo "   Applying Deployment and Service..."
kubectl apply -f deployment.yaml
echo ""

# Wait for deployment to be ready
echo "5. Waiting for deployment to be ready..."
kubectl wait --for=condition=available --timeout=300s deployment/payment-tracker
echo ""

# Show deployment status
echo "6. Deployment status:"
kubectl get deployment payment-tracker
echo ""

echo "7. Pod status:"
kubectl get pods -l app=payment-tracker
echo ""

echo "8. Service status:"
kubectl get service payment-tracker-service
echo ""

# Get pod name for exec commands
POD_NAME=$(kubectl get pods -l app=payment-tracker -o jsonpath="{.items[0].metadata.name}")
echo "9. Pod name: $POD_NAME"
echo ""

echo "10. Testing the application..."
echo "    Running 'payment-tracker --help' in pod:"
kubectl exec $POD_NAME -- /app/payment-tracker --help
echo ""

echo "11. Initializing database (if not already initialized)..."
kubectl exec $POD_NAME -- /app/payment-tracker init
echo ""

echo "=== Setup Complete ==="
echo ""
echo "To interact with the application:"
echo ""
echo "1. List transactions:"
echo "   kubectl exec $POD_NAME -- /app/payment-tracker list"
echo ""
echo "2. Add a transaction:"
echo "   kubectl exec $POD_NAME -- /app/payment-tracker add --amount 100 --description 'Test' --type in"
echo ""
echo "3. Generate summary:"
echo "   kubectl exec $POD_NAME -- /app/payment-tracker summary"
echo ""
echo "4. View logs:"
echo "   kubectl logs $POD_NAME"
echo ""
echo "5. Port forward to access service locally:"
echo "   kubectl port-forward service/payment-tracker-service 8080:80"
echo ""
echo "To update the deployment:"
echo "1. Rebuild Docker image: docker build -t payment-tracker:latest ."
echo "2. Update deployment: kubectl rollout restart deployment/payment-tracker"
echo ""
echo "To clean up:"
echo "   kubectl delete -f ."