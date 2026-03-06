# 🚀 Setting Up GitHub Actions Self-Hosted Runner on k3s Machine

## 📋 Prerequisites

1. **GitHub Personal Access Token** with `repo` and `workflow` permissions
   - Create at: https://github.com/settings/tokens
   - Save the token securely

2. **Access to the k3s machine** (this machine: `oldpanther.online`)

## 🔧 Setup Instructions

### Option 1: Automated Setup (Recommended)

Run the automated setup script:

```bash
# Make the script executable
chmod +x setup-github-runner.sh

# Run with your GitHub token
sudo GITHUB_TOKEN=your_github_token_here ./setup-github-runner.sh
```

### Option 2: Manual Setup Steps

If you prefer manual setup, follow these steps:

#### 1. Install Dependencies
```bash
sudo apt-get update
sudo apt-get install -y curl jq docker.io
```

#### 2. Configure kubectl for k3s
```bash
sudo mkdir -p /root/.kube
sudo cp /etc/rancher/k3s/k3s.yaml /root/.kube/config
sudo chmod 600 /root/.kube/config
```

#### 3. Create Runner Directory
```bash
sudo mkdir -p /opt/github-runner
cd /opt/github-runner
```

#### 4. Download GitHub Runner
```bash
# Get latest version
RUNNER_VERSION=$(curl -s https://api.github.com/repos/actions/runner/releases/latest | jq -r '.tag_name' | sed 's/^v//')

# Download
sudo curl -o actions-runner-linux-arm64-${RUNNER_VERSION}.tar.gz -L https://github.com/actions/runner/releases/download/v${RUNNER_VERSION}/actions-runner-linux-arm64-${RUNNER_VERSION}.tar.gz

# Extract
sudo tar xzf ./actions-runner-linux-arm64-${RUNNER_VERSION}.tar.gz
```

#### 5. Create Runner User
```bash
sudo useradd -m -s /bin/bash github-runner
sudo usermod -aG docker github-runner
sudo usermod -aG k3s github-runner
sudo chown -R github-runner:github-runner /opt/github-runner
```

#### 6. Configure Runner
```bash
# Get configuration token from GitHub
# Go to: https://github.com/vincent-tg/payment-tracker/settings/actions/runners
# Click "New self-hosted runner"
# Copy the configuration command

# Run as github-runner user
sudo -u github-runner ./config.sh --url https://github.com/vincent-tg/payment-tracker --token YOUR_TOKEN --name k3s-runner-$(hostname) --labels k3s,self-hosted,linux,arm64 --unattended
```

#### 7. Create Systemd Service
```bash
sudo cat > /etc/systemd/system/github-runner.service << 'EOF'
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

[Install]
WantedBy=multi-user.target
EOF
```

#### 8. Start the Service
```bash
sudo systemctl daemon-reload
sudo systemctl enable github-runner.service
sudo systemctl start github-runner.service
```

#### 9. Verify Setup
```bash
# Check service status
sudo systemctl status github-runner.service

# Check runner appears in GitHub
# Visit: https://github.com/vincent-tg/payment-tracker/settings/actions/runners
```

## 🎯 What This Enables

Once the runner is set up:

### ✅ **Local Builds & Tests**
- Rust code compiles on ARM64 (this machine's architecture)
- Tests run in the same environment as production
- No cross-compilation issues

### ✅ **Direct k3s Access**
- GitHub Actions can directly deploy to k3s
- No need for remote kubectl configuration
- Full access to cluster resources

### ✅ **Faster CI/CD**
- No network latency to GitHub-hosted runners
- Build cache persists between runs
- Local Docker registry available

### ✅ **Cost Effective**
- Free unlimited minutes on self-hosted runners
- No GitHub Actions minute costs
- Full control over runner resources

## 🔧 Runner Configuration Details

### Labels Applied:
- `k3s` - Identifies runner as k3s-capable
- `self-hosted` - Required for self-hosted runners
- `linux` - Operating system
- `arm64` - CPU architecture

### Resources:
- **CPU**: All available cores
- **Memory**: System memory
- **Storage**: System storage
- **Network**: Full network access

### Permissions:
- **Docker**: Can build and push images
- **kubectl**: Full k3s cluster access
- **Git**: Repository access

## 📊 Verification Steps

After setup, verify everything works:

### 1. Check Runner Status
```bash
sudo systemctl status github-runner.service
```

### 2. Check GitHub UI
- Visit: https://github.com/vincent-tg/payment-tracker/settings/actions/runners
- Should see runner with "Idle" status

### 3. Test Simple Workflow
Push a commit to trigger the CI/CD pipeline:
```bash
git add .
git commit -m "test: Trigger CI/CD"
git push origin main
```

### 4. Monitor Workflow Run
- Go to: https://github.com/vincent-tg/payment-tracker/actions
- Watch the workflow execute on your self-hosted runner

## 🚨 Troubleshooting

### Common Issues:

#### 1. Runner Not Appearing in GitHub
```bash
# Check service logs
sudo journalctl -u github-runner.service -f

# Restart service
sudo systemctl restart github-runner.service
```

#### 2. Docker Permission Denied
```bash
# Add user to docker group
sudo usermod -aG docker github-runner

# Restart service
sudo systemctl restart github-runner.service
```

#### 3. kubectl Access Issues
```bash
# Test kubectl access
sudo -u github-runner kubectl get nodes

# Fix kubeconfig permissions
sudo cp /etc/rancher/k3s/k3s.yaml /home/github-runner/.kube/config
sudo chown -R github-runner:github-runner /home/github-runner/.kube
sudo chmod 600 /home/github-runner/.kube/config
```

#### 4. Runner Goes Offline
```bash
# Check network connectivity
ping github.com

# Check service status
sudo systemctl status github-runner.service

# Restart if needed
sudo systemctl restart github-runner.service
```

## 🔄 Maintenance

### Update Runner Version
```bash
# Stop service
sudo systemctl stop github-runner.service

# Download latest
cd /opt/github-runner
sudo curl -o actions-runner-linux-arm64-latest.tar.gz -L https://github.com/actions/runner/releases/latest/download/actions-runner-linux-arm64-latest.tar.gz

# Extract (preserves config)
sudo tar xzf actions-runner-linux-arm64-latest.tar.gz

# Start service
sudo systemctl start github-runner.service
```

### Monitor Resource Usage
```bash
# Check CPU/Memory
top -u github-runner

# Check disk usage
df -h /opt/github-runner

# Check logs
sudo journalctl -u github-runner.service --since "1 hour ago"
```

## 🎉 Success Criteria

The setup is successful when:

1. ✅ Runner appears in GitHub UI with "Idle" status
2. ✅ CI/CD workflow triggers on push to main
3. ✅ Tests run successfully on self-hosted runner
4. ✅ Docker image builds without errors
5. ✅ Deployment to k3s works automatically
6. ✅ Daily tracking cronjob runs as scheduled

## 📞 Support

If you encounter issues:

1. Check service logs: `sudo journalctl -u github-runner.service -f`
2. Verify GitHub token has correct permissions
3. Ensure k3s is running: `sudo k3s kubectl get nodes`
4. Check Docker is running: `sudo systemctl status docker`

The runner will now handle all CI/CD for the VIB Bank Payment Tracker directly on your k3s machine! 🚀