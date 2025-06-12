const express = require('express');
const { execSync, exec } = require('child_process');
const path = require('path');
const fs = require('fs');
const http = require('http');
const { Server } = require('socket.io');

const app = express();
const port = 3000;

// Store manual edges and nodes
let manualEdges = [];
let manualNodes = [];

// Enhanced logging with timestamps
function log(message) {
    const timestamp = new Date().toISOString();
    console.log(`[${timestamp}] ${message}`);
}

function executeGitCommand(command, cwd, timeout = 5000) {
    return new Promise((resolve, reject) => {
        log(`Executing: ${command} in ${cwd}`);

        let timeoutHandle;

        const child = exec(command, {
            cwd,
            timeout,
            killSignal: 'SIGKILL'
        }, (error, stdout, stderr) => {
            clearTimeout(timeoutHandle);
            if (error) {
                log(`Command failed: ${command} - ${error.message}`);
                reject(error);
            } else {
                log(`Command completed: ${command}`);
                resolve(stdout.trim());
            }
        });

        // Force kill after timeout
        timeoutHandle = setTimeout(() => {
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

    const testDir = path.join(process.cwd(), 'git-knowledge-test');
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
                    commits: logOutput.split('\n').length,
                    repo: entry,  // Add this
                    fullPath: repoPath,  // Add this
                    lastUpdated: new Date().toISOString()  // Add this
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
    const gitNodes = repos.map((repo, i) => ({
        id: i,
        name: repo.name,
        type: 'repository',
        commits: repo.commits,
        repo: repo.repo,
        fullPath: repo.fullPath,
        lastUpdated: repo.lastUpdated
    }));
    
    // Combine git nodes with manual nodes
    const allNodes = [...gitNodes, ...manualNodes];

    return { nodes: allNodes, links: manualEdges };
}

app.use(express.json());
app.use(express.static(path.join(__dirname, '..', 'webapp')));

const server = http.createServer(app);
const io = new Server(server, {
    cors: {
        origin: "*",
        methods: ["GET", "POST"]
    }
});

io.on('connection', async (socket) => {
    log('Client connected via WebSocket');

    // Send graph data immediately when client connects
    try {
        const data = await scanRepositories();
        log('WebSocket data being sent: ' + JSON.stringify(data, null, 2));
        socket.emit('graph-update', {
            nodes: data.nodes,
            edges: data.links,  // Frontend expects 'edges', not 'links'
            stats: {
                activeProcesses: 0,
                totalCommits: data.nodes.reduce((sum, node) => sum + (node.commits || 0), 0),
                commitsPerSecond: 0
            }
        });
        log('WebSocket emit completed');
        log(`Sent ${data.nodes.length} nodes via WebSocket`);
    } catch (error) {
        log(`WebSocket error: ${error.message}`);
        socket.emit('graph-update', {
            nodes: [],
            edges: [],
            stats: { activeProcesses: 0, totalCommits: 0, commitsPerSecond: 0 }
        });
    }
    
    // Handle refresh requests
    socket.on('request-update', async () => {
        log('Refresh request received');
        try {
            const data = await scanRepositories();
            socket.emit('graph-update', {
                nodes: data.nodes,
                edges: data.links,
                stats: {
                    activeProcesses: 0,
                    totalCommits: data.nodes.reduce((sum, node) => sum + (node.commits || 0), 0),
                    commitsPerSecond: 0
                }
            });
        } catch (error) {
            log(`Refresh error: ${error.message}`);
        }
    });
});

// Add these routes before app.listen
app.post('/api/node', async (req, res) => {
    log('Manual node creation request');
    const { name, repo } = req.body;
    
    // Get current node count to assign unique ID
    const data = await scanRepositories();
    const nextId = data.nodes.length;
    
    // Create new manual node
    const node = {
        id: nextId,
        name: name,
        type: 'manual',
        commits: 0,
        repo: repo,
        fullPath: `manual-node-${name}`,
        lastUpdated: new Date().toISOString()
    };
    
    manualNodes.push(node);
    log(`Created manual node: ${name} in ${repo}`);
    
    // Broadcast updated graph to all clients
    const updatedData = await scanRepositories();
    io.emit('graph-update', {
        nodes: updatedData.nodes,
        edges: updatedData.links,
        stats: {
            activeProcesses: 0,
            totalCommits: updatedData.nodes.reduce((sum, node) => sum + (node.commits || 0), 0),
            commitsPerSecond: 0
        }
    });
    
    res.json({ success: true, id: node.id });
});

app.post('/api/edge', async (req, res) => {
    log('Manual edge creation request');
    const { sourceId, targetId } = req.body;
    
    // Create new edge
    const edge = {
        id: `edge-${Date.now()}`,
        source: parseInt(sourceId),
        target: parseInt(targetId),
        type: 'MANUAL',
        hash: `manual-${Date.now()}`,
        lastUpdated: new Date().toISOString()
    };
    
    manualEdges.push(edge);
    log(`Created edge: ${sourceId} → ${targetId}`);
    
    // Broadcast updated graph to all clients
    const data = await scanRepositories();
    io.emit('graph-update', {
        nodes: data.nodes,
        edges: data.links,
        stats: {
            activeProcesses: 0,
            totalCommits: data.nodes.reduce((sum, node) => sum + (node.commits || 0), 0),
            commitsPerSecond: 0
        }
    });
    
    res.json({ success: true, id: edge.id });
});

app.get('/api/graph-data', async (req, res) => {
    try {
        log('📡 Graph data request received');
        const data = await scanRepositories();
        log('📤 Sending response with ' + data.nodes.length + ' nodes');
        res.json(data);
    } catch (error) {
        log('❌ Error in /api/graph-data: ' + error.message);
        res.status(500).json({ error: error.message, nodes: [], links: [] });
    }
});

server.listen(port, () => {
    log(`🚀 Debug server running at http://localhost:${port}`);
    log('📁 Current directory: ' + process.cwd());
    log('📁 Test directory should be at: ' + path.join(process.cwd(), 'git-knowledge-test'));
});