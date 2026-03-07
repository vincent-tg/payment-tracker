# GitHub Push Guide for Payment Tracker

## Step 1: Create GitHub Repository

1. Go to https://github.com/new
2. Fill in the repository details:
   - **Repository name**: `payment-tracker`
   - **Description**: `Rust application for tracking payment cash in/out by parsing bank emails`
   - **Visibility**: Public (or Private if you prefer)
   - **Initialize with README**: ❌ **UNCHECK THIS** (we already have files)
   - **Add .gitignore**: ❌ **UNCHECK THIS** (we already have .gitignore)
   - **Choose a license**: Optional (MIT recommended)

3. Click "Create repository"

## Step 2: Connect Local Repository to GitHub

### Option A: Using HTTPS (Recommended for first time)
```bash
cd payment-tracker
git remote add origin https://github.com/YOUR_USERNAME/payment-tracker.git
git branch -M main
git push -u origin main
```

### Option B: Using SSH (If you have SSH keys set up)
```bash
cd payment-tracker
git remote add origin git@github.com:YOUR_USERNAME/payment-tracker.git
git branch -M main
git push -u origin main
```

## Step 3: Verify Push Success

After pushing, visit: `https://github.com/YOUR_USERNAME/payment-tracker`

You should see all the files including:
- `src/` directory with Rust source code
- `k8s/` directory with Kubernetes manifests
- `Dockerfile` and `docker-compose.yml`
- `README.md` and other documentation
- `.github/workflows/ci-cd.yml` for CI/CD

## Step 4: Set Up GitHub Secrets (For CI/CD)

Go to your repository → Settings → Secrets and variables → Actions

Add these secrets:

### 1. **KUBECONFIG** (for k3s deployment)
- Get your kubeconfig: `cat ~/.kube/config | base64`
- Add as secret with name `KUBECONFIG`

### 2. **DOCKER_USERNAME** (optional, for Docker registry)
- Your Docker Hub or GitHub Container Registry username

### 3. **DOCKER_PASSWORD** (optional)
- Your Docker registry password or token

### 4. **EMAIL_APP_PASSWORD** (for testing)
- Your Gmail app password: `kkwj gnwd gioh xjdj`
- **Note**: This is sensitive - only add if you want CI to test email functionality

## Step 5: Test CI/CD Pipeline

1. Make a small change to trigger the workflow:
```bash
echo "# Test commit" >> README.md
git add README.md
git commit -m "Test CI/CD pipeline"
git push
```

2. Go to your repository → Actions
3. You should see the CI/CD pipeline running with:
   - ✅ Tests (format, lint, build, test)
   - ✅ Docker image building
   - ✅ (Optional) Deployment to k3s

## Step 6: Explore Your Repository

### Key Files to Review:
1. **Source Code**: `src/` - Rust implementation
2. **Deployment**: `k8s/` - Kubernetes manifests for k3s
3. **CI/CD**: `.github/workflows/ci-cd.yml` - Automated pipeline
4. **Documentation**: `README.md`, `K3S_DEPLOYMENT.md`
5. **Build**: `Makefile`, `Dockerfile`

### Repository Structure:
```
payment-tracker/
├── src/                    # Rust source code
│   ├── main.rs            # CLI interface
│   ├── lib.rs             # Application logic
│   ├── config.rs          # Configuration
│   ├── db.rs              # Database operations
│   ├── email.rs           # Email parsing
│   ├── models.rs          # Data structures
│   └── transactions.rs    # Utilities
├── k8s/                   # Kubernetes manifests
│   ├── deployment.yaml    # k8s Deployment
│   ├── pvc.yaml           # PersistentVolumeClaim
│   ├── secrets.yaml       # Secrets template
│   └── setup-k3s.sh       # Deployment script
├── .github/workflows/     # CI/CD pipelines
│   └── ci-cd.yml          # Main workflow
├── Dockerfile             # Container definition
├── docker-compose.yml     # Local development
├── Makefile               # Build automation
├── README.md              # Documentation
└── *.md                   # Additional guides
```

## Step 7: Next Steps After GitHub Setup

### 1. **Enable GitHub Pages** (Optional)
- Go to Settings → Pages
- Source: `GitHub Actions`
- This can host your documentation

### 2. **Set Up Issues Template**
- Create `.github/ISSUE_TEMPLATE/`
- Add bug_report.md and feature_request.md

### 3. **Configure Branch Protection**
- Go to Settings → Branches → Add rule
- Branch: `main`
- Require pull request reviews
- Require status checks to pass
- Include administrators

### 4. **Add Collaborators** (Optional)
- Go to Settings → Collaborators
- Add team members who can contribute

## Troubleshooting

### If push fails with authentication error:
```bash
# Check remote URL
git remote -v

# Update remote URL
git remote set-url origin https://github.com/YOUR_USERNAME/payment-tracker.git

# Or use SSH
git remote set-url origin git@github.com:YOUR_USERNAME/payment-tracker.git
```

### If push fails with "non-fast-forward" error:
```bash
# Pull changes first
git pull origin main --allow-unrelated-histories
git push
```

### If CI/CD fails:
1. Check Actions tab for error details
2. Verify secrets are correctly set
3. Check workflow file syntax

## Quick Reference Commands

```bash
# Clone (after creating on GitHub)
git clone https://github.com/YOUR_USERNAME/payment-tracker.git
cd payment-tracker

# Make changes
git add .
git commit -m "Description of changes"
git push

# Create new branch
git checkout -b feature/new-feature
git push -u origin feature/new-feature

# Create pull request (via GitHub UI)
# Then merge and delete branch
```

## Support

If you encounter issues:
1. Check GitHub documentation: https://docs.github.com
2. Review repository settings
3. Verify file permissions and paths
4. Check CI/CD workflow logs

Your Payment Tracker is now ready for GitHub! 🚀

**Repository URL:** `https://github.com/YOUR_USERNAME/payment-tracker`

**Next:** Set up your k3s cluster and deploy using the manifests in `k8s/` directory.