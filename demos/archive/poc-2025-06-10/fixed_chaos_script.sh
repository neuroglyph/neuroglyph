#!/bin/bash
# SPDX-License-Identifier: Apache-2.0

# chaos-worker.sh - Fixed for macOS and git repo structure

# macOS-compatible random selection function
random_choice() {
    local arr=("$@")
    local index=$((RANDOM % ${#arr[@]}))
    echo "${arr[$index]}"
}

while true; do
    # Pick random repo (macOS compatible)
    repos=("vault" "projects")
    repo=$(random_choice "${repos[@]}")
    
    echo "Working in $repo..."
    cd "$repo"
    
    # Random chaos action
    actions=("add" "modify" "delete")
    action=$(random_choice "${actions[@]}")
    
    case $action in
        "add")
            # Create new file with random cross-links
            filename="random-$RANDOM.md"
            echo "# New file created at $(date)" > "$filename"
            echo "Links to [other file](../vault/resume.md)" >> "$filename"
            echo "And [another](../projects/portfolio/README.md)" >> "$filename"
            echo "Created $filename"
            ;;
        "modify") 
            # Modify existing file, add random link
            if ls *.md >/dev/null 2>&1; then
                files=(*.md)
                file=$(random_choice "${files[@]}")
                echo "" >> "$file"
                echo "Added link at $(date) to [something](../projects/portfolio/experience.md)" >> "$file"
                echo "Modified $file"
            else
                echo "No .md files to modify in $repo"
            fi
            ;;
        "delete")
            # Delete random file (only delete files we created)
            if ls random-*.md >/dev/null 2>&1; then
                files=(random-*.md)
                file=$(random_choice "${files[@]}")
                rm "$file"
                echo "Deleted $file"
            else
                echo "No random files to delete in $repo"
            fi
            ;;
    esac
    
    # Commit changes
    git add . 
    git commit -m "chaos $action at $(date)" >/dev/null 2>&1
    
    cd ..
    sleep 0.5  # Brief pause, then more chaos
done