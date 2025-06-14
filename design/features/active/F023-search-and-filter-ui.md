# F023: Search and Filter UI

**Status:** PLANNED  
**Priority:** Medium (Phase 1b)  
**Effort:** 3 days  
**Dependencies:** F020 (Graph Visualization)  

## Summary
Powerful search and filtering capabilities that allow users to quickly find nodes, explore patterns, and focus on relevant subgraphs within their knowledge graph.

## User Stories

### 1. The Knowledge Seeker
As a developer looking for specific information, I want to search for files by name or content references, so I can quickly locate what I need in a large graph.

### 2. The Pattern Explorer
As a project architect, I want to filter the graph by link types and see only "DEPENDS_ON" relationships, so I can analyze our dependency structure.

### 3. The Time Investigator
As someone debugging an issue, I want to filter nodes by modification date, so I can see what changed recently.

### 4. The Scope Focuser
As a team member working on a feature, I want to filter by directory path, so I can focus only on my area of the codebase.

## Acceptance Criteria

### 1. Search Functionality
- [ ] Real-time search as you type
- [ ] Search by filename (fuzzy matching)
- [ ] Search by path pattern
- [ ] Search within link descriptions
- [ ] Search history/recent searches
- [ ] Clear search button

### 2. Filter Options
- [ ] Filter by file type/extension
- [ ] Filter by link type
- [ ] Filter by node degree (connectivity)
- [ ] Filter by date range
- [ ] Filter by file size
- [ ] Filter by directory

### 3. Visual Feedback
- [ ] Matching nodes highlighted
- [ ] Non-matching nodes faded
- [ ] Match count displayed
- [ ] Filter chips showing active filters
- [ ] Smooth transitions when filtering
- [ ] Loading states for async operations

### 4. Filter Combinations
- [ ] AND/OR logic for multiple filters
- [ ] Save filter combinations
- [ ] Quick filter presets
- [ ] Invert filter selections
- [ ] Clear all filters option
- [ ] Filter syntax for power users

### 5. Performance
- [ ] Instant filtering for <1000 nodes
- [ ] Sub-second for <10000 nodes
- [ ] Progressive filtering for larger graphs
- [ ] Debounced search input
- [ ] Efficient index structures
- [ ] Background indexing

### 6. Persistence
- [ ] Remember last search
- [ ] Save custom filters
- [ ] Share filter URLs
- [ ] Export filtered results
- [ ] Filter history
- [ ] Cross-session persistence

## Technical Design

### Search Engine
```javascript
class GraphSearchEngine {
  constructor(nodes, links) {
    this.fuse = new Fuse(nodes, {
      keys: ['id', 'path', 'content'],
      threshold: 0.3,
      includeScore: true
    });
    
    this.filters = new FilterManager();
    this.index = this.buildIndex(nodes, links);
  }

  search(query) {
    // Parse query for special syntax
    const parsed = this.parseQuery(query);
    
    if (parsed.isAdvanced) {
      return this.advancedSearch(parsed);
    }
    
    // Fuzzy search
    const results = this.fuse.search(query);
    return this.rankResults(results);
  }

  parseQuery(query) {
    // Support query syntax like:
    // type:doc path:src/ modified:>2025-06-01
    const filters = {};
    const patterns = {
      type: /type:(\S+)/g,
      path: /path:(\S+)/g,
      modified: /modified:([><=])(\S+)/g,
      size: /size:([><=])(\d+)/g
    };
    
    let cleanQuery = query;
    for (const [key, pattern] of Object.entries(patterns)) {
      const match = pattern.exec(query);
      if (match) {
        filters[key] = match[1];
        cleanQuery = cleanQuery.replace(match[0], '');
      }
    }
    
    return {
      query: cleanQuery.trim(),
      filters,
      isAdvanced: Object.keys(filters).length > 0
    };
  }
}
```

### Filter System
```javascript
class FilterManager {
  constructor() {
    this.activeFilters = new Map();
    this.presets = this.loadPresets();
  }

  addFilter(type, config) {
    const filter = this.createFilter(type, config);
    this.activeFilters.set(filter.id, filter);
    this.notifyChange();
  }

  createFilter(type, config) {
    switch(type) {
      case 'fileType':
        return new FileTypeFilter(config.extensions);
      case 'linkType':
        return new LinkTypeFilter(config.types);
      case 'dateRange':
        return new DateRangeFilter(config.start, config.end);
      case 'connectivity':
        return new ConnectivityFilter(config.min, config.max);
      case 'path':
        return new PathFilter(config.pattern);
      default:
        throw new Error(`Unknown filter type: ${type}`);
    }
  }

  applyFilters(nodes) {
    let filtered = nodes;
    
    for (const filter of this.activeFilters.values()) {
      filtered = filter.apply(filtered);
    }
    
    return filtered;
  }
}
```

### Filter Types
```javascript
class FileTypeFilter {
  constructor(extensions) {
    this.extensions = new Set(extensions);
  }

  apply(nodes) {
    return nodes.filter(node => {
      const ext = node.path.split('.').pop();
      return this.extensions.has(ext);
    });
  }

  toChip() {
    return {
      label: `Type: ${Array.from(this.extensions).join(', ')}`,
      removable: true
    };
  }
}

class ConnectivityFilter {
  constructor(min, max = Infinity) {
    this.min = min;
    this.max = max;
  }

  apply(nodes, links) {
    const degrees = this.calculateDegrees(nodes, links);
    return nodes.filter(node => {
      const degree = degrees.get(node.id) || 0;
      return degree >= this.min && degree <= this.max;
    });
  }
}
```

### UI Components

#### Search Bar
```javascript
class SearchBar {
  constructor(container, onSearch) {
    this.container = container;
    this.onSearch = debounce(onSearch, 200);
    this.history = new SearchHistory();
    
    this.render();
  }

  render() {
    this.container.innerHTML = `
      <div class="search-container">
        <input 
          type="text" 
          class="search-input"
          placeholder="Search nodes... (try 'type:md path:docs/')"
          autocomplete="off"
        >
        <button class="search-clear" aria-label="Clear search">×</button>
        <div class="search-suggestions"></div>
      </div>
    `;
    
    this.bindEvents();
  }

  bindEvents() {
    const input = this.container.querySelector('.search-input');
    
    input.addEventListener('input', (e) => {
      this.onSearch(e.target.value);
      this.updateSuggestions(e.target.value);
    });
    
    input.addEventListener('focus', () => {
      this.showHistory();
    });
  }
}
```

#### Filter Panel
```javascript
class FilterPanel {
  render() {
    return `
      <div class="filter-panel">
        <h3>Filters</h3>
        
        <div class="filter-section">
          <label>File Type</label>
          <div class="checkbox-group">
            <label><input type="checkbox" value="md"> Markdown</label>
            <label><input type="checkbox" value="js"> JavaScript</label>
            <label><input type="checkbox" value="rs"> Rust</label>
          </div>
        </div>
        
        <div class="filter-section">
          <label>Link Type</label>
          <select multiple class="link-type-select">
            <option value="IMPLEMENTS">Implements</option>
            <option value="REFERENCES">References</option>
            <option value="INSPIRED_BY">Inspired By</option>
            <option value="DEPENDS_ON">Depends On</option>
          </select>
        </div>
        
        <div class="filter-section">
          <label>Connectivity</label>
          <input type="range" min="0" max="20" class="connectivity-slider">
          <span class="connectivity-value">5+ connections</span>
        </div>
        
        <div class="filter-section">
          <label>Modified Date</label>
          <input type="date" class="date-from">
          <input type="date" class="date-to">
        </div>
        
        <div class="filter-actions">
          <button class="apply-filters">Apply</button>
          <button class="clear-filters">Clear All</button>
          <button class="save-preset">Save Preset</button>
        </div>
      </div>
    `;
  }
}
```

### Visual Feedback
```javascript
class FilterVisualizer {
  applyVisualFilters(nodes, matchingIds) {
    const matchSet = new Set(matchingIds);
    
    // Fade non-matching nodes
    nodes
      .classed('filtered-out', d => !matchSet.has(d.id))
      .transition()
      .duration(300)
      .style('opacity', d => matchSet.has(d.id) ? 1 : 0.2);
    
    // Update match counter
    this.updateMatchCount(matchingIds.length, nodes.data().length);
    
    // Highlight matching nodes
    nodes
      .filter(d => matchSet.has(d.id))
      .classed('search-match', true)
      .transition()
      .duration(200)
      .attr('r', d => d.radius * 1.2)
      .transition()
      .duration(200)
      .attr('r', d => d.radius);
  }

  showFilterChips(filters) {
    const chips = d3.select('.filter-chips')
      .selectAll('.chip')
      .data(filters);
    
    chips.enter()
      .append('div')
      .attr('class', 'chip')
      .html(d => `
        ${d.label}
        <span class="chip-remove" data-id="${d.id}">×</span>
      `);
  }
}
```

## Performance Optimizations

### Indexing Strategy
```javascript
class GraphIndex {
  constructor(nodes, links) {
    this.buildIndices(nodes, links);
  }

  buildIndices(nodes, links) {
    // Path trie for efficient prefix matching
    this.pathTrie = new Trie();
    nodes.forEach(node => this.pathTrie.insert(node.path));
    
    // Inverted index for content
    this.contentIndex = new InvertedIndex();
    nodes.forEach(node => {
      if (node.content) {
        this.contentIndex.addDocument(node.id, node.content);
      }
    });
    
    // Pre-computed degree map
    this.degreeMap = this.computeDegrees(nodes, links);
    
    // Date index for range queries
    this.dateIndex = new BTree();
    nodes.forEach(node => {
      this.dateIndex.insert(node.modified, node.id);
    });
  }
}
```

## Testing Strategy

### Search Tests
- Fuzzy matching accuracy
- Special character handling
- Performance with large datasets
- Query syntax parsing
- Unicode support

### Filter Tests
- Filter combination logic
- Edge cases (empty results)
- Performance benchmarks
- State persistence
- URL serialization

### UI Tests
- Responsive design
- Keyboard navigation
- Screen reader support
- Touch interactions
- Cross-browser compatibility

## Implementation Plan

### Phase 1: Basic Search (1 day)
1. Implement search bar UI
2. Add fuzzy search with Fuse.js
3. Visual highlighting of results
4. Clear and history functionality

### Phase 2: Filtering System (1 day)
1. Create filter panel UI
2. Implement filter types
3. Filter combination logic
4. Visual feedback system

### Phase 3: Advanced Features (1 day)
1. Query syntax parser
2. Filter presets
3. Performance optimization
4. Persistence layer
5. Polish and testing

## Related Features
- [F020: Graph Visualization Engine](F020-graph-visualization-engine.md) - Visual updates
- [F022: Time Travel Interface](F022-time-travel-interface.md) - Date filtering
- [F024: Export and Sharing](F024-export-and-sharing.md) - Export filtered results