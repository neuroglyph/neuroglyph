#!/bin/bash
# SPDX-License-Identifier: Apache-2.0

# Git Knowledge Graph - Full Stack Setup
# Real-time interactive git-based knowledge graph with chaos testing

echo "🚀 Setting up Git Knowledge Graph - The Ultimate Distributed System Demo"
echo "========================================================================"

# Check if we're in the right directory
if [ ! -d "git-knowledge-test" ]; then
    echo "❌ Error: git-knowledge-test directory not found!"
    echo "Please run this from the directory containing your git-knowledge-test folder"
    exit 1
fi

# Create web app directory
echo "📁 Creating web application structure..."
mkdir -p git-knowledge-graph
cd git-knowledge-graph

# Create package.json
cat > package.json << 'EOF'
{
  "name": "git-knowledge-graph",
  "version": "1.0.0",
  "description": "Real-time interactive git-based knowledge graph with D3.js visualization",
  "main": "server.js",
  "scripts": {
    "start": "node server.js",
    "dev": "nodemon server.js",
    "chaos": "cd ../git-knowledge-test && ./chaos-worker.sh"
  },
  "dependencies": {
    "express": "^4.18.2",
    "socket.io": "^4.7.2",
    "cors": "^2.8.5",
    "chokidar": "^3.5.3"
  },
  "devDependencies": {
    "nodemon": "^3.0.1"
  }
}
EOF

# Create server.js (the Node.js backend)
cat > server.js << 'EOF'
const express = require('express');
const http = require('http');
const socketIo = require('socket.io');
const fs = require('fs');
const path = require('path');
const { exec } = require('child_process');
const chokidar = require('chokidar');
const cors = require('cors');

const app = express();
const server = http.createServer(app);
const io = socketIo(server, {
    cors: { origin: "*", methods: ["GET", "POST"] }
});

app.use(cors());
app.use(express.json());
app.use(express.static('public'));

// Configuration
const GIT_REPOS_PATH = '../git-knowledge-test';
const REPOS = ['vault', 'projects', 'relationships'];

// In-memory graph state
let graphData = { nodes: new Map(), edges: new Map(), lastUpdate: Date.now() };
let chaosStats = { activeProcesses: 0, totalCommits: 0, lastCommit: null, commitsPerSecond: 0 };

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
            const objectList = await executeGit(repo, 'rev-list --all --objects');
            const objects = objectList.split('\n').map(line => line.split(' ')[0]).filter(Boolean);
            
            const relationships = [];
            for (const hash of objects) {
                try {
                    const content = await executeGit(repo, `cat-file -p ${hash}`);
                    if (content.includes('LINK:') || content.includes('CROSS_REF:')) {
                        relationships.push({ hash, content, repo, timestamp: Date.now() });
                    }
                } catch (e) {}
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
            return { type: match[1], source: match[2].trim(), target: match[3].trim() };
        }
        return null;
    }).filter(Boolean);
}

async function scanAllRepositories() {
    console.log('📊 Scanning repositories...');
    
    const allRelationships = [];
    for (const repo of REPOS) {
        const repoRelationships = await parseGitObjects(repo);
        allRelationships.push(...repoRelationships);
    }
    
    const newNodes = new Map();
    const newEdges = new Map();
    
    allRelationships.forEach(rel => {
        const relationships = parseRelationshipContent(rel.content);
        
        relationships.forEach(r => {
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
            
            const edgeId = `${r.source}->${r.target}`;
            newEdges.set(edgeId, {
                id: edgeId, source: r.source, target: r.target,
                type: r.type, hash: rel.hash, lastUpdated: rel.timestamp
            });
        });
    });
    
    graphData.nodes = newNodes;
    graphData.edges = newEdges;
    graphData.lastUpdate = Date.now();
    
    console.log(`📈 Found ${newNodes.size} nodes and ${newEdges.size} edges`);
    
    io.emit('graph-update', {
        nodes: Array.from(newNodes.values()),
        edges: Array.from(newEdges.values()),
        timestamp: graphData.lastUpdate,
        stats: chaosStats
    });
}

function setupFileWatchers() {
    console.log('👀 Setting up file watchers...');
    
    REPOS.forEach(repo => {
        const repoPath = path.join(GIT_REPOS_PATH, repo);
        const gitLogPath = path.join(repoPath, '.git/logs/HEAD');
        
        if (fs.existsSync(gitLogPath)) {
            chokidar.watch(gitLogPath).on('change', async () => {
                console.log(`🔄 Git activity in ${repo}`);
                chaosStats.totalCommits++;
                chaosStats.lastCommit = Date.now();
                
                clearTimeout(scanAllRepositories.timeout);
                scanAllRepositories.timeout = setTimeout(scanAllRepositories, 500);
            });
        }
        
        chokidar.watch(path.join(repoPath, '**/*.md')).on('change', (filePath) => {
            io.emit('file-change', { file: filePath, repo: repo, timestamp: Date.now() });
        });
    });
}

function trackChaosActivity() {
    setInterval(() => {
        exec('ps aux | grep chaos-worker | grep -v grep | wc -l', (error, stdout) => {
            if (!error) {
                chaosStats.activeProcesses = parseInt(stdout.trim()) || 0;
                io.emit('chaos-stats', chaosStats);
            }
        });
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
    const { name, repo } = req.body;
    
    try {
        const repoPath = path.join(GIT_REPOS_PATH, repo);
        const fileName = `${name.replace(/\s+/g, '-').toLowerCase()}.md`;
        const filePath = path.join(repoPath, fileName);
        
        const content = `# ${name}\n\nCreated via web interface at ${new Date().toISOString()}\n`;
        
        fs.writeFileSync(filePath, content);
        
        await executeGit(repo, `add "${fileName}"`);
        await executeGit(repo, `commit -m "Added ${name} via web interface"`);
        
        res.json({ success: true, file: fileName });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

app.post('/api/edge', async (req, res) => {
    const { sourceId, targetId } = req.body;
    
    try {
        const sourceRepo = sourceId.split('/')[0];
        const sourceFile = sourceId.split('/').slice(1).join('/');
        const repoPath = path.join(GIT_REPOS_PATH, sourceRepo);
        const filePath = path.join(repoPath, sourceFile);
        
        if (fs.existsSync(filePath)) {
            let content = fs.readFileSync(filePath, 'utf8');
            content += `\n\nLinked to [${path.basename(targetId)}](${targetId})\n`;
            
            fs.writeFileSync(filePath, content);
            
            await executeGit(sourceRepo, `add "${sourceFile}"`);
            await executeGit(sourceRepo, `commit -m "Added link to ${targetId} via web interface"`);
            
            res.json({ success: true });
        } else {
            res.status(404).json({ error: 'Source file not found' });
        }
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

// WebSocket handling
io.on('connection', (socket) => {
    console.log('🔌 Client connected:', socket.id);
    
    socket.emit('graph-update', {
        nodes: Array.from(graphData.nodes.values()),
        edges: Array.from(graphData.edges.values()),
        timestamp: graphData.lastUpdate,
        stats: chaosStats
    });
    
    socket.on('disconnect', () => {
        console.log('🔌 Client disconnected:', socket.id);
    });
    
    socket.on('request-update', () => {
        scanAllRepositories();
    });
});

async function initialize() {
    console.log('🚀 Starting Git Knowledge Graph Server...');
    
    await scanAllRepositories();
    setupFileWatchers();
    trackChaosActivity();
    
    const PORT = process.env.PORT || 3000;
    server.listen(PORT, () => {
        console.log(`📡 Server running on http://localhost:${PORT}`);
        console.log('🎨 Open browser to see the live graph!');
    });
}

initialize().catch(console.error);
EOF

# Create public directory and index.html
mkdir -p public
cat > public/index.html << 'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Git Knowledge Graph - LIVE 🔥</title>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/d3/7.8.5/d3.min.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/socket.io/4.7.2/socket.io.js"></script>
    <style>
        body {
            margin: 0; padding: 0;
            background: linear-gradient(135deg, #0c0c0c 0%, #1a1a2e 50%, #16213e 100%);
            font-family: 'Monaco', monospace; color: #00ff41; overflow: hidden;
        }
        .container { display: flex; height: 100vh; }
        .sidebar {
            width: 300px; background: rgba(0, 0, 0, 0.8);
            border-right: 2px solid #00ff41; padding: 20px; overflow-y: auto;
        }
        .main-graph { flex: 1; position: relative; }
        .header { text-align: center; margin-bottom: 20px; border-bottom: 1px solid #00ff41; }
        .header h1 { margin: 0; font-size: 18px; text-shadow: 0 0 10px #00ff41; animation: pulse 2s infinite; }
        @keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.7; } }
        .stats {
            background: rgba(0, 255, 65, 0.1); border: 1px solid #00ff41;
            border-radius: 5px; padding: 10px; margin-bottom: 15px;
        }
        .stats h3 { margin: 0 0 10px 0; font-size: 14px; color: #ff6b35; }
        .stat-item { display: flex; justify-content: space-between; margin: 5px 0; font-size: 12px; }
        .stat-value { color: #ffff00; font-weight: bold; }
        .controls { margin-bottom: 20px; }
        .control-group { margin-bottom: 15px; }
        .control-group label { display: block; margin-bottom: 5px; font-size: 12px; color: #ff6b35; }
        .control-group input, .control-group select, .control-group button {
            width: 100%; padding: 8px; background: rgba(0, 0, 0, 0.8);
            border: 1px solid #00ff41; color: #00ff41; border-radius: 3px;
            font-family: inherit; font-size: 12px;
        }
        .control-group button {
            background: rgba(0, 255, 65, 0.2); cursor: pointer; transition: background 0.3s;
        }
        .control-group button:hover { background: rgba(0, 255, 65, 0.4); }
        .log {
            background: rgba(0, 0, 0, 0.9); border: 1px solid #00ff41;
            border-radius: 5px; padding: 10px; height: 200px; overflow-y: auto;
            font-size: 10px; line-height: 1.3;
        }
        .log-entry { margin: 2px 0; opacity: 0.8; }
        .log-entry.chaos { color: #ff6b35; }
        .log-entry.git { color: #00ff41; }
        .log-entry.user { color: #ffff00; }
        .node { cursor: pointer; transition: all 0.3s; }
        .node:hover { stroke-width: 3px; }
        .node.vault { fill: #00ff41; stroke: #00cc33; }
        .node.projects { fill: #ff6b35; stroke: #cc4422; }
        .node.relationships { fill: #ffff00; stroke: #cccc00; }
        .node.selected { stroke: #ffffff; stroke-width: 4px; }
        .link { stroke: #00ff41; stroke-opacity: 0.6; stroke-width: 2px; }
        .link.CROSS_REF { stroke: #ff6b35; stroke-dasharray: 5,5; }
        .node-label {
            fill: #ffffff; font-size: 10px; text-anchor: middle;
            dominant-baseline: central; pointer-events: none;
        }
        .chaos-indicator {
            position: absolute; top: 20px; right: 20px;
            background: rgba(255, 107, 53, 0.9); color: white;
            padding: 10px 15px; border-radius: 25px; display: none;
        }
        .chaos-indicator.active { display: block; animation: chaosGlow 1s infinite alternate; }
        @keyframes chaosGlow { from { box-shadow: 0 0 10px #ff6b35; } to { box-shadow: 0 0 30px #ff6b35; } }
        .tooltip {
            position: absolute; background: rgba(0, 0, 0, 0.9);
            border: 1px solid #00ff41; border-radius: 5px; padding: 10px;
            pointer-events: none; font-size: 12px; z-index: 1000; opacity: 0;
        }
        .connection-status {
            position: absolute; top: 10px; left: 10px; padding: 5px 10px;
            border-radius: 15px; font-size: 12px; font-weight: bold;
        }
        .connection-status.connected {
            background: rgba(0, 255, 65, 0.2); border: 1px solid #00ff41; color: #00ff41;
        }
        .connection-status.disconnected {
            background: rgba(255, 107, 53, 0.2); border: 1px solid #ff6b35; color: #ff6b35;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="sidebar">
            <div class="header">
                <h1>🔥 LIVE GIT GRAPH 🔥</h1>
                <div>Real-time Knowledge Network</div>
            </div>

            <div class="stats">
                <h3>📊 CHAOS METRICS</h3>
                <div class="stat-item"><span>Active Monkeys:</span><span class="stat-value" id="active-processes">0</span></div>
                <div class="stat-item"><span>Total Commits:</span><span class="stat-value" id="total-commits">0</span></div>
                <div class="stat-item"><span>Nodes:</span><span class="stat-value" id="node-count">0</span></div>
                <div class="stat-item"><span>Edges:</span><span class="stat-value" id="edge-count">0</span></div>
            </div>

            <div class="controls">
                <div class="control-group">
                    <label>Add Node:</label>
                    <input type="text" id="new-node-name" placeholder="Node name">
                    <select id="new-node-repo">
                        <option value="vault">Vault</option>
                        <option value="projects">Projects</option>
                    </select>
                    <button onclick="addNode()">Create Node</button>
                </div>

                <div class="control-group">
                    <label>Add Edge:</label>
                    <select id="edge-source"><option value="">Select source...</option></select>
                    <select id="edge-target"><option value="">Select target...</option></select>
                    <button onclick="addEdge()">Create Link</button>
                </div>

                <div class="control-group">
                    <button onclick="requestUpdate()">🔄 Refresh Graph</button>
                </div>
            </div>

            <div class="log" id="activity-log">
                <div class="log-entry">📡 Connecting to git knowledge graph...</div>
            </div>
        </div>

        <div class="main-graph">
            <div class="connection-status disconnected" id="connection-status">DISCONNECTED</div>
            <div class="chaos-indicator" id="chaos-indicator">🐒 CHAOS ACTIVE</div>
            <div class="graph-container" id="graph-container"></div>
            <div class="tooltip" id="tooltip"></div>
        </div>
    </div>

    <script>
        let socket, graphData = { nodes: [], edges: [] }, simulation, svg, nodeGroup, linkGroup, labelGroup;

        function init() {
            setupWebSocket();
            setupD3();
        }

        function setupWebSocket() {
            socket = io();
            
            socket.on('connect', () => {
                logActivity('🔌 Connected to server', 'git');
                document.getElementById('connection-status').className = 'connection-status connected';
                document.getElementById('connection-status').textContent = 'CONNECTED';
            });

            socket.on('disconnect', () => {
                logActivity('🔌 Disconnected', 'git');
                document.getElementById('connection-status').className = 'connection-status disconnected';
                document.getElementById('connection-status').textContent = 'DISCONNECTED';
            });

            socket.on('graph-update', (data) => {
                logActivity(`📊 Graph: ${data.nodes.length} nodes, ${data.edges.length} edges`, 'git');
                updateGraph(data);
            });

            socket.on('chaos-stats', (stats) => {
                document.getElementById('active-processes').textContent = stats.activeProcesses || 0;
                document.getElementById('total-commits').textContent = stats.totalCommits || 0;
                
                if (stats.activeProcesses > 0) {
                    document.getElementById('chaos-indicator').classList.add('active');
                } else {
                    document.getElementById('chaos-indicator').classList.remove('active');
                }
            });

            socket.on('file-change', (data) => {
                logActivity(`📝 File changed: ${data.file}`, 'chaos');
            });
        }

        function setupD3() {
            const container = d3.select('#graph-container');
            const width = container.node().clientWidth;
            const height = container.node().clientHeight;

            svg = container.append('svg').attr('width', width).attr('height', height);
            linkGroup = svg.append('g').attr('class', 'links');
            nodeGroup = svg.append('g').attr('class', 'nodes');
            labelGroup = svg.append('g').attr('class', 'labels');

            simulation = d3.forceSimulation()
                .force('link', d3.forceLink().id(d => d.id).distance(100))
                .force('charge', d3.forceManyBody().strength(-300))
                .force('center', d3.forceCenter(width / 2, height / 2));
        }

        function updateGraph(data) {
            graphData = data;
            document.getElementById('node-count').textContent = data.nodes.length;
            document.getElementById('edge-count').textContent = data.edges.length;
            
            updateDropdowns(data.nodes);
            renderGraph();
        }

        function renderGraph() {
            const link = linkGroup.selectAll('.link').data(graphData.edges, d => d.id);
            link.exit().remove();
            const linkEnter = link.enter().append('line').attr('class', d => `link ${d.type}`);
            link.merge(linkEnter);

            const node = nodeGroup.selectAll('.node').data(graphData.nodes, d => d.id);
            node.exit().remove();
            const nodeEnter = node.enter().append('circle')
                .attr('class', d => `node ${d.repo}`).attr('r', 15)
                .on('click', selectNode).on('mouseover', showNodeTooltip).on('mouseout', hideTooltip);
            node.merge(nodeEnter);

            const label = labelGroup.selectAll('.node-label').data(graphData.nodes, d => d.id);
            label.exit().remove();
            const labelEnter = label.enter().append('text')
                .attr('class', 'node-label').text(d => d.name);
            label.merge(labelEnter);

            simulation.nodes(graphData.nodes);
            simulation.force('link').links(graphData.edges);
            
            simulation.on('tick', () => {
                linkGroup.selectAll('.link')
                    .attr('x1', d => d.source.x).attr('y1', d => d.source.y)
                    .attr('x2', d => d.target.x).attr('y2', d => d.target.y);
                nodeGroup.selectAll('.node').attr('cx', d => d.x).attr('cy', d => d.y);
                labelGroup.selectAll('.node-label').attr('x', d => d.x).attr('y', d => d.y + 25);
            });

            simulation.alpha(0.3).restart();
        }

        function updateDropdowns(nodes) {
            const sourceSelect = document.getElementById('edge-source');
            const targetSelect = document.getElementById('edge-target');
            
            sourceSelect.innerHTML = '<option value="">Select source...</option>';
            targetSelect.innerHTML = '<option value="">Select target...</option>';
            
            nodes.forEach(node => {
                sourceSelect.add(new Option(node.name, node.id));
                targetSelect.add(new Option(node.name, node.id));
            });
        }

        function selectNode(event, d) {
            nodeGroup.selectAll('.node').classed('selected', false);
            d3.select(event.currentTarget).classed('selected', true);
            logActivity(`🎯 Selected: ${d.name}`, 'user');
        }

        function showNodeTooltip(event, d) {
            const tooltip = document.getElementById('tooltip');
            tooltip.innerHTML = `<strong>${d.name}</strong><br>Repo: ${d.repo}<br>Path: ${d.fullPath}`;
            tooltip.style.left = (event.pageX + 10) + 'px';
            tooltip.style.top = (event.pageY + 10) + 'px';
            tooltip.style.opacity = 1;
        }

        function hideTooltip() {
            document.getElementById('tooltip').style.opacity = 0;
        }

        function logActivity(message, type = 'git') {
            const log = document.getElementById('activity-log');
            const entry = document.createElement('div');
            entry.className = `log-entry ${type}`;
            entry.textContent = `[${new Date().toLocaleTimeString()}] ${message}`;
            log.appendChild(entry);
            log.scrollTop = log.scrollHeight;
            
            while (log.children.length > 50) {
                log.removeChild(log.firstChild);
            }
        }

        async function addNode() {
            const name = document.getElementById('new-node-name').value.trim();
            const repo = document.getElementById('new-node-repo').value;
            
            if (!name) return;
            
            try {
                const response = await fetch('/api/node', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ name, repo })
                });
                
                const result = await response.json();
                if (result.success) {
                    logActivity(`✨ Created: ${name}`, 'user');
                    document.getElementById('new-node-name').value = '';
                }
            } catch (error) {
                alert('Error: ' + error.message);
            }
        }

        async function addEdge() {
            const sourceId = document.getElementById('edge-source').value;
            const targetId = document.getElementById('edge-target').value;
            
            if (!sourceId || !targetId) return;
            
            try {
                const response = await fetch('/api/edge', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ sourceId, targetId })
                });
                
                const result = await response.json();
                if (result.success) {
                    logActivity(`🔗 Linked: ${sourceId} → ${targetId}`, 'user');
                }
            } catch (error) {
                alert('Error: ' + error.message);
            }
        }

        function requestUpdate() {
            socket.emit('request-update');
            logActivity('🔄 Refresh requested', 'user');
        }

        window.addEventListener('load', init);
    </script>
</body>
</html>
EOF

echo "📦 Installing dependencies..."
npm install

echo ""
echo "🎉 SETUP COMPLETE!"
echo "=================="
echo ""
echo "🚀 TO START THE EPIC DEMO:"
echo ""
echo "1. Start the web server:"
echo "   cd git-knowledge-graph"
echo "   npm start"
echo ""
echo "2. Open browser to: http://localhost:3000"
echo ""
echo "3. (Optional) Start chaos monkeys in another terminal:"
echo "   cd git-knowledge-test"
echo "   ./chaos-worker.sh"
echo ""
echo "🔥 FEATURES:"
echo "• Real-time D3.js graph visualization"
echo "• Interactive node/edge editing"
echo "• Live updates from git commits"  
echo "• Chaos monkey activity monitoring"
echo "• WebSocket-powered real-time sync"
echo ""
echo "You just built a full-stack distributed knowledge graph!"
echo "This is genuinely revolutionary technology. 🚀"
EOF

chmod +x setup.sh

echo "🎯 BOOM! Full-stack Git Knowledge Graph setup complete!"
echo ""
echo "📁 FILES CREATED:"
echo "   • server.js (Node.js backend with WebSocket)"
echo "   • public/index.html (D3.js interactive frontend)"
echo "   • package.json (dependencies)"
echo "   • setup.sh (automated setup script)"
echo ""
echo "🚀 TO DEPLOY THE ULTIMATE DEMO:"
echo ""
echo "1. Run the setup script:"
echo "   ./setup.sh"
echo ""
echo "2. This will:"
echo "   • Create the web app directory structure"
echo "   • Install all Node.js dependencies"
echo "   • Set up the real-time server"
echo "   • Configure the interactive D3 visualization"
echo ""
echo "3. Start the server and open browser to see:"
echo "   • Live graph visualization of your git relationships"
echo "   • Real-time updates as chaos monkeys make changes"
echo "   • Interactive node/edge editing"
echo "   • WebSocket-powered collaborative experience"
echo ""
echo "**THIS IS THE GRAND FINALE** - a full-stack interactive demonstration"
echo "of our revolutionary git-based distributed knowledge graph system!"
echo ""
echo "Ready to blow some minds with this demo? 🤯"