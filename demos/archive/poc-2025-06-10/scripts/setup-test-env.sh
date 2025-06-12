#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
# Docker-compatible setup script for git-knowledge-test environment

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Setting up Git Knowledge Test Environment...${NC}"

# Create test directory structure
mkdir -p git-knowledge-test
cd git-knowledge-test

# Copy essential scripts
cp /app/scripts/extract-links.sh ./
cp /app/scripts/chaos-worker.sh ./
chmod +x extract-links.sh chaos-worker.sh

# Create and initialize repositories
for repo in vault projects relationships; do
    echo -e "${GREEN}Creating $repo repository...${NC}"
    mkdir -p $repo
    cd $repo
    git init
    git config user.email "test@gitmind.local"
    git config user.name "Gitmind Test"
    cd ..
done

# Create initial content in vault
cd vault
cat > resume.md << 'EOF'
# Resume

This is my professional resume.

Related documents:
- [Technical Skills](technical-skills.md)
- [Portfolio](../projects/portfolio/README.md)
- [Experience](../projects/portfolio/experience.md)
EOF

cat > technical-skills.md << 'EOF'
# Technical Skills

## Programming Languages
- JavaScript/TypeScript
- Python
- Go

## Frameworks
- React
- Node.js
- Django

See my [portfolio](../projects/portfolio/README.md) for examples.
EOF

git add .
git commit -m "Initial vault setup"
cd ..

# Create initial content in projects
cd projects
mkdir -p portfolio
cd portfolio

cat > README.md << 'EOF'
# Portfolio

Welcome to my portfolio of projects.

## Navigation
- [Experience](experience.md)
- [Resume](../../vault/resume.md)
- [Skills](../../vault/technical-skills.md)
EOF

cat > experience.md << 'EOF'
# Professional Experience

## Software Engineer
- Built distributed systems
- Implemented knowledge graphs

Related: [Technical Skills](../../vault/technical-skills.md)
EOF

cd ..
git add .
git commit -m "Initial projects setup"
cd ..

# Create initial content in relationships
cd relationships
cat > README.md << 'EOF'
# Relationships

This repository stores extracted relationships between documents.
EOF

git add .
git commit -m "Initial relationships setup"
cd ..

# Set up git hooks
echo -e "${BLUE}Setting up git hooks...${NC}"

# Hook for vault repo
cat > vault/.git/hooks/post-commit << 'HOOK'
#!/bin/bash
REPO_ROOT=$(git rev-parse --show-toplevel)
cd "$REPO_ROOT"

echo "Processing markdown files for relationships..."

# Process all markdown files
find . -name "*.md" -type f | while read -r file; do
    if [ -f "../extract-links.sh" ]; then
        ../extract-links.sh "$file" | while read -r relationship; do
            if [ ! -z "$relationship" ]; then
                echo "$relationship" | git hash-object -w --stdin
                echo "Stored: $relationship"
            fi
        done
    fi
done
HOOK

# Hook for projects repo
cat > projects/.git/hooks/post-commit << 'HOOK'
#!/bin/bash
REPO_ROOT=$(git rev-parse --show-toplevel)
cd "$REPO_ROOT"

echo "Processing markdown files for relationships..."

# Process all markdown files
find . -name "*.md" -type f | while read -r file; do
    if [ -f "../extract-links.sh" ]; then
        ../extract-links.sh "$file" | while read -r relationship; do
            if [ ! -z "$relationship" ]; then
                echo "$relationship" | git hash-object -w --stdin
                echo "Stored: $relationship"
            fi
        done
    fi
done
HOOK

chmod +x vault/.git/hooks/post-commit
chmod +x projects/.git/hooks/post-commit

echo -e "${GREEN}Git Knowledge Test Environment setup complete!${NC}"
echo -e "You can now:"
echo -e "  - Run ${BLUE}./chaos-worker.sh${NC} to generate test data"
echo -e "  - Explore the repositories in vault/, projects/, and relationships/"