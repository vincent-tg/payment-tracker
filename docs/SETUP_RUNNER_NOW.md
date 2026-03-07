# 🚀 URGENT: Set Up GitHub Runner NOW

GitHub Actions is waiting for a runner with message: **"Waiting for a runner to pick up this job..."**

## 🔑 Step 1: Get GitHub Token

1. Go to: https://github.com/settings/tokens
2. Click "Generate new token" → "Generate new token (classic)"
3. Name: `k3s-runner-token`
4. Select scopes: `repo` and `workflow`
5. Click "Generate token"
6. **COPY THE TOKEN** (you won't see it again!)

## 🖥️ Step 2: Run This Command

```bash
cd /home/ubuntu/.openclaw/workspace/payment-tracker
sudo bash QUICK_RUNNER_SETUP.sh
```

When prompted, paste your GitHub token.

## ✅ What Will Happen

1. ✅ Runner installed on this machine
2. ✅ Automatically registers with GitHub
3. ✅ Immediately picks up waiting job
4. ✅ CI/CD pipeline starts running
5. ✅ VIB Payment Tracker gets deployed

## 🎯 After Setup

The runner will appear here: https://github.com/vincent-tg/payment-tracker/settings/actions/runners

The waiting job (started at 00:44 UTC) will immediately begin executing.

## 🔧 Alternative: Manual Commands

If the script doesn't work, run these commands manually:

```bash
# 1. Install dependencies
sudo apt-get update
sudo apt-get install -y curl jq

# 2. Create directory
sudo mkdir -p /opt/github-runner
cd /opt/github-runner

# 3. Download runner
RUNNER_VERSION=$(curl -s https://api.github.com/repos/actions/runner/releases/latest | jq -r '.tag_name' | sed 's/^v//')
sudo curl -o actions-runner-linux-arm64-${RUNNER_VERSION}.tar.gz -L https://github.com/actions/runner/releases/download/v${RUNNER_VERSION}/actions-runner-linux-arm64-${RUNNER_VERSION}.tar.gz
sudo tar xzf ./actions-runner-linux-arm64-${RUNNER_VERSION}.tar.gz

# 4. Create user
sudo useradd -m -s /bin/bash github-runner
sudo chown -R github-runner:github-runner /opt/github-runner

# 5. Configure (REPLACE YOUR_TOKEN_HERE with your actual token)
sudo -u github-runner ./config.sh --url https://github.com/vincent-tg/payment-tracker --token YOUR_TOKEN_HERE --name "k3s-runner-$(hostname)" --labels k3s,self-hosted,linux,arm64 --unattended

# 6. Run in background
sudo -u github-runner ./run.sh &
```

## 🚨 IMMEDIATE ACTION REQUIRED

The CI/CD pipeline is stuck waiting. As soon as you set up the runner:

1. ✅ Tests will run on this machine
2. ✅ Docker image will build
3. ✅ Deployment to k3s will happen
4. ✅ VIB Payment Tracker will be live
5. ✅ Daily tracking will be scheduled

**Run the setup now to unblock the deployment!**