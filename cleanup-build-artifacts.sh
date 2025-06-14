#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
# © 2025 J. Kirby Ross / Neuroglyph Collective

# Script to clean up build artifacts that were accidentally committed/created
# DO NOT RUN THIS WITHOUT UNDERSTANDING WHAT IT DOES!

set -euo pipefail  # Exit on error, undefined vars, pipe failures

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}=== Git Build Artifact Cleanup Script ===${NC}"
echo "This script will help clean up the mess from building outside Docker"
echo

# Step 1: Show current state
echo -e "${YELLOW}Step 1: Current state analysis${NC}"
echo "Checking for tracked object files..."
tracked_objects=$(git ls-files "*.o" 2>/dev/null || true)
if [ -n "$tracked_objects" ]; then
    echo -e "${RED}Found tracked object files:${NC}"
    echo "$tracked_objects"
else
    echo -e "${GREEN}No tracked object files found${NC}"
fi

echo
echo "Checking for untracked build artifacts..."
untracked_artifacts=$(git status --porcelain | grep -E '\.(o|a)$|gitmind$' | grep '^??' | awk '{print $2}' || true)
if [ -n "$untracked_artifacts" ]; then
    echo -e "${YELLOW}Found untracked build artifacts:${NC}"
    echo "$untracked_artifacts"
fi

echo
echo "Checking for modified tracked object files..."
modified_objects=$(git status --porcelain | grep -E '\.o$' | grep '^ M' | awk '{print $2}' || true)
if [ -n "$modified_objects" ]; then
    echo -e "${RED}Found modified tracked object files:${NC}"
    echo "$modified_objects"
fi

# Step 2: Remove from git tracking (but keep files for now)
echo
echo -e "${YELLOW}Step 2: Commands to remove files from git tracking${NC}"
echo "To remove these files from git tracking WITHOUT deleting them:"
echo
for obj in c/src/*.o; do
    if git ls-files --error-unmatch "$obj" >/dev/null 2>&1; then
        echo "git rm --cached $obj"
    fi
done

# Step 3: Delete the actual files
echo
echo -e "${YELLOW}Step 3: Commands to delete the actual build artifacts${NC}"
echo "To remove all build artifacts from the filesystem:"
echo
echo "# Remove object files"
echo "rm -f c/src/*.o"
echo
echo "# Remove binary"
echo "rm -f c/gitmind"
echo
echo "# Remove any static libraries if they exist"
echo "rm -f c/*.a c/src/*.a"

# Step 4: Update .gitignore
echo
echo -e "${YELLOW}Step 4: Ensure .gitignore is properly configured${NC}"
echo "Check that .gitignore includes these patterns:"
echo
cat << 'EOF'
# C build artifacts
*.o
*.a
*.so
*.dylib
gitmind
!*/gitmind/  # Don't ignore gitmind directories

# Build directories
build/
out/
bin/
EOF

# Step 5: Verify gitignore is working
echo
echo -e "${YELLOW}Step 5: Test if gitignore is working${NC}"
echo "After cleaning, you can test if gitignore works:"
echo
echo "# Build in Docker (the right way!)"
echo "cd c && docker compose run --rm dev make"
echo
echo "# Check if build artifacts are ignored"
echo "git status --porcelain | grep -E '\.(o|a)$|gitmind$'"
echo "(Should return nothing if gitignore is working)"

# Step 6: Commit the cleanup
echo
echo -e "${YELLOW}Step 6: Commit the cleanup (when ready)${NC}"
echo "Once you've removed the files from tracking:"
echo
echo "git add -u  # Stage the deletions"
echo "git commit -m 'chore: remove accidentally committed build artifacts'"
echo
echo "If you also update .gitignore:"
echo "git add .gitignore"
echo "git commit -m 'chore: update .gitignore for C build artifacts'"

# Safety reminder
echo
echo -e "${RED}=== IMPORTANT REMINDERS ===${NC}"
echo "1. This script shows commands but DOES NOT execute them"
echo "2. Review each command before running"
echo "3. The 'git rm --cached' removes from git but keeps the file"
echo "4. The 'rm' commands delete files permanently"
echo "5. Always build in Docker: 'cd c && docker compose run --rm dev make'"
echo
echo -e "${GREEN}Good luck cleaning up! 🧹${NC}"