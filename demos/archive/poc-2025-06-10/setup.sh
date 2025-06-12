#!/bin/bash
# SPDX-License-Identifier: Apache-2.0

# Git Knowledge Graph MVP Setup Script

echo "Setting up Git Knowledge Graph test..."

# Create test directory
mkdir -p git-knowledge-test
cd git-knowledge-test

# Create the three repos
echo "Creating git repos..."
mkdir vault projects relationships

# Initialize repos
cd vault && git init && cd ..
cd projects && git init && cd ..
cd relationships && git init && cd ..

# Create some test files
echo "Creating test files..."

# Vault files (knowledge notes)
cat > vault/resume.md << 'EOF'
# My Resume

## Experience
See [experience notes](../projects/portfolio/experience.md) for details.

## Skills
- [Technical skills](technical-skills.md)
- [Soft skills](soft-skills.md)

References my [portfolio project](../projects/portfolio/README.md).
EOF

cat > vault/technical-skills.md << 'EOF'
# Technical Skills

## Programming
- Git and version control
- See [portfolio examples](../projects/portfolio/README.md)

Links back to [resume](resume.md).
EOF

# Projects files (code/work)
mkdir -p projects/portfolio
cat > projects/portfolio/README.md << 'EOF'
# Portfolio Project

This showcases work referenced in [my resume](../../vault/resume.md).

## Components
- [Experience documentation](experience.md)
- [Code samples](code/)
EOF

cat > projects/portfolio/experience.md << 'EOF'
# Experience Documentation

Detailed experience for [my resume](../../vault/resume.md).

Related to [technical skills](../../vault/technical-skills.md).
EOF

# Simple link extraction script
cat > extract-links.sh << 'EOF'
#!/bin/bash
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
EOF

chmod +x extract-links.sh

# Basic git hook for vault
mkdir -p vault/.git/hooks
cat > vault/.git/hooks/post-commit << 'EOF'
#!/bin/bash
echo "=== Vault Post-Commit Hook ==="

# Get changed files
changed_files=$(git diff --name-only HEAD~1 2>/dev/null || git ls-files)

for file in $changed_files; do
    if [[ "$file" == *.md ]]; then
        echo "Processing $file for relationships..."
        
        # Extract links using our script
        ../../extract-links.sh "$file" | while read line; do
            if [[ "$line" == LINK:* ]] || [[ "$line" == CROSS_REF:* ]]; then
                # Store relationship as git object
                echo "$line" | git hash-object -w --stdin
                echo "Stored: $line"
            fi
        done
    fi
done

echo "=== Done processing vault changes ==="
EOF

chmod +x vault/.git/hooks/post-commit

# Basic git hook for projects  
mkdir -p projects/.git/hooks
cat > projects/.git/hooks/post-commit << 'EOF'
#!/bin/bash
echo "=== Projects Post-Commit Hook ==="

# Get changed files
changed_files=$(git diff --name-only HEAD~1 2>/dev/null || git ls-files)

for file in $changed_files; do
    if [[ "$file" == *.md ]]; then
        echo "Processing $file for relationships..."
        
        # Extract links using our script
        ../../extract-links.sh "$file" | while read line; do
            if [[ "$line" == LINK:* ]] || [[ "$line" == CROSS_REF:* ]]; then
                # Store relationship as git object
                echo "$line" | git hash-object -w --stdin
                echo "Stored: $line"
            fi
        done
    fi
done

echo "=== Done processing projects changes ==="
EOF

chmod +x projects/.git/hooks/post-commit

# Commit initial files
echo "Making initial commits..."

cd vault
git add .
git commit -m "Initial vault files"
cd ..

cd projects  
git add .
git commit -m "Initial project files"
cd ..

cd relationships
echo "# Relationships Index" > README.md
git add .
git commit -m "Initial relationships repo"
cd ..

echo ""
echo "✅ Setup complete!"
echo ""
echo "Test the system:"
echo "1. cd git-knowledge-test/vault"
echo "2. Edit resume.md"  
echo "3. git add . && git commit -m 'test change'"
echo "4. Check the hook output for relationship detection"
echo ""
echo "Next steps:"
echo "- Add relationships push/pull to central repo"
echo "- Build query interface"
echo "- Test cross-repo coordination"