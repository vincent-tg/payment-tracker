#!/bin/bash
set -e

echo "🚀 Deploying VIB Bank Payment Tracker to k3s"
echo "============================================"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Must run from payment-tracker directory"
    exit 1
fi

# Build the Docker image locally
echo "🔨 Building Docker image..."
docker build -t vib-payment-tracker:latest .

# Load image into k3s
echo "📦 Loading image into k3s..."
sudo k3s ctr images import vib-payment-tracker:latest

# Create namespace if it doesn't exist
echo "📁 Creating namespace..."
sudo k3s kubectl create namespace payment-tracker --dry-run=client -o yaml | sudo k3s kubectl apply -f -

# Deploy PostgreSQL if not already deployed
echo "🗄️  Deploying PostgreSQL..."
sudo k3s kubectl apply -f k8s/postgresql-existing.yaml -n payment-tracker

# Wait for PostgreSQL to be ready
echo "⏳ Waiting for PostgreSQL to be ready..."
sudo k3s kubectl wait --for=condition=ready pod -l app=postgres -n payment-tracker --timeout=120s

# Create secrets
echo "🔐 Creating secrets..."
cat <<EOF | sudo k3s kubectl apply -n payment-tracker -f -
apiVersion: v1
kind: Secret
metadata:
  name: payment-tracker-secrets
type: Opaque
stringData:
  email-password: "tfme vicd wwkx iafg"
  postgres-password: "payment_password"
EOF

# Create config map
echo "📝 Creating config map..."
sudo k3s kubectl create configmap payment-tracker-config -n payment-tracker \
  --from-file=config.toml=config_example.toml \
  --dry-run=client -o yaml | sudo k3s kubectl apply -f -

# Deploy payment tracker
echo "🚀 Deploying payment tracker..."
sudo k3s kubectl apply -f k8s/payment-tracker.yaml -n payment-tracker

# Wait for deployment
echo "⏳ Waiting for deployment to be ready..."
sudo k3s kubectl wait --for=condition=available deployment/payment-tracker -n payment-tracker --timeout=120s

# Get pod status
echo "📊 Pod status:"
sudo k3s kubectl get pods -n payment-tracker

# Get service info
echo "🌐 Service info:"
sudo k3s kubectl get svc -n payment-tracker

# Run initial database setup
echo "🗄️  Initializing database..."
sudo k3s kubectl run payment-tracker-init -n payment-tracker \
  --image=vib-payment-tracker:latest \
  --restart=Never \
  --command -- /app/docker-entrypoint.sh init-db

# Wait for init to complete
sudo k3s kubectl wait --for=condition=complete job/payment-tracker-init -n payment-tracker --timeout=60s

# Clean up init pod
sudo k3s kubectl delete pod payment-tracker-init -n payment-tracker --ignore-not-found

echo ""
echo "🎉 DEPLOYMENT COMPLETE!"
echo "======================="
echo ""
echo "📊 To check status:"
echo "   sudo k3s kubectl get pods -n payment-tracker"
echo ""
echo "📝 To view logs:"
echo "   sudo k3s kubectl logs -f deployment/payment-tracker -n payment-tracker"
echo ""
echo "🌐 To access the service:"
echo "   sudo k3s kubectl port-forward svc/payment-tracker 8080:8080 -n payment-tracker"
echo "   Then open: http://localhost:8080"
echo ""
echo "📅 Daily tracking will run automatically at 8 AM"
echo "   To run manually: sudo k3s kubectl create job --from=cronjob/payment-tracker-daily daily-manual-\$(date +%s) -n payment-tracker"