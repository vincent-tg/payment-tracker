#!/bin/bash

# Script to push Payment Tracker to GitHub
set -e

echo "=== Payment Tracker GitHub Push ==="
echo ""

# Check if git is available
if ! command -v git &> /dev/null; then
    echo "Error: git is not installed"
    exit 1
fi

echo "1. Checking current repository status..."
cd "$(dirname "$0")"

# Check if we're in a git repository
if [ ! -d ".git" ]; then
    echo "Error: Not a git repository"
    exit 1
fi

# Get current branch
CURRENT_BRANCH=$(git branch --show-current)
echo "   Current branch: $CURRENT_BRANCH"
echo ""

# Check remote
REMOTE_URL=$(git remote get-url origin 2>/dev/null || echo "")
if [ -z "$REMOTE_URL" ]; then
    echo "2. No remote repository configured"
    echo ""
    echo "Please provide your GitHub username:"
    read -p "GitHub Username: " GITHUB_USERNAME
    
    if [ -z "$GITHUB_USERNAME" ]; then
        echo "Error: GitHub username is required"
        exit 1
    fi
    
    echo ""
    echo "Choose connection method:"
    echo "1) HTTPS (recommended for first time)"
    echo "2) SSH (requires SSH keys setup)"
    read -p "Enter choice (1 or 2): " CONNECTION_CHOICE
    
    case $CONNECTION_CHOICE in
        1)
            REMOTE_URL="https://github.com/$GITHUB_USERNAME/payment-tracker.git"
            ;;
        2)
            REMOTE_URL="git@github.com:$GITHUB_USERNAME/payment-tracker.git"
            ;;
        *)
            echo "Error: Invalid choice"
            exit 1
            ;;
    esac
    
    echo ""
    echo "Setting remote to: $REMOTE_URL"
    git remote add origin "$REMOTE_URL"
    
    echo ""
    echo "⚠️  IMPORTANT: First create the repository on GitHub:"
    echo "   Go to: https://github.com/new"
    echo "   Repository name: payment-tracker"
    echo "   DO NOT initialize with README or .gitignore"
    echo "   Click 'Create repository'"
    echo ""
    read -p "Press Enter after creating the repository on GitHub..."
else
    echo "2. Remote already configured: $REMOTE_URL"
    echo ""
fi

# Switch to main branch if not already
if [ "$CURRENT_BRANCH" != "main" ]; then
    echo "3. Switching to main branch..."
    git branch -M main
    echo "   Switched to main branch"
else
    echo "3. Already on main branch"
fi
echo ""

# Check for uncommitted changes
if [ -n "$(git status --porcelain)" ]; then
    echo "4. You have uncommitted changes:"
    git status --short
    echo ""
    echo "Would you like to commit them now? (y/n)"
    read -p "Choice: " COMMIT_CHOICE
    
    if [[ "$COMMIT_CHOICE" =~ ^[Yy]$ ]]; then
        echo "   Enter commit message:"
        read -p "   Message: " COMMIT_MESSAGE
        if [ -z "$COMMIT_MESSAGE" ]; then
            COMMIT_MESSAGE="Update payment tracker"
        fi
        git add .
        git commit -m "$COMMIT_MESSAGE"
        echo "   Changes committed"
    else
        echo "   Skipping commit"
    fi
else
    echo "4. No uncommitted changes"
fi
echo ""

# Push to GitHub
echo "5. Pushing to GitHub..."
echo "   This may ask for your GitHub credentials"
echo ""
if git push -u origin main; then
    echo "✅ Successfully pushed to GitHub!"
    echo ""
    echo "Repository URL:"
    if [[ "$REMOTE_URL" == https://* ]]; then
        echo "   https://github.com/$(echo "$REMOTE_URL" | sed 's|https://github.com/||;s|\.git$||')"
    elif [[ "$REMOTE_URL" == git@* ]]; then
        echo "   https://github.com/$(echo "$REMOTE_URL" | sed 's|git@github.com:||;s|\.git$||')"
    fi
else
    echo "❌ Push failed"
    echo ""
    echo "Possible issues:"
    echo "1. Repository not created on GitHub"
    echo "2. Authentication failed"
    echo "3. Network issues"
    echo ""
    echo "Solutions:"
    echo "1. Create repo: https://github.com/new"
    echo "2. Check credentials"
    echo "3. Try again with correct URL"
    exit 1
fi
echo ""

echo "6. Verifying push..."
echo "   Checking remote status..."
git remote -v
echo ""
echo "   Latest commit:"
git log --oneline -1
echo ""

echo "=== GitHub Setup Complete ==="
echo ""
echo "Next steps:"
echo "1. Visit your repository:"
if [[ "$REMOTE_URL" == https://* ]]; then
    REPO_NAME=$(echo "$REMOTE_URL" | sed 's|https://github.com/||;s|\.git$||')
    echo "   https://github.com/$REPO_NAME"
elif [[ "$REMOTE_URL" == git@* ]]; then
    REPO_NAME=$(echo "$REMOTE_URL" | sed 's|git@github.com:||;s|\.git$||')
    echo "   https://github.com/$REPO_NAME"
fi
echo ""
echo "2. Set up GitHub Secrets for CI/CD:"
echo "   - Go to Settings → Secrets and variables → Actions"
echo "   - Add KUBECONFIG, DOCKER_USERNAME, DOCKER_PASSWORD"
echo ""
echo "3. Test CI/CD pipeline by making a small change"
echo ""
echo "4. Deploy to k3s:"
echo "   cd k8s && ./setup-k3s.sh"
echo ""
echo "Your Payment Tracker is now on GitHub! 🎉"