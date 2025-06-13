# F024: Export and Sharing

**Status:** PLANNED  
**Priority:** Low (Phase 2)  
**Effort:** 3 days  
**Dependencies:** F020 (Graph Visualization)  

## Summary
Enable users to export their knowledge graphs in various formats and share visualizations with others through images, interactive widgets, or embeddable components.

## User Stories

### 1. The Documentation Creator
As a project maintainer, I want to export my knowledge graph as an image for our README, so new contributors can understand our project structure at a glance.

### 2. The Presenter
As a developer giving a talk, I want to export high-resolution images of specific graph views, so I can include them in my presentation slides.

### 3. The Analyst
As a data scientist, I want to export the graph data as JSON or CSV, so I can perform custom analysis in other tools.

### 4. The Collaborator
As a team member, I want to share a specific view of the graph with a colleague, so they can see exactly what I'm referring to without needing GitMind installed.

## Acceptance Criteria

### 1. Image Export
- [ ] Export as PNG with transparent background
- [ ] Export as SVG for scalability
- [ ] Custom resolution settings
- [ ] Include/exclude UI elements
- [ ] Preserve current zoom/pan state
- [ ] Add watermark/attribution option

### 2. Data Export
- [ ] Export as JSON (full graph structure)
- [ ] Export as CSV (node and edge lists)
- [ ] Export as GraphML
- [ ] Export as DOT (Graphviz)
- [ ] Export as Cypher queries
- [ ] Filtered export (current view only)

### 3. Sharing Features
- [ ] Copy image to clipboard
- [ ] Generate shareable link (local server)
- [ ] Create embeddable iframe code
- [ ] Export interactive HTML snapshot
- [ ] Social media optimized images
- [ ] QR code for mobile viewing

### 4. Customization Options
- [ ] Title and description overlay
- [ ] Custom color schemes
- [ ] Hide sensitive information
- [ ] Add annotations/callouts
- [ ] Timestamp and version info
- [ ] Company/project branding

### 5. Batch Operations
- [ ] Export multiple views at once
- [ ] Automated export via CLI
- [ ] Scheduled exports
- [ ] Export presets/templates
- [ ] Diff exports (showing changes)
- [ ] Animation sequence export

## Technical Design

### Export Engine
```javascript
class ExportEngine {
  constructor(graphEngine) {
    this.graphEngine = graphEngine;
    this.exporters = {
      png: new PNGExporter(),
      svg: new SVGExporter(),
      json: new JSONExporter(),
      csv: new CSVExporter(),
      graphml: new GraphMLExporter(),
      dot: new DOTExporter()
    };
  }

  async export(format, options = {}) {
    const exporter = this.exporters[format];
    if (!exporter) {
      throw new Error(`Unsupported format: ${format}`);
    }

    const graphData = await this.prepareData(options);
    return exporter.export(graphData, options);
  }

  async prepareData(options) {
    const { includeHidden, filterActive, bounds } = options;
    
    let nodes = this.graphEngine.getNodes();
    let links = this.graphEngine.getLinks();
    
    if (!includeHidden) {
      nodes = nodes.filter(n => n.visible);
    }
    
    if (filterActive) {
      const activeFilter = this.graphEngine.getActiveFilter();
      nodes = activeFilter.apply(nodes);
      links = this.filterLinks(links, nodes);
    }
    
    if (bounds) {
      nodes = this.nodesInBounds(nodes, bounds);
      links = this.filterLinks(links, nodes);
    }
    
    return { nodes, links, metadata: this.getMetadata() };
  }
}
```

### Image Exporters

#### SVG Exporter
```javascript
class SVGExporter {
  export(graphData, options) {
    const svg = this.graphEngine.getSVGElement();
    const clone = svg.cloneNode(true);
    
    // Clean up interactive elements
    this.removeInteractiveElements(clone);
    
    // Add styling
    this.inlineStyles(clone);
    
    // Add metadata
    if (options.includeMetadata) {
      this.addMetadata(clone, graphData.metadata);
    }
    
    // Add watermark
    if (options.watermark) {
      this.addWatermark(clone, options.watermark);
    }
    
    // Convert to string
    const serializer = new XMLSerializer();
    const svgString = serializer.serializeToString(clone);
    
    return {
      data: svgString,
      mimeType: 'image/svg+xml',
      extension: 'svg'
    };
  }

  inlineStyles(svg) {
    // Convert CSS classes to inline styles for portability
    const styles = getComputedStyle(svg);
    const elements = svg.querySelectorAll('*');
    
    elements.forEach(el => {
      const computedStyle = getComputedStyle(el);
      const important = ['fill', 'stroke', 'font-family', 'font-size'];
      
      important.forEach(prop => {
        if (computedStyle[prop]) {
          el.style[prop] = computedStyle[prop];
        }
      });
    });
  }
}
```

#### PNG Exporter
```javascript
class PNGExporter {
  async export(graphData, options) {
    const { width = 1920, height = 1080, scale = 2 } = options;
    
    // Create off-screen canvas
    const canvas = document.createElement('canvas');
    canvas.width = width * scale;
    canvas.height = height * scale;
    
    const ctx = canvas.getContext('2d');
    ctx.scale(scale, scale);
    
    // Render SVG to canvas
    const svgData = new SVGExporter().export(graphData, options);
    const img = new Image();
    const svgBlob = new Blob([svgData.data], { type: 'image/svg+xml' });
    const url = URL.createObjectURL(svgBlob);
    
    return new Promise((resolve) => {
      img.onload = () => {
        // White background option
        if (!options.transparent) {
          ctx.fillStyle = 'white';
          ctx.fillRect(0, 0, width, height);
        }
        
        ctx.drawImage(img, 0, 0, width, height);
        URL.revokeObjectURL(url);
        
        canvas.toBlob(blob => {
          resolve({
            data: blob,
            mimeType: 'image/png',
            extension: 'png'
          });
        }, 'image/png');
      };
      img.src = url;
    });
  }
}
```

### Data Exporters

#### JSON Exporter
```javascript
class JSONExporter {
  export(graphData, options) {
    const output = {
      version: '1.0',
      generator: 'GitMind',
      created: new Date().toISOString(),
      graph: {
        nodes: graphData.nodes.map(n => ({
          id: n.id,
          label: n.label,
          type: n.type,
          metadata: options.includeMetadata ? n.metadata : undefined,
          position: options.includeLayout ? { x: n.x, y: n.y } : undefined
        })),
        edges: graphData.links.map(l => ({
          source: l.source.id,
          target: l.target.id,
          type: l.type,
          weight: l.weight
        }))
      }
    };

    return {
      data: JSON.stringify(output, null, 2),
      mimeType: 'application/json',
      extension: 'json'
    };
  }
}
```

#### GraphML Exporter
```javascript
class GraphMLExporter {
  export(graphData, options) {
    const doc = document.implementation.createDocument(null, 'graphml');
    const graphml = doc.documentElement;
    
    // Add schema
    graphml.setAttribute('xmlns', 'http://graphml.graphdrawing.org/xmlns');
    
    // Define attributes
    this.addKeyDefinitions(doc, graphml);
    
    // Create graph element
    const graph = doc.createElement('graph');
    graph.setAttribute('id', 'G');
    graph.setAttribute('edgedefault', 'directed');
    
    // Add nodes
    graphData.nodes.forEach(node => {
      const n = doc.createElement('node');
      n.setAttribute('id', node.id);
      
      this.addDataElements(doc, n, node);
      graph.appendChild(n);
    });
    
    // Add edges
    graphData.links.forEach((link, i) => {
      const e = doc.createElement('edge');
      e.setAttribute('id', `e${i}`);
      e.setAttribute('source', link.source.id);
      e.setAttribute('target', link.target.id);
      
      this.addDataElements(doc, e, link);
      graph.appendChild(e);
    });
    
    graphml.appendChild(graph);
    
    const serializer = new XMLSerializer();
    return {
      data: serializer.serializeToString(doc),
      mimeType: 'application/xml',
      extension: 'graphml'
    };
  }
}
```

### Sharing Components

#### Embed Generator
```javascript
class EmbedGenerator {
  generateIframeCode(viewState, options) {
    const { width = 800, height = 600, readonly = true } = options;
    const params = new URLSearchParams({
      view: JSON.stringify(viewState),
      readonly: readonly.toString(),
      zoom: viewState.zoom,
      centerX: viewState.centerX,
      centerY: viewState.centerY
    });
    
    return `<iframe 
  src="http://localhost:7432/embed?${params}" 
  width="${width}" 
  height="${height}"
  frameborder="0"
  title="GitMind Knowledge Graph">
</iframe>`;
  }

  generateStaticHTML(graphData, options) {
    // Create self-contained HTML with inlined JS/CSS
    const html = `<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>${options.title || 'Knowledge Graph'}</title>
  <style>${this.getInlineCSS()}</style>
</head>
<body>
  <div id="graph"></div>
  <script src="https://d3js.org/d3.v7.min.js"></script>
  <script>
    const data = ${JSON.stringify(graphData)};
    ${this.getInlineJS()}
  </script>
</body>
</html>`;
    
    return {
      data: html,
      mimeType: 'text/html',
      extension: 'html'
    };
  }
}
```

#### Clipboard Integration
```javascript
class ClipboardExporter {
  async copyImage(format = 'png') {
    const exporter = new PNGExporter();
    const result = await exporter.export(this.graphData, {
      width: 1200,
      height: 800,
      transparent: true
    });
    
    try {
      // Modern clipboard API
      await navigator.clipboard.write([
        new ClipboardItem({
          [result.mimeType]: result.data
        })
      ]);
      
      this.showNotification('Image copied to clipboard!');
    } catch (err) {
      // Fallback for older browsers
      this.fallbackCopy(result);
    }
  }

  async copyData(format = 'json') {
    const exporter = this.exporters[format];
    const result = exporter.export(this.graphData);
    
    await navigator.clipboard.writeText(result.data);
    this.showNotification(`${format.toUpperCase()} copied to clipboard!`);
  }
}
```

## UI Components

### Export Dialog
```html
<div class="export-dialog">
  <h2>Export Graph</h2>
  
  <div class="export-tabs">
    <button class="tab active" data-tab="image">Image</button>
    <button class="tab" data-tab="data">Data</button>
    <button class="tab" data-tab="share">Share</button>
  </div>
  
  <div class="tab-content" id="image-tab">
    <div class="format-selection">
      <label>
        <input type="radio" name="image-format" value="png" checked>
        PNG (High Quality)
      </label>
      <label>
        <input type="radio" name="image-format" value="svg">
        SVG (Scalable)
      </label>
    </div>
    
    <div class="options">
      <label>
        <input type="checkbox" id="transparent-bg">
        Transparent Background
      </label>
      <label>
        <input type="checkbox" id="include-title">
        Include Title
      </label>
      <label>
        Resolution:
        <select id="resolution">
          <option value="1920x1080">1920×1080 (Full HD)</option>
          <option value="3840x2160">3840×2160 (4K)</option>
          <option value="custom">Custom...</option>
        </select>
      </label>
    </div>
    
    <button class="export-button">Export Image</button>
  </div>
  
  <div class="tab-content" id="data-tab" hidden>
    <!-- Data export options -->
  </div>
  
  <div class="tab-content" id="share-tab" hidden>
    <!-- Sharing options -->
  </div>
</div>
```

## Testing Strategy

### Export Tests
- Output format validation
- File size optimization
- Cross-platform compatibility
- Character encoding
- Large graph handling

### Visual Tests
- Image quality verification
- SVG rendering accuracy
- Layout preservation
- Color accuracy
- Font rendering

### Integration Tests
- Clipboard operations
- File download triggers
- Progress indicators
- Error handling
- Memory management

## Implementation Plan

### Phase 1: Basic Export (1 day)
1. Implement PNG/SVG export
2. Basic JSON/CSV export
3. Download functionality
4. Simple UI dialog

### Phase 2: Advanced Features (1 day)
1. GraphML/DOT exporters
2. Clipboard integration
3. Custom resolution options
4. Filtered exports

### Phase 3: Sharing Features (1 day)
1. Embed code generator
2. Static HTML export
3. Share URLs
4. Social media optimization
5. Batch export support

## Performance Considerations
- Stream large exports to avoid memory issues
- Use Web Workers for heavy processing
- Progressive rendering for previews
- Optimize SVG output size
- Cache export templates

## Future Enhancements
- Video export of graph evolution
- 3D model export (GLTF)
- PowerPoint/Keynote integration
- Cloud storage integration
- Collaborative sharing
- Export automation API

## Related Features
- [F020: Graph Visualization Engine](F020-graph-visualization-engine.md) - Source visualization
- [F022: Time Travel Interface](F022-time-travel-interface.md) - Animation export
- [F023: Search and Filter UI](F023-search-and-filter-ui.md) - Filtered exports