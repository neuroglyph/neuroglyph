# Git Knowledge Graph Server Hanging - Debug Guide

## Quick Diagnostics

**Location:** `~/git/gitgud/`

### Step 1: Check Repository Health
```bash
cd ~/git/gitgud
# Check if git repos are corrupted
find git-knowledge-test/ -name ".git" -type d | while read repo; do
    cd "$(dirname "$repo")"
    echo "=== Checking $(pwd) ==="
    git status
    git fsck --full
    git gc --prune=now
    cd - > /dev/null
done
```

### Step 2: Test Git Operations Manually
```bash
cd ~/git/gitgud/git-knowledge-test
# Try basic git operations that the server might be doing
git log --oneline -10
git branch -a
git ls-files
git show-ref
```

### Step 3: Kill Any Hanging Git Processes
```bash
# Kill any git processes that might be hanging
pkill -f git
ps aux | grep git
```

## Enhanced Server Debug Version

Create this debug version of your server.js:

```javascript
const express = require('express');
const { execSync, exec } = require('child_process');
const path = require('path');
const fs = require('fs');

const app = express();
const port = 3000;

// Enhanced logging with timestamps
function log(message) {
    const timestamp = new Date().toISOString();
    console.log(`[${timestamp}] ${message}`);
}

function executeGitCommand(command, cwd, timeout = 5000) {
    return new Promise((resolve, reject) => {
        log(`Executing: ${command} in ${cwd}`);
        
        const child = exec(command, { 
            cwd,
            timeout,
            killSignal: 'SIGKILL'
        }, (error, stdout, stderr) => {
            if (error) {
                log(`Command failed: ${command} - ${error.message}`);
                reject(error);
            } else {
                log(`Command completed: ${command}`);
                resolve(stdout.trim());
            }
        });

        // Force kill after timeout
        setTimeout(() => {
            if (child.pid) {
                log(`Force killing command after timeout: ${command}`);
                try {
                    process.kill(child.pid, 'SIGKILL');
                } catch (e) {
                    log(`Error killing process: ${e.message}`);
                }
                reject(new Error(`Command timeout: ${command}`));
            }
        }, timeout);
    });
}

async function scanRepositories() {
    log('🔍 Starting repository scan...');
    
    const testDir = path.join(__dirname, 'git-knowledge-test');
    log(`Scanning directory: ${testDir}`);
    
    if (!fs.existsSync(testDir)) {
        log('❌ git-knowledge-test directory not found');
        return { nodes: [], links: [] };
    }

    const repos = [];
    const entries = fs.readdirSync(testDir);
    log(`Found entries: ${entries.join(', ')}`);

    // Process repos one by one with detailed logging
    for (const entry of entries) {
        const repoPath = path.join(testDir, entry);
        const gitPath = path.join(repoPath, '.git');
        
        log(`Checking ${entry}...`);
        
        if (fs.existsSync(gitPath)) {
            log(`✅ Found git repo: ${entry}`);
            
            try {
                // Test basic git operations with short timeouts
                log(`Testing git status in ${entry}...`);
                await executeGitCommand('git status --porcelain', repoPath, 2000);
                
                log(`Testing git log in ${entry}...`);
                const logOutput = await executeGitCommand('git log --oneline -5', repoPath, 3000);
                log(`Log output length: ${logOutput.length} chars`);
                
                log(`Testing git branch in ${entry}...`);
                await executeGitCommand('git branch -a', repoPath, 2000);
                
                repos.push({
                    name: entry,
                    path: repoPath,
                    commits: logOutput.split('\n').length
                });
                
                log(`✅ Successfully processed ${entry}`);
                
            } catch (error) {
                log(`❌ Error processing ${entry}: ${error.message}`);
                // Continue with other repos instead of failing completely
            }
        } else {
            log(`⏭️  Skipping ${entry} (not a git repo)`);
        }
    }

    log(`📊 Completed scan. Found ${repos.length} working repositories.`);
    
    // Return minimal data for now
    const nodes = repos.map((repo, i) => ({
        id: i,
        name: repo.name,
        type: 'repository',
        commits: repo.commits
    }));

    return { nodes, links: [] };
}

app.use(express.static('public'));

app.get('/api/graph-data', async (req, res) => {
    try {
        log('📡 Graph data request received');
        const data = await scanRepositories();
        log(`📤 Sending response with ${data.nodes.length} nodes`);
        res.json(data);
    } catch (error) {
        log(`❌ Error in /api/graph-data: ${error.message}`);
        log(`Stack trace: ${error.stack}`);
        res.status(500).json({ error: error.message, nodes: [], links: [] });
    }
});

app.listen(port, () => {
    log(`🚀 Debug server running at http://localhost:${port}`);
    log('📁 Current directory:', __dirname);
    log('📁 Test directory should be at:', path.join(__dirname, 'git-knowledge-test'));
});
```

## Debugging Steps

### Step 1: Use Debug Server
```bash
cd ~/git/gitgud
# Backup original and use debug version
cp server.js server.js.backup
# Replace server.js with debug version above
node server.js
```

### Step 2: Watch for Hang Location
The debug version will show exactly where it hangs:
- If it hangs on "Testing git status..." → Git repo corruption
- If it hangs on "Testing git log..." → Commit history issues  
- If it hangs on "Testing git branch..." → Branch reference problems

### Step 3: Fix Identified Issues

**If git status hangs:**
```bash
cd ~/git/gitgud/git-knowledge-test/[problem-repo]
git reset --hard HEAD
git clean -fd
```

**If git log hangs:**
```bash
cd ~/git/gitgud/git-knowledge-test/[problem-repo]
git fsck --full
git repack -ad
```

**If git branch hangs:**
```bash
cd ~/git/gitgud/git-knowledge-test/[problem-repo]
git remote prune origin
git gc --aggressive --prune=now
```

### Step 4: Progressive Testing
Start with one repo and add more:
```bash
# Move all repos out temporarily
cd ~/git/gitgud
mkdir temp-repos
mv git-knowledge-test/* temp-repos/

# Test with one repo
mv temp-repos/[smallest-repo] git-knowledge-test/
node server.js  # Test if it works

# Add repos back one by one until you find the problematic one
```

## Expected Output
The debug server should show:
```
[2025-06-11T...] 🚀 Debug server running at http://localhost:3000
[2025-06-11T...] 🔍 Starting repository scan...
[2025-06-11T...] Checking repo1...
[2025-06-11T...] Testing git status in repo1...
[2025-06-11T...] Command completed: git status --porcelain
[2025-06-11T...] ✅ Successfully processed repo1
```

If it hangs, you'll see exactly which command and repo caused it!

## Next Steps
Once you identify the hanging operation, we can create a targeted fix for that specific git issue.
