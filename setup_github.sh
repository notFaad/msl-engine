#!/bin/bash

# MSL Engine GitHub Setup Script
# This script helps you create a GitHub repository for the MSL Engine project

echo "🚀 Setting up GitHub repository for MSL Engine..."
echo ""

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Please run this script from the msl-engine directory"
    exit 1
fi

# Check git status
if [ -z "$(git status --porcelain)" ]; then
    echo "✅ Working directory is clean"
else
    echo "⚠️  Warning: You have uncommitted changes"
    echo "   Consider committing them before pushing to GitHub"
    echo ""
fi

echo "📋 Next steps to create your GitHub repository:"
echo ""
echo "1. Go to https://github.com/new"
echo "2. Repository name: msl-engine"
echo "3. Description: Rust-based web scraping engine with custom DSL"
echo "4. Make it Public or Private (your choice)"
echo "5. DO NOT initialize with README (we already have one)"
echo "6. Click 'Create repository'"
echo ""
echo "7. After creating the repository, run these commands:"
echo ""
echo "   git remote add origin https://github.com/YOUR_USERNAME/msl-engine.git"
echo "   git branch -M main"
echo "   git push -u origin main"
echo ""
echo "8. Replace YOUR_USERNAME with your actual GitHub username"
echo ""
echo "🎉 Your MSL Engine will then be live on GitHub!"
echo ""
echo "📖 Documentation: The docs/ folder is excluded from this repository"
echo "   You can host the documentation separately or add it later"
echo ""
echo "📦 What's included in this repository:"
echo "   ✅ Source code (src/)"
echo "   ✅ Examples (examples/)"
echo "   ✅ Cargo.toml and Cargo.lock"
echo "   ✅ README.md"
echo "   ✅ .gitignore"
echo ""
echo "🚫 What's excluded:"
echo "   ❌ Documentation (docs/)"
echo "   ❌ Downloaded images and media"
echo "   ❌ Build artifacts (target/)"
echo "   ❌ Generated files"
echo ""
echo "Ready to create your GitHub repository! 🚀" 