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
echo "Searching for C/H files in: $PROJECT_ROOT"

# Store list of files missing newlines
tmpfile=$(mktemp)
trap 'rm -f "$tmpfile"' EXIT

# Find and check all C/H files
total=0
missing=0

find "$PROJECT_ROOT" -type f \( -name "*.c" -o -name "*.h" \) -not -path "*/build/*" -not -path "*/.git/*" | while read -r file; do
    echo "CHECK:$file" >> "$tmpfile"
    if [ -f "$file" ] && [ -r "$file" ]; then
        # Check if file ends with newline
        if [ -n "$(tail -c 1 "$file" 2>/dev/null)" ]; then
            echo "MISSING:$file" >> "$tmpfile"
        fi
    fi
done

# Count results (set +e temporarily to handle grep's exit code)
set +e
total=$(grep "^CHECK:" "$tmpfile" 2>/dev/null | wc -l | tr -d ' ')
missing=$(grep "^MISSING:" "$tmpfile" 2>/dev/null | wc -l | tr -d ' ')
set -e

# Ensure we have numeric values
total=${total:-0}
missing=${missing:-0}

echo "Checked $total C/H files"

if [ "$missing" -eq 0 ]; then
    echo -e "${GREEN}✓ All C/H files end with a newline${NC}"
    exit 0
else
    echo -e "${RED}✗ Found $missing files missing trailing newline:${NC}" >&2
    grep "^MISSING:" "$tmpfile" | cut -d: -f2- | while read -r file; do
        relative_path="${file#$PROJECT_ROOT/}"
        echo -e "  ${YELLOW}${relative_path}${NC}" >&2
    done
    echo "" >&2
    echo "To fix these files, run:" >&2
    echo "  for f in \\" >&2
    grep "^MISSING:" "$tmpfile" | cut -d: -f2- | while read -r file; do
        relative_path="${file#$PROJECT_ROOT/}"
        echo "    ${relative_path} \\" >&2
    done
    echo "  ; do echo >> \"\$f\"; done" >&2
    echo "" >&2
    echo "C standard requires files to end with a newline character." >&2
    echo "This is enforced by compilers with -pedantic flag." >&2
    exit 1
fi