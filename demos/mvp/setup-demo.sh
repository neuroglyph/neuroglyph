#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
# Demo setup script - runs INSIDE Docker container

set -e

echo "🚀 Setting up GitMind demo repository..."

# Create a fresh directory for the demo
DEMO_DIR="$HOME/neuroglyph-demo"
rm -rf "$DEMO_DIR"
mkdir -p "$DEMO_DIR"
cd "$DEMO_DIR"

# Initialize git repo
git init
git config user.name "Demo User"
git config user.email "demo@example.com"

# Copy demo content
cp -r "$HOME/vault-template/"* .

# Create initial commit
git add .
git commit -m "Initial demo repository"

# Initialize gitmind
echo "📦 Initializing GitMind..."
gitmind init

# Create semantic links based on the demo content
echo "🔗 Creating semantic links..."

# Project structure links
gitmind link README.md project/roadmap.md --type REFERENCES
gitmind link project/roadmap.md project/milestones.md --type IMPLEMENTS
gitmind link project/architecture.md specs/api-design.md --type REFERENCES

# Research connections
gitmind link research/distributed-systems.md project/architecture.md --type INSPIRED_BY
gitmind link research/graph-databases.md ideas/semantic-layer.md --type INSPIRED_BY
gitmind link research/knowledge-graphs.md README.md --type INSPIRED_BY

# Ideas to implementation
gitmind link ideas/semantic-layer.md specs/link-format.md --type IMPLEMENTS
gitmind link ideas/chaos-mode.md research/emergence.md --type REFERENCES
gitmind link ideas/visualization.md specs/web-ui.md --type IMPLEMENTS

# Meeting notes connections
gitmind link meetings/2025-06-kickoff.md project/roadmap.md --type DISCUSSES
gitmind link meetings/2025-06-design-review.md specs/api-design.md --type REVIEWS

# Cross-references
gitmind link specs/api-design.md specs/link-format.md --type DEPENDS_ON
gitmind link specs/web-ui.md project/milestones.md --type IMPLEMENTS

# Show the created graph
echo ""
echo "✅ Demo repository created at: $DEMO_DIR"
echo ""
echo "📊 GitMind Status:"
gitmind status
echo ""
echo "🕸️  Link Graph:"
gitmind list
echo ""
echo "🎯 Try these commands:"
echo "  gitmind list --source README.md"
echo "  gitmind list --target project/architecture.md"
echo "  gitmind check"
echo ""
echo "📁 Repository structure:"
tree -a -I '.git'