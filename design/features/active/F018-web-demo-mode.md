# F018: Web Demo Mode

**Status:** PLANNED  
**Priority:** High (Marketing Critical)  
**Effort:** 1 week  
**Dependencies:** None (static site)  

## Summary
A zero-installation web demo that lets potential users experience GitMind's value in 30 seconds through an interactive visualization of a pre-built knowledge graph.

## User Stories

### 1. The Skeptical Developer
As a developer browsing Hacker News, I want to quickly understand what "Git-native knowledge graphs" means without installing anything, so I can decide if it's worth my time.

### 2. The Visual Learner  
As someone who thinks visually, I want to see an interactive graph before reading documentation, so I can immediately grasp the concept.

### 3. The Tool Evaluator
As a team lead evaluating tools, I want to experience the interface and capabilities firsthand, so I can assess if it fits our workflow.

### 4. The Sharing Enthusiast
As an excited early adopter, I want to share a link that immediately shows what GitMind does, so I can evangelize effectively.

## Acceptance Criteria

### 1. Zero Friction Experience
- [ ] Loads in < 2 seconds on average connection
- [ ] No login, signup, or cookies required
- [ ] Works on mobile devices (responsive design)
- [ ] Degrades gracefully without JavaScript
- [ ] Clear "Try Demo" button on homepage

### 2. Interactive Demonstration
- [ ] Pre-loaded with compelling example repository
- [ ] At least 30-50 nodes showing real relationships
- [ ] Hover to highlight connections
- [ ] Click to focus on node and its network
- [ ] Drag to reorganize for clarity
- [ ] Pinch/scroll to zoom

### 3. Guided Discovery
- [ ] 30-second animated intro (skippable)
- [ ] Tooltips for first-time interactions
- [ ] "Reset Demo" button to start fresh
- [ ] Example queries to try
- [ ] Clear value propositions displayed

### 4. Call to Action
- [ ] Prominent "Install GitMind" button
- [ ] Platform-specific install commands
- [ ] "Learn More" links to documentation
- [ ] "View on GitHub" for source
- [ ] Social sharing buttons

### 5. Demo Content Quality
- [ ] Realistic software project structure
- [ ] Mix of documentation, code, and notes
- [ ] Various link types demonstrated
- [ ] Time-based relationships shown
- [ ] Clear narrative in the connections

## Technical Design

### Architecture
```
[Static HTML] -> [Demo Data JSON] -> [D3.js Viz] -> [User Interaction]
      |                                    |
      v                                    v
[Install CTA] <----------------------- [State Management]
```

### Core Components

#### 1. Static Site Structure
```
demos/web-demo/
├── index.html          # Entry point
├── demo.js            # Main application
├── graph.js           # D3.js visualization
├── data/
│   └── sample.json    # Pre-computed graph data
├── styles/
│   └── demo.css       # Responsive styles
└── assets/
    └── images/        # Screenshots, logos
```

#### 2. Demo Data Format
```json
{
  "nodes": [
    {
      "id": "README.md",
      "type": "documentation",
      "size": 1024,
      "lastModified": "2025-06-10T10:00:00Z",
      "preview": "# Project Overview\n\nThis demonstrates..."
    }
  ],
  "links": [
    {
      "source": "README.md",
      "target": "ARCHITECTURE.md",
      "type": "REFERENCES",
      "timestamp": "2025-06-10T10:30:00Z"
    }
  ],
  "metadata": {
    "projectName": "Example Knowledge Graph",
    "nodeCount": 42,
    "linkCount": 67,
    "dateRange": ["2025-01-01", "2025-06-10"]
  }
}
```

#### 3. Visualization Features
- Force-directed layout with customizable physics
- Node sizing based on connectivity
- Edge styling by link type
- Smooth animations for all transitions
- WebGL fallback for large graphs

### Performance Requirements
- Initial load < 2 seconds
- Smooth interaction with 100+ nodes
- 60 FPS during animations
- Total bundle size < 500KB
- Works on 2018+ devices

## Implementation Plan

### Phase 1: Core Demo (3 days)
1. Create static site structure
2. Design compelling demo dataset
3. Implement basic D3.js visualization
4. Add core interactions (hover, click, drag)
5. Style for desktop browsers

### Phase 2: Polish (2 days)  
1. Add intro animation
2. Implement guided tour
3. Create responsive mobile layout
4. Add keyboard navigation
5. Optimize performance

### Phase 3: Marketing (2 days)
1. Write compelling copy
2. Add install instructions
3. Create social sharing cards
4. Add analytics (privacy-respecting)
5. Deploy to CDN

## Success Metrics
- 50% of visitors interact with graph
- 10% click through to installation
- Average session > 2 minutes
- Positive feedback on HN/Reddit
- Reduced "what is this?" questions

## Demo Script Example

### Opening (5 seconds)
*Graph fades in with title: "Your Knowledge, Connected"*

### Problem Setup (10 seconds)
*Zoom into messy folder structure*
"Every project has hidden connections..."

### Magic Moment (10 seconds)
*Connections animate into existence*
"GitMind reveals them"

### Interaction (15 seconds)
*User hovers, clicks, explores*
"Explore your project's knowledge graph"

### Call to Action (5 seconds)
*Install button pulses*
"Ready to map your own knowledge?"

## Content Strategy

### Example Repository: "OpenAPI Documentation System"
A realistic project showing:
- API specifications linking to implementations
- Test files referencing specs
- Documentation cross-references  
- Meeting notes inspiring features
- Architecture decisions affecting code

This creates a rich, realistic graph that developers will recognize from their own projects.

## Risk Mitigation
- **Static site**: No backend to crash or scale
- **CDN delivery**: Fast global performance
- **Fallback mode**: PNG screenshot if JS fails
- **Open source**: Demo code helps adoption
- **A/B testing**: Try different datasets

## Related Documents
- [ADR-003: Web Visualization Strategy](../decisions/ADR-003-web-visualization-strategy.md) - Architecture decision
- [Web Visualization Revival Proposal](../proposals/web-visualization-revival.md) - Original proposal
- [F019: Local Web Server](F019-local-web-server.md) - For installed users
- [F020: Graph Visualization Engine](F020-graph-visualization-engine.md) - Shared components