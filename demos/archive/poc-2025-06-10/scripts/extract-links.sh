#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
# Extract markdown links and cross-references from a file
# Usage: ./extract-links.sh <file>

file="$1"
repo_dir="$(pwd)"
repo_name="$(basename "$repo_dir")"

if [ ! -f "$file" ]; then
    exit 0
fi

echo "=== Links found in $repo_name/$file ==="

# Extract markdown links [text](path)
grep -o '\[.*\]([^)]*\.md)' "$file" | while read link; do
    path=$(echo "$link" | sed 's/.*(\([^)]*\)).*/\1/')
    echo "LINK: $repo_name/$file -> $path"
done

# Extract simple cross-references
grep -o '\[.*\](../[^)]*\.md)' "$file" | while read link; do
    path=$(echo "$link" | sed 's/.*(\([^)]*\)).*/\1/')
    echo "CROSS_REF: $repo_name/$file -> $path"
done
