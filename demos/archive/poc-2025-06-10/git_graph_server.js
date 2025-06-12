// Git Knowledge Graph Server
// Real-time collaborative git-based knowledge graph with D3 visualization

const express = require('express');
const http = require('http');
const socketIo = require('socket.io');
const fs = require('fs');
const path = require('path');
const { exec, spawn } = require('child_process');
const chokidar = require('chokidar');
const cors = require('cors');

const app = express();
const server = http.createServer(app);
const io = socketIo(server, {
    cors: {
        origin: "*",
        methods: ["GET", "POST"]
    }
});

app.use(cors());
app.use(express.json());
app.use(express.static('public'));

// Configuration
const GIT_REPOS_PATH = './git-knowledge-test';
const REPOS = ['vault', 'projects', 'relationships'];

// In-memory graph state
let graphData = {
    nodes: new Map(),
    edges: new Map(),
    lastUpdate: Date.now()
};

// Chaos monkey tracking
let chaosStats = {
    activeProcesses: 0,
    totalCommits: 0,
    lastCommit: null,
    commitsPerSecond: 0
};

// Git helper functions
function executeGit(repo, command) {
    return new Promise((resolve, reject) => {
        const repoPath = path.join(GIT_REPOS_PATH, repo);
        exec(`cd "${repoPath}" && git ${command}`, (error, stdout, stderr) => {
            if (error && !error.message.includes('index.lock')) {
                reject(error);
            } else {
                resolve(stdout.trim());
            }
        });
    });
}

function parseGitObjects(repo) {
    return new Promise(async (resolve) => {
        try {
            // Get all git objects that look like relationships
            const objectList = await executeGit(repo, 'rev-list --all --objects');
            const objects = objectList.split('\n').map(line => line.split(' ')[0]).filter(Boolean);
            
            const relationships = [];
            for (const hash of objects) {
                try {
                    const content = await executeGit(repo, `cat-file -p ${hash}`);
                    if (content.includes('LINK:') || content.includes('CROSS_REF:')) {
                        relationships.push({
                            hash,
                            content,
                            repo,
                            timestamp: Date.now()
                        });
                    }
                } catch (e) {
                    // Skip objects we can't read
                }
            }
            resolve(relationships);
        } catch (error) {
            resolve([]);
        }
    });
}

function parseRelationshipContent(content) {
    const lines = content.split('\n').filter(line => 
        line.includes('LINK:') || line.includes('CROSS_REF:')
    );
    
    return lines.map(line => {
        const match = line.match(/(LINK|CROSS_REF): (.+?) -> (.+)/);
        if (match) {
            return {
                type: match[1],
                source: match[2].trim(),
                target: match[3].trim()
            };
        }
        return null;
    }).filter(Boolean);
}

async function scanAllRepositories() {
    console.log('📊 Scanning all repositories for relationships...');
    
    const allRelationships = [];
    for (const repo of REPOS) {
        const repoRelationships = await parseGitObjects(repo);
        allRelationships.push(...repoRelationships);
    }
    
    // Build graph from relationships
    const newNodes = new Map();
    const newEdges = new Map();
    
    allRelationships.forEach(rel => {
        const relationships = parseRelationshipContent(rel.content);
        
        relationships.forEach(r => {
            // Add nodes
            if (!newNodes.has(r.source)) {
                newNodes.set(r.source, {
                    id: r.source,
                    name: path.basename(r.source),
                    fullPath: r.source,
                    repo: r.source.split('/')[0] || 'unknown',
                    lastUpdated: rel.timestamp
                });
            }
            
            if (!newNodes.has(r.target)) {
                newNodes.set(r.target, {
                    id: r.target,
                    name: path.basename(r.target),
                    fullPath: r.target,
                    repo: r.target.split('/')[0] || 'unknown',
                    lastUpdated: rel.timestamp
                });
            }
            
            // Add edge
            const edgeId = `${r.source}->${r.target}`;
            newEdges.set(edgeId, {
                id: edgeId,
                source: r.source,
                target: r.target,
                type: r.type,
                hash: rel.hash,
                lastUpdated: rel.timestamp
            });
        });
    });
    
    // Update global state
    graphData.nodes = newNodes;
    graphData.edges = newEdges;
    graphData.lastUpdate = Date.now();
    
    console.log(`📈 Found ${newNodes.size} nodes and ${newEdges.size} edges`);
    
    // Broadcast to all clients
    io.emit('graph-update', {
        nodes: Array.from(newNodes.values()),
        edges: Array.from(newEdges.values()),
        timestamp: graphData.lastUpdate,
        stats: chaosStats
    });
}

// File watcher for real-time updates
function setupFileWatchers() {
    console.log('👀 Setting up file watchers...');
    
    REPOS.forEach(repo => {
        const repoPath = path.join(GIT_REPOS_PATH, repo);
        
        // Watch for git commits (changes to .git/logs/HEAD)
        const gitLogPath = path.join(repoPath, '.git/logs/HEAD');
        
        if (fs.existsSync(gitLogPath)) {
            chokidar.watch(gitLogPath).on('change', async () => {
                console.log(`🔄 Git activity detected in ${repo}`);
                chaosStats.totalCommits++;
                chaosStats.lastCommit = Date.now();
                
                // Debounce rapid updates
                clearTimeout(scanAllRepositories.timeout);
                scanAllRepositories.timeout = setTimeout(scanAllRepositories, 500);
            });
        }
        
        // Watch for file changes
        chokidar.watch(path.join(repoPath, '**/*.md')).on('change', (filePath) => {
            console.log(`📝 File changed: ${filePath}`);
            io.emit('file-change', {
                file: filePath,
                repo: repo,
                timestamp: Date.now()
            });
        });
    });
}

// Chaos monkey tracking
function trackChaosActivity() {
    setInterval(() => {
        const now = Date.now();
        
        // Count active bash processes (chaos monkeys)
        exec('ps aux | grep chaos-worker | grep -v grep | wc -l', (error, stdout) => {
            if (!error) {
                chaosStats.activeProcesses = parseInt(stdout.trim()) || 0;
            }
        });
        
        // Calculate commits per second
        if (chaosStats.lastCommit && now - chaosStats.lastCommit < 10000) {
            // Recent activity
            chaosStats.commitsPerSecond = chaosStats.totalCommits / ((now - chaosStats.lastCommit) / 1000);
        } else {
            chaosStats.commitsPerSecond = 0;
        }
        
        // Broadcast stats
        io.emit('chaos-stats', chaosStats);
    }, 2000);
}

// API Routes
app.get('/api/graph', async (req, res) => {
    res.json({
        nodes: Array.from(graphData.nodes.values()),
        edges: Array.from(graphData.edges.values()),
        lastUpdate: graphData.lastUpdate,
        stats: chaosStats
    });
});

app.post('/api/node', async (req, res) => {
    const { id, name, repo } = req.body;
    
    try {
        // Create a new markdown file
        const repoPath = path.join(GIT_REPOS_PATH, repo);
        const fileName = `${name.replace(/\s+/g, '-').toLowerCase()}.md`;
        const filePath = path.join(repoPath, fileName);
        
        const content = `# ${name}\n\nCreated via web interface at ${new Date().toISOString()}\n`;
        
        fs.writeFileSync(filePath, content);
        
        // Commit the file
        await executeGit(repo, `add "${fileName}"`);
        await executeGit(repo, `commit -m "Added ${name} via web interface"`);
        
        res.json({ success: true, file: fileName });
    } catch (error) {
        console.error('Error creating node:', error);
        res.status(500).json({ error: error.message });
    }
});

app.post('/api/edge', async (req, res) => {
    const { sourceId, targetId, type = 'LINK' } = req.body;
    
    try {
        // Find the source file and add a link to target
        const sourceRepo = sourceId.split('/')[0];
        const sourceFile = sourceId.split('/').slice(1).join('/');
        const repoPath = path.join(GIT_REPOS_PATH, sourceRepo);
        const filePath = path.join(repoPath, sourceFile);
        
        if (fs.existsSync(filePath)) {
            let content = fs.readFileSync(filePath, 'utf8');
            
            // Add link to the end of the file
            const linkText = `\n\nLinked to [${path.basename(targetId)}](${targetId})\n`;
            content += linkText;
            
            fs.writeFileSync(filePath, content);
            
            // Commit the change
            await executeGit(sourceRepo, `add "${sourceFile}"`);
            await executeGit(sourceRepo, `commit -m "Added link to ${targetId} via web interface"`);
            
            res.json({ success: true });
        } else {
            res.status(404).json({ error: 'Source file not found' });
        }
    } catch (error) {
        console.error('Error creating edge:', error);
        res.status(500).json({ error: error.message });
    }
});

// WebSocket connection handling
io.on('connection', (socket) => {
    console.log('🔌 Client connected:', socket.id);
    
    // Send current graph state
    socket.emit('graph-update', {
        nodes: Array.from(graphData.nodes.values()),
        edges: Array.from(graphData.edges.values()),
        timestamp: graphData.lastUpdate,
        stats: chaosStats
    });
    
    socket.on('disconnect', () => {
        console.log('🔌 Client disconnected:', socket.id);
    });
    
    // Handle client requests for graph updates
    socket.on('request-update', () => {
        scanAllRepositories();
    });
});

// Initialize
async function initialize() {
    console.log('🚀 Starting Git Knowledge Graph Server...');
    
    // Initial scan
    await scanAllRepositories();
    
    // Setup watchers
    setupFileWatchers();
    
    // Start chaos tracking
    trackChaosActivity();
    
    const PORT = process.env.PORT || 3000;
    server.listen(PORT, () => {
        console.log(`📡 Server running on http://localhost:${PORT}`);
        console.log('🎨 Open the web interface to see the live graph!');
        console.log('🐒 Chaos monkeys will update the graph in real-time');
    });
}

// Graceful shutdown
process.on('SIGINT', () => {
    console.log('\n🛑 Shutting down server...');
    server.close(() => {
        console.log('👋 Server stopped');
        process.exit(0);
    });
});

initialize().catch(console.error);

module.exports = app;