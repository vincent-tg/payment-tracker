#!/bin/bash
set -e

echo "🚀 Quick GitHub Runner Setup for k3s"
echo "===================================="

# Check if we have sudo
if [ "$EUID" -ne 0 ]; then 
    echo "⚠️  Running without sudo, some steps may fail"
    echo "   Consider running with: sudo bash $0"
fi

echo ""
echo "📋 Prerequisites:"
echo "1. GitHub Personal Access Token with 'repo' and 'workflow' permissions"
echo "2. Token saved securely (won't be shown again!)"
echo ""
echo "🔗 Create token at: https://github.com/settings/tokens"
echo ""

# Ask for token
read -sp "🔑 Enter your GitHub Personal Access Token: " GITHUB_TOKEN
echo ""

if [ -z "$GITHUB_TOKEN" ]; then
    echo "❌ No token provided. Exiting."
    exit 1
fi

echo "✅ Token received (hidden for security)"
echo ""

# Repository details
REPO_OWNER="vincent-tg"
REPO_NAME="payment-tracker"
RUNNER_NAME="k3s-runner-$(hostname)-$(date +%s)"

echo "📦 Setting up runner for: $REPO_OWNER/$REPO_NAME"
echo "🏷️  Runner name: $RUNNER_NAME"
echo ""

# Step 1: Install dependencies
echo "🔧 Step 1: Installing dependencies..."
sudo apt-get update > /dev/null 2>&1
sudo apt-get install -y curl jq > /dev/null 2>&1
echo "✅ Dependencies installed"

# Step 2: Create directory
echo "📁 Step 2: Creating runner directory..."
sudo mkdir -p /opt/github-runner
cd /opt/github-runner
echo "✅ Directory created: /opt/github-runner"

# Step 3: Download runner
echo "📥 Step 3: Downloading GitHub Actions runner..."
RUNNER_VERSION=$(curl -s https://api.github.com/repos/actions/runner/releases/latest | jq -r '.tag_name' | sed 's/^v//')
echo "   Latest version: $RUNNER_VERSION"

sudo curl -o actions-runner-linux-arm64-${RUNNER_VERSION}.tar.gz -L https://github.com/actions/runner/releases/download/v${RUNNER_VERSION}/actions-runner-linux-arm64-${RUNNER_VERSION}.tar.gz > /dev/null 2>&1
sudo tar xzf ./actions-runner-linux-arm64-${RUNNER_VERSION}.tar.gz > /dev/null 2>&1
echo "✅ Runner downloaded and extracted"

# Step 4: Create runner user
echo "👤 Step 4: Creating runner user..."
if ! id -u github-runner > /dev/null 2>&1; then
    sudo useradd -m -s /bin/bash github-runner
    echo "✅ User created: github-runner"
else
    echo "✅ User already exists: github-runner"
fi

# Step 5: Set permissions
echo "🔐 Step 5: Setting permissions..."
sudo chown -R github-runner:github-runner /opt/github-runner
echo "✅ Permissions set"

# Step 6: Configure runner
echo "⚙️  Step 6: Configuring runner..."
sudo -u github-runner ./config.sh --url https://github.com/${REPO_OWNER}/${REPO_NAME} --token ${GITHUB_TOKEN} --name ${RUNNER_NAME} --labels k3s,self-hosted,linux,arm64 --unattended --replace
echo "✅ Runner configured"

# Step 7: Create service
echo "📋 Step 7: Creating systemd service..."
sudo cat > /etc/systemd/system/github-runner.service << 'EOF'
[Unit]
Description=GitHub Actions Runner
After=network.target

[Service]
Type=simple
User=github-runner
WorkingDirectory=/opt/github-runner
ExecStart=/opt/github-runner/run.sh
Restart=always
RestartSec=10
Environment="PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"

[Install]
WantedBy=multi-user.target
EOF
echo "✅ Service file created"

# Step 8: Start service
echo "🚀 Step 8: Starting runner service..."
sudo systemctl daemon-reload
sudo systemctl enable github-runner.service
sudo systemctl start github-runner.service
echo "✅ Service started"

# Step 9: Wait and check status
echo "⏳ Step 9: Waiting for runner to start..."
sleep 3
echo "📊 Checking status..."
sudo systemctl status github-runner.service --no-pager | head -20

echo ""
echo "🎉 GitHub Runner Setup Complete!"
echo "================================"
echo ""
echo "✅ Runner name: $RUNNER_NAME"
echo "✅ Labels: k3s, self-hosted, linux, arm64"
echo "✅ Service: github-runner (running)"
echo ""
echo "🌐 Check runner status in GitHub:"
echo "   https://github.com/vincent-tg/payment-tracker/settings/actions/runners"
echo ""
echo "📝 The runner should now pick up waiting jobs!"
echo "   Current waiting job: VIB Bank Payment Tracker CI/CD"
echo ""
echo "🔧 Management commands:"
echo "   sudo systemctl status github-runner"
echo "   sudo systemctl restart github-runner"
echo "   sudo journalctl -u github-runner -f"
echo ""
echo "🚀 GitHub Actions will now run on this k3s machine!"