#!/bin/bash
set -e

echo "🚀 Setting up GitHub Actions Self-Hosted Runner for k3s"
echo "========================================================"

# Check if running as root
if [ "$EUID" -ne 0 ]; then 
    echo "❌ Please run as root or with sudo"
    exit 1
fi

# Check for GitHub token
if [ -z "$GITHUB_TOKEN" ]; then
    echo "❌ GITHUB_TOKEN environment variable is not set"
    echo "💡 Create a Personal Access Token at: https://github.com/settings/tokens"
    echo "   Required scopes: repo, workflow"
    echo ""
    echo "Usage:"
    echo "  sudo GITHUB_TOKEN=your_token_here ./setup-github-runner.sh"
    exit 1
fi

# Repository details
REPO_OWNER="vincent-tg"
REPO_NAME="payment-tracker"
RUNNER_NAME="k3s-runner-$(hostname)"

echo "📦 Repository: $REPO_OWNER/$REPO_NAME"
echo "🏷️  Runner name: $RUNNER_NAME"

# Install dependencies
echo "🔧 Installing dependencies..."
apt-get update
apt-get install -y curl jq docker.io

# Install kubectl if not present
if ! command -v kubectl &> /dev/null; then
    echo "📦 Installing kubectl..."
    curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/arm64/kubectl"
    chmod +x kubectl
    mv kubectl /usr/local/bin/
fi

# Configure kubectl for k3s
echo "🔧 Configuring kubectl for k3s..."
mkdir -p /root/.kube
cp /etc/rancher/k3s/k3s.yaml /root/.kube/config
chmod 600 /root/.kube/config
export KUBECONFIG=/root/.kube/config

# Download and install GitHub runner
echo "📥 Downloading GitHub Actions runner..."
mkdir -p /opt/github-runner
cd /opt/github-runner

# Get latest runner version
RUNNER_VERSION=$(curl -s https://api.github.com/repos/actions/runner/releases/latest | jq -r '.tag_name' | sed 's/^v//')

# Download runner
curl -o actions-runner-linux-arm64-${RUNNER_VERSION}.tar.gz -L https://github.com/actions/runner/releases/download/v${RUNNER_VERSION}/actions-runner-linux-arm64-${RUNNER_VERSION}.tar.gz

# Extract
tar xzf ./actions-runner-linux-arm64-${RUNNER_VERSION}.tar.gz

# Create runner user
echo "👤 Creating runner user..."
useradd -m -s /bin/bash github-runner
usermod -aG docker github-runner
usermod -aG k3s github-runner

# Set ownership
chown -R github-runner:github-runner /opt/github-runner

# Configure runner
echo "⚙️  Configuring runner..."
sudo -u github-runner ./config.sh --url https://github.com/${REPO_OWNER}/${REPO_NAME} --token ${GITHUB_TOKEN} --name ${RUNNER_NAME} --labels k3s,self-hosted,linux,arm64 --unattended

# Create service file
echo "📋 Creating systemd service..."
cat > /etc/systemd/system/github-runner.service << EOF
[Unit]
Description=GitHub Actions Runner
After=network.target docker.service k3s.service

[Service]
Type=simple
User=github-runner
WorkingDirectory=/opt/github-runner
ExecStart=/opt/github-runner/run.sh
Restart=always
RestartSec=10
Environment="KUBECONFIG=/etc/rancher/k3s/k3s.yaml"
Environment="PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/local/bin"

[Install]
WantedBy=multi-user.target
EOF

# Start and enable service
echo "🚀 Starting GitHub runner service..."
systemctl daemon-reload
systemctl enable github-runner.service
systemctl start github-runner.service

# Check status
echo "📊 Checking runner status..."
sleep 5
systemctl status github-runner.service --no-pager

# Create kubeconfig for runner user
echo "🔧 Setting up kubeconfig for runner..."
mkdir -p /home/github-runner/.kube
cp /etc/rancher/k3s/k3s.yaml /home/github-runner/.kube/config
chown -R github-runner:github-runner /home/github-runner/.kube
chmod 600 /home/github-runner/.kube/config

# Update kubeconfig for internal IP
sed -i 's/127.0.0.1/10.0.0.229/g' /home/github-runner/.kube/config

# Test kubectl access
echo "🧪 Testing kubectl access..."
sudo -u github-runner kubectl get nodes

echo ""
echo "🎉 GitHub Actions Self-Hosted Runner Setup Complete!"
echo "==================================================="
echo ""
echo "✅ Runner installed: $RUNNER_NAME"
echo "✅ Labels: k3s, self-hosted, linux, arm64"
echo "✅ Service: github-runner (systemd)"
echo "✅ kubectl configured for k3s"
echo ""
echo "📊 To check runner status:"
echo "   systemctl status github-runner"
echo ""
echo "📝 To view logs:"
echo "   journalctl -u github-runner -f"
echo ""
echo "🔧 To restart runner:"
echo "   systemctl restart github-runner"
echo ""
echo "🌐 The runner will now appear in GitHub:"
echo "   https://github.com/$REPO_OWNER/$REPO_NAME/settings/actions/runners"
echo ""
echo "🚀 GitHub Actions workflows will now run on this k3s machine!"