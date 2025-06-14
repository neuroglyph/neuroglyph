# F021: Interactive Graph Editing

**Status:** PLANNED  
**Priority:** Medium (Phase 1b)  
**Effort:** 1 week  
**Dependencies:** F020 (Graph Visualization), F019 (Web Server)  

## Summary
Enable users to create, modify, and delete links directly through the web interface using intuitive drag-and-drop and context menu interactions, with all changes properly committed to Git.

## User Stories

### 1. The Visual Link Creator
As a user who thinks visually, I want to create links by dragging from one node to another, so I can build connections without memorizing CLI commands.

### 2. The Bulk Organizer
As someone refactoring a project, I want to select multiple files and create links between them in batches, so I can quickly establish relationships.

### 3. The Precision Editor
As a detail-oriented user, I want to right-click on nodes and choose specific link types from a menu, so I can create exactly the relationships I intend.

### 4. The Mistake Corrector
As a human who makes errors, I want to easily undo link creation or delete wrong links visually, so I can fix mistakes without leaving the UI.

## Acceptance Criteria

### 1. Link Creation Methods
- [ ] Drag from source node to target node
- [ ] Right-click node → "Create Link" → Select target
- [ ] Select multiple nodes → "Link All" options
- [ ] Keyboard shortcuts (Ctrl+L for link mode)
- [ ] Touch gestures for tablet users

### 2. Link Type Selection
- [ ] Radial menu appears during drag
- [ ] Keyboard shortcuts for common types (I=Implements, R=References)
- [ ] Visual preview of link style before creation
- [ ] Recent link types shown first
- [ ] Custom link type input option

### 3. Visual Feedback
- [ ] Drag preview line follows cursor
- [ ] Valid drop targets highlighted
- [ ] Invalid targets grayed out
- [ ] Link type preview on hover
- [ ] Success animation on creation
- [ ] Error shake on failure

### 4. Bulk Operations
- [ ] Box select multiple nodes
- [ ] Ctrl+click for multi-select
- [ ] "Link all selected to..." option
- [ ] "Chain selected nodes" option
- [ ] Batch link type assignment

### 5. Link Deletion
- [ ] Click link to select, Delete key to remove
- [ ] Right-click link → "Delete"
- [ ] Hover over link shows X button
- [ ] Confirm dialog for multiple deletions
- [ ] Undo recent deletions

### 6. Git Integration
- [ ] Each operation creates proper Git commit
- [ ] Batch operations = single commit
- [ ] Meaningful commit messages
- [ ] Show commit status in UI
- [ ] Handle Git errors gracefully

## Technical Design

### Interaction State Machine
```javascript
const InteractionStates = {
  IDLE: 'idle',
  DRAGGING: 'dragging',
  LINK_TYPE_SELECTION: 'linkTypeSelection',
  CONFIRMING: 'confirming',
  PROCESSING: 'processing'
};

class LinkCreationManager {
  constructor(graphEngine, apiClient) {
    this.state = InteractionStates.IDLE;
    this.sourceNode = null;
    this.targetNode = null;
    this.previewLink = null;
  }

  startDrag(sourceNode, event) {
    this.state = InteractionStates.DRAGGING;
    this.sourceNode = sourceNode;
    this.previewLink = this.createPreviewLink(sourceNode, event);
  }

  updateDrag(event) {
    if (this.state !== InteractionStates.DRAGGING) return;
    
    const target = this.findNodeUnderCursor(event);
    this.updatePreviewLink(event, target);
    this.highlightValidTargets(this.sourceNode, target);
  }

  endDrag(event) {
    const target = this.findNodeUnderCursor(event);
    if (target && this.isValidLink(this.sourceNode, target)) {
      this.showLinkTypeMenu(this.sourceNode, target, event);
    } else {
      this.cancelDrag();
    }
  }
}
```

### Link Type Selection UI
```javascript
class LinkTypeRadialMenu {
  constructor() {
    this.linkTypes = [
      { type: 'IMPLEMENTS', icon: '🔨', angle: 0 },
      { type: 'REFERENCES', icon: '📚', angle: 90 },
      { type: 'INSPIRED_BY', icon: '💡', angle: 180 },
      { type: 'DEPENDS_ON', icon: '🔗', angle: 270 }
    ];
  }

  show(x, y, onSelect) {
    const menu = d3.select('body')
      .append('div')
      .attr('class', 'link-type-menu')
      .style('left', x + 'px')
      .style('top', y + 'px');

    const options = menu.selectAll('.option')
      .data(this.linkTypes)
      .enter()
      .append('div')
      .attr('class', 'option')
      .style('transform', d => `rotate(${d.angle}deg) translateY(-60px)`)
      .on('click', d => {
        onSelect(d.type);
        menu.remove();
      });

    // Animations and styling
    options.transition()
      .duration(200)
      .style('opacity', 1)
      .style('transform', d => `rotate(${d.angle}deg) translateY(-80px)`);
  }
}
```

### API Integration
```javascript
class GraphEditingAPI {
  async createLink(source, target, type) {
    try {
      const response = await fetch('/api/link', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          source: source.id,
          target: target.id,
          type: type
        })
      });

      if (!response.ok) {
        throw new Error(await response.text());
      }

      return await response.json();
    } catch (error) {
      this.showError('Failed to create link: ' + error.message);
      throw error;
    }
  }

  async deleteLink(linkId) {
    const response = await fetch(`/api/link/${linkId}`, {
      method: 'DELETE'
    });

    if (!response.ok) {
      throw new Error('Failed to delete link');
    }
  }
}
```

### Undo/Redo System
```javascript
class UndoManager {
  constructor(maxHistory = 50) {
    this.history = [];
    this.currentIndex = -1;
    this.maxHistory = maxHistory;
  }

  addAction(action) {
    // Remove any redo history
    this.history = this.history.slice(0, this.currentIndex + 1);
    
    // Add new action
    this.history.push(action);
    this.currentIndex++;
    
    // Limit history size
    if (this.history.length > this.maxHistory) {
      this.history.shift();
      this.currentIndex--;
    }
  }

  async undo() {
    if (this.currentIndex < 0) return;
    
    const action = this.history[this.currentIndex];
    await action.undo();
    this.currentIndex--;
  }

  async redo() {
    if (this.currentIndex >= this.history.length - 1) return;
    
    this.currentIndex++;
    const action = this.history[this.currentIndex];
    await action.redo();
  }
}
```

### Validation Rules
```javascript
function validateLink(source, target, type) {
  // Cannot link to self
  if (source.id === target.id) {
    return { valid: false, reason: "Cannot link file to itself" };
  }

  // Check for duplicate links
  const existingLink = findLink(source.id, target.id, type);
  if (existingLink) {
    return { valid: false, reason: "Link already exists" };
  }

  // Type-specific validation
  if (type === 'IMPLEMENTS' && !isCodeFile(source)) {
    return { valid: false, reason: "Only code files can implement" };
  }

  // Circular dependency check for DEPENDS_ON
  if (type === 'DEPENDS_ON' && wouldCreateCycle(source, target)) {
    return { valid: false, reason: "Would create circular dependency" };
  }

  return { valid: true };
}
```

## User Interface Components

### Drag Preview
```css
.link-preview {
  stroke: #4A90E2;
  stroke-width: 2;
  stroke-dasharray: 5, 5;
  opacity: 0.6;
  pointer-events: none;
}

.valid-drop-target {
  stroke: #7ED321;
  stroke-width: 3;
  filter: drop-shadow(0 0 5px #7ED321);
}

.invalid-drop-target {
  opacity: 0.3;
  cursor: not-allowed;
}
```

### Context Menus
```javascript
const nodeContextMenu = [
  {
    label: 'Create Link From Here',
    icon: '🔗',
    action: (node) => startLinkCreation(node)
  },
  {
    label: 'Create Link To Here',
    icon: '🎯',
    action: (node) => startReverseLinkCreation(node)
  },
  {
    label: 'Delete All Links',
    icon: '🗑️',
    action: (node) => deleteAllLinks(node),
    confirm: true
  }
];
```

## Error Handling

### User-Friendly Messages
```javascript
const errorMessages = {
  'git_locked': "Another operation is in progress. Please try again.",
  'file_not_found': "One of the files no longer exists.",
  'permission_denied': "Cannot modify repository. Check permissions.",
  'network_error': "Connection lost. Please check your network."
};

function showError(error) {
  const message = errorMessages[error.code] || error.message;
  
  const toast = document.createElement('div');
  toast.className = 'error-toast';
  toast.textContent = message;
  document.body.appendChild(toast);
  
  setTimeout(() => toast.remove(), 5000);
}
```

## Testing Strategy

### Interaction Tests
- Drag and drop across browsers
- Touch gesture handling
- Keyboard navigation
- Multi-select operations
- Undo/redo functionality

### Integration Tests
- Git commit creation
- Concurrent editing
- Error recovery
- Large batch operations
- Network interruptions

### Usability Tests
- Time to create first link
- Error rate in link creation
- Discoverability of features
- Mobile/tablet usability

## Implementation Plan

### Phase 1: Basic Editing (3 days)
1. Implement drag-and-drop
2. Create link type menu
3. Add API integration
4. Basic error handling
5. Visual feedback

### Phase 2: Advanced Features (2 days)
1. Multi-select operations
2. Undo/redo system
3. Context menus
4. Keyboard shortcuts
5. Validation rules

### Phase 3: Polish (2 days)
1. Smooth animations
2. Touch support
3. Accessibility
4. Performance optimization
5. Comprehensive testing

## Accessibility
- Keyboard-only link creation
- Screen reader announcements
- High contrast mode support
- Focus management
- Alternative input methods

## Future Enhancements
- Link strength adjustment
- Link metadata editing
- Batch link operations via CSV
- Link templates/patterns
- AI-suggested links
- Collaborative editing

## Related Features
- [F020: Graph Visualization Engine](F020-graph-visualization-engine.md) - Rendering foundation
- [F019: Local Web Server](F019-local-web-server.md) - API backend
- [F022: Time Travel Interface](F022-time-travel-interface.md) - History viewing