# F022: Time Travel Interface

**Status:** PLANNED  
**Priority:** Medium (Phase 2)  
**Effort:** 1 week  
**Dependencies:** F020 (Graph Visualization), F019 (Web Server)  

## Summary
A timeline-based interface that allows users to navigate through the history of their knowledge graph, visualizing how connections evolved over time with smooth animations between states.

## User Stories

### 1. The Knowledge Archaeologist
As a long-time project maintainer, I want to see how our architecture evolved over the past year, so I can understand what decisions led to our current state.

### 2. The Blame Detective
As a developer debugging an issue, I want to see when specific connections were created and by whom, so I can understand the context of architectural decisions.

### 3. The Pattern Analyst
As a team lead, I want to identify periods of rapid change or stability in our knowledge graph, so I can correlate them with project events.

### 4. The Visual Historian
As someone giving a presentation, I want to replay the evolution of our project structure, so I can tell the story of how we got here.

## Acceptance Criteria

### 1. Timeline Navigation
- [ ] Horizontal timeline showing commit history
- [ ] Scrubber for smooth navigation
- [ ] Play/pause for automatic playback
- [ ] Speed controls (0.5x to 5x)
- [ ] Jump to specific dates
- [ ] Keyboard shortcuts (←/→ for prev/next)

### 2. Visual Representation
- [ ] Commits shown as points on timeline
- [ ] Density visualization for busy periods
- [ ] Color coding by change type
- [ ] Tooltip previews on hover
- [ ] Current position indicator
- [ ] Date/time display

### 3. Graph Animation
- [ ] Smooth transitions between states
- [ ] Nodes fade in/out appropriately
- [ ] Links animate position changes
- [ ] Morphing for file renames
- [ ] Highlight changes in each transition
- [ ] Optional motion blur for speed

### 4. Change Highlighting
- [ ] New nodes pulse green
- [ ] Deleted nodes fade red
- [ ] Modified links flash
- [ ] Diff panel showing changes
- [ ] Change statistics overlay
- [ ] Author attribution

### 5. Filtering and Focus
- [ ] Filter by author
- [ ] Filter by date range
- [ ] Filter by file pattern
- [ ] Filter by link type
- [ ] Show only changes
- [ ] Focus on specific subgraph

### 6. Performance
- [ ] Lazy load commit data
- [ ] Cache computed layouts
- [ ] Smooth 60 FPS playback
- [ ] Handle repos with 10K+ commits
- [ ] Progressive detail loading
- [ ] Background precomputation

## Technical Design

### Architecture
```javascript
class TimeTravelEngine {
  constructor(graphEngine, gitAPI) {
    this.graphEngine = graphEngine;
    this.gitAPI = gitAPI;
    this.timeline = new Timeline();
    this.cache = new LayoutCache();
    this.animator = new GraphAnimator();
  }

  async loadHistory(startDate, endDate) {
    const commits = await this.gitAPI.getCommits(startDate, endDate);
    const graphStates = await this.computeGraphStates(commits);
    this.timeline.setData(graphStates);
  }

  async navigateToCommit(commitSha) {
    const fromState = this.currentState;
    const toState = await this.getGraphState(commitSha);
    
    const animation = this.animator.createTransition(fromState, toState);
    await animation.play();
    
    this.currentState = toState;
    this.timeline.setPosition(commitSha);
  }
}
```

### Timeline Component
```javascript
class Timeline {
  constructor(container) {
    this.scale = d3.scaleTime();
    this.brush = d3.brushX();
    this.zoom = d3.zoom();
    
    this.svg = d3.select(container)
      .append('svg')
      .attr('class', 'timeline');
  }

  render(commits) {
    // Create commit density heatmap
    const density = this.computeDensity(commits);
    
    // Render timeline axis
    const axis = d3.axisBottom(this.scale)
      .tickFormat(d3.timeFormat('%Y-%m-%d'));
    
    // Add commit markers
    const markers = this.svg.selectAll('.commit')
      .data(commits)
      .enter()
      .append('circle')
      .attr('class', d => `commit ${d.changeType}`)
      .attr('cx', d => this.scale(d.date))
      .attr('cy', 20)
      .attr('r', d => Math.sqrt(d.changeCount));
    
    // Add brush for selection
    this.svg.append('g')
      .attr('class', 'brush')
      .call(this.brush);
  }
}
```

### Graph State Management
```javascript
class GraphStateManager {
  async getGraphAtCommit(commitSha) {
    // Check cache first
    if (this.cache.has(commitSha)) {
      return this.cache.get(commitSha);
    }
    
    // Compute graph state
    const files = await this.gitAPI.getFilesAtCommit(commitSha);
    const links = await this.gitAPI.getLinksAtCommit(commitSha);
    
    const graphState = {
      commit: commitSha,
      date: await this.gitAPI.getCommitDate(commitSha),
      nodes: this.processNodes(files),
      links: this.processLinks(links),
      layout: await this.computeLayout(files, links)
    };
    
    this.cache.set(commitSha, graphState);
    return graphState;
  }
}
```

### Animation System
```javascript
class GraphAnimator {
  createTransition(fromState, toState) {
    const changes = this.computeChanges(fromState, toState);
    
    return {
      duration: this.calculateDuration(changes),
      
      play: async () => {
        // Animate deletions first
        await this.animateDeletions(changes.deleted);
        
        // Then modifications
        await this.animateModifications(changes.modified);
        
        // Finally additions
        await this.animateAdditions(changes.added);
        
        // Update layout
        await this.animateLayout(fromState.layout, toState.layout);
      }
    };
  }

  animateAdditions(nodes) {
    return new Promise(resolve => {
      d3.selectAll(nodes)
        .style('opacity', 0)
        .transition()
        .duration(500)
        .style('opacity', 1)
        .on('end', resolve);
    });
  }
}
```

### Diff Visualization
```javascript
class DiffPanel {
  showDiff(fromState, toState) {
    const changes = {
      added: toState.nodes.filter(n => !fromState.nodes.find(f => f.id === n.id)),
      removed: fromState.nodes.filter(n => !toState.nodes.find(t => t.id === n.id)),
      modified: this.findModified(fromState, toState)
    };

    this.render({
      title: `Changes in ${toState.commit.slice(0, 7)}`,
      author: toState.author,
      date: toState.date,
      stats: {
        nodesAdded: changes.added.length,
        nodesRemoved: changes.removed.length,
        linksChanged: changes.modified.links.length
      },
      details: changes
    });
  }
}
```

### Performance Optimizations

#### Layout Caching
```javascript
class LayoutCache {
  constructor(maxSize = 100) {
    this.cache = new LRUCache(maxSize);
    this.precomputing = new Set();
  }

  async precomputeRange(commits) {
    // Background computation
    for (const commit of commits) {
      if (!this.cache.has(commit.sha) && !this.precomputing.has(commit.sha)) {
        this.precomputing.add(commit.sha);
        
        // Non-blocking computation
        requestIdleCallback(() => {
          this.computeLayout(commit).then(layout => {
            this.cache.set(commit.sha, layout);
            this.precomputing.delete(commit.sha);
          });
        });
      }
    }
  }
}
```

#### Incremental Updates
```javascript
function computeGraphDelta(previousState, currentCommit) {
  // Only compute what changed
  const changedFiles = getChangedFiles(previousState.commit, currentCommit);
  
  // Reuse unchanged portions
  const unchangedNodes = previousState.nodes.filter(n => 
    !changedFiles.includes(n.id)
  );
  
  // Only recompute layout for affected subgraph
  return {
    ...previousState,
    nodes: [...unchangedNodes, ...computeNewNodes(changedFiles)],
    partialLayout: true
  };
}
```

## User Interface Design

### Timeline Controls
```html
<div class="timeline-controls">
  <button class="play-pause" aria-label="Play/Pause">▶️</button>
  <input type="range" class="speed-control" min="0.5" max="5" step="0.5" value="1">
  <span class="current-date">2025-06-12</span>
  
  <div class="timeline-container">
    <svg class="timeline-visualization"></svg>
    <div class="scrubber"></div>
  </div>
  
  <div class="filter-controls">
    <input type="text" placeholder="Filter by author...">
    <input type="date" class="date-filter">
    <select class="change-type-filter">
      <option value="all">All Changes</option>
      <option value="additions">Additions Only</option>
      <option value="deletions">Deletions Only</option>
    </select>
  </div>
</div>
```

### Visual Styles
```css
.timeline-visualization {
  height: 80px;
  background: linear-gradient(to bottom, #f0f0f0, #ffffff);
}

.commit {
  transition: all 0.2s ease;
  cursor: pointer;
}

.commit:hover {
  stroke: #4A90E2;
  stroke-width: 2;
}

.commit.addition { fill: #7ED321; }
.commit.deletion { fill: #D0021B; }
.commit.modification { fill: #F5A623; }

.node-entering {
  animation: pulse 0.6s ease-out;
}

.node-exiting {
  animation: fadeOut 0.4s ease-out forwards;
}

@keyframes pulse {
  0% { transform: scale(0); opacity: 0; }
  50% { transform: scale(1.2); }
  100% { transform: scale(1); opacity: 1; }
}
```

## Testing Strategy

### Performance Tests
- Load 10K commit history
- Measure animation FPS
- Cache hit rates
- Memory usage over time
- Layout computation time

### Interaction Tests
- Timeline scrubbing accuracy
- Keyboard navigation
- Filter combinations
- Playback controls
- Browser compatibility

### Visual Tests
- Animation smoothness
- Layout stability
- Change highlighting
- Responsive design

## Implementation Plan

### Phase 1: Basic Timeline (3 days)
1. Create timeline component
2. Load commit history
3. Basic navigation
4. Simple transitions
5. Current state display

### Phase 2: Advanced Features (2 days)
1. Playback controls
2. Change highlighting
3. Diff panel
4. Filtering system
5. Performance optimization

### Phase 3: Polish (2 days)
1. Smooth animations
2. Layout caching
3. Keyboard shortcuts
4. Mobile support
5. Export timeline

## Future Enhancements
- Branch comparison
- Merge visualization
- Blame information overlay
- Predictive preloading
- Video export of evolution
- Collaborative replay

## Related Features
- [F020: Graph Visualization Engine](F020-graph-visualization-engine.md) - Core rendering
- [F023: Search and Filter UI](F023-search-and-filter-ui.md) - Filtering system
- [F001: Git Object Storage](F001-git-object-storage.md) - Historical data