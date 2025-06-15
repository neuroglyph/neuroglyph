#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
# Test that all C/H files end with a newline character

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# In Docker, the project is mounted at /test
if [ -f /.dockerenv ] || [ "${DOCKER_CONTAINER:-}" = "1" ]; then
    PROJECT_ROOT="/test"
else
    PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
fi

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "=== Checking C/H files for trailing newlines ==="

# Find all C and H files
files_missing_newline=()
total_files=0
files_checked=0

while IFS= read -r -d '' file; do
    ((total_files++))
    # Check if file exists and is readable
    if [ -f "$file" ] && [ -r "$file" ]; then
        ((files_checked++))
        # Check if file ends with newline
        if [ -n "$(tail -c 1 "$file" 2>/dev/null)" ]; then
            files_missing_newline+=("$file")
        fi
    fi
done < <(find "$PROJECT_ROOT" -type f \( -name "*.c" -o -name "*.h" \) -not -path "*/build/*" -not -path "*/.git/*" -print0)

echo "Checked $files_checked out of $total_files C/H files"

if [ ${#files_missing_newline[@]} -eq 0 ]; then
    echo -e "${GREEN}✓ All C/H files end with a newline${NC}"
    exit 0
else
    echo -e "${RED}✗ Found ${#files_missing_newline[@]} files missing trailing newline:${NC}"
    for file in "${files_missing_newline[@]}"; do
        # Make path relative to project root for cleaner output
        relative_path="${file#$PROJECT_ROOT/}"
        echo -e "  ${YELLOW}$relative_path${NC}"
    done
    echo ""
    echo "To fix these files, run:"
    echo "  for f in \\"
    for file in "${files_missing_newline[@]}"; do
        relative_path="${file#$PROJECT_ROOT/}"
        echo "    $relative_path \\"
    done
    echo "  ; do echo >> \"\$f\"; done"
    echo ""
    echo "C standard requires files to end with a newline character."
    echo "This is enforced by compilers with -pedantic flag."
    exit 1
fi