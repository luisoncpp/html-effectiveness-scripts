---
name: yaml-components
description: Use when the user is writing, editing, or debugging YAML component blocks in hybrid Markdown files for the Rust UI Compiler. Covers all 12 available primitives (notice, card, data-grid, timeline, board-layout, code-panel, code-map, svg-canvas, flowchart, module-map, prompt-box, triage-board), frontmatter config, children nesting rules, and common compilation errors.
---

# YAML Components Skill

## Overview

The Rust UI Compiler consumes hybrid Markdown files that mix standard prose with fenced YAML component blocks. The compiler produces a single self-contained HTML file with inlined CSS and JS.

A component block looks like this inside a Markdown file:

```markdown
```yaml
type: notice
variant: warning
content: |
  <strong>Heads up:</strong> This is a callout.
```
```

## Frontmatter

Document-level settings go in a `---` delimited block at the very top of the file:

```yaml
---
title: My Document
layout: wide        # reading-column | wide | canvas
theme: clay-slate   # loads assets/tokens/<theme>.css
---
```

- `title`: Optional. Appears in `<title>` and can be referenced by layouts.
- `layout`: `reading-column` (default, 65ch max), `wide`, or `canvas`.
- `theme`: Optional. Activates a theme token CSS file.

## Available Primitives

### 1. Notice

Callout / alert box with left border accent.

```yaml
type: notice
variant: warning    # info | success | danger | warning | sticky-nav
icon: alert-triangle  # optional
content: |
  <strong>Breaking Change:</strong> The parser now expects multiple blocks.
```

- `variant`: CSS modifier class. Determines border/color accent.
- `icon`: Optional string. Rendered as text if present.
- `content`: HTML string rendered with `|safe`. Use inline tags for formatting.

### 2. Card

Atomic boxed container. Can contain its own content and nested children.

```yaml
type: card
title: Feature Card
elevation: 2          # 1 | 2 | 3
tags:
  - rust
  - urgent
content: |
  Main card content here.
children:
  - type: notice
    variant: info
    content: Nested callout inside the card.
```

- `title`: Optional header.
- `elevation`: Visual depth (box shadow). Defaults to `1`.
- `tags`: Optional list of strings rendered as small chips.
- `content`: HTML string rendered with `|safe` inside the card body.
- `children`: Optional array of nested component blocks.

### 3. DataGrid

Rich HTML table.

```yaml
type: data-grid
columns:
  - Feature
  - Status
  - Risk
rows:
  - ["AST Traversal", "Shipped", "Low"]
  - ["Drag & Drop", "WIP", "High"]
```

- `columns`: Array of header strings.
- `rows`: Array of arrays. Each inner array is a row. Cell values are rendered with `|safe`, so you can inline HTML badges if needed.

### 4. Timeline

Sequential milestones or incident steps.

```yaml
type: timeline
orientation: vertical   # default
steps:
  - timestamp: "2026-05-18 10:00"
    title: "Initial Outage"
    type: "critical"
  - timestamp: "2026-05-18 10:15"
    title: "Rolled back to v1.2"
    type: "recovery"
```

- `orientation`: Defaults to `vertical`.
- `steps`: Array of `TimelineStep`.
  - `timestamp`: String label (can be any text, not strictly a date).
  - `title`: Step heading.
  - `type`: Step modifier (`critical`, `recovery`, `info`, etc.). Maps to a CSS class.

### 5. BoardLayout

Flex/grid spatial container for organizing items.

```yaml
type: board-layout
variant: kanban       # kanban | grid | slides
columns:
  - title: "To Do"
    items:
      - "Task A"
      - "Task B"
  - title: "Done"
    items:
      - "Task C"
```

- `variant`: Determines layout mode.
  - `kanban`: Flex columns with cards.
  - `grid`: CSS grid layout.
  - `slides`: Horizontal scroll-snapping flex.
- `columns`: Array of `BoardColumn`.
  - `title`: Column header.
  - `items`: Array of strings rendered as simple cards inside the column.

> Note: Columns do NOT support nested component children directly. Only the top-level `children` array of a component is parsed as nested `Block`s. Use the component's top-level `children` field for nested components.

### 6. CodePanel

Tabbed code snippet display.

```yaml
type: code-panel
tabs:
  - name: "src/compiler.rs"
    language: rust
    diff: true
    content: |
      - let tree = parse_single();
      + let ast = parse_blocks();
  - name: "Cargo.toml"
    language: toml
    content: |
      [dependencies]
      pulldown-cmark = "0.9"
```

- `tabs`: Array of `CodeTab`.
  - `name`: Tab label.
  - `language`: Used as a CSS class (e.g. `language-rust`).
  - `diff`: Boolean. If `true`, the panel gets a `code-panel--diff` class.
  - `content`: Raw code text rendered inside `<pre><code>`.

### 7. SvgCanvas

Declarative SVG wrapper.

```yaml
type: svg-canvas
viewBox: "0 0 800 600"   # default if omitted
interactive: false       # default if omitted
elements:
  - type: rect
    x: 10
    y: 10
    width: 100
    height: 60
    class: "node-primary"
  - type: circle
    cx: 200
    cy: 200
    r: 50
    class: "node-secondary"
  - type: text
    x: 10
    y: 10
    text: "Hello SVG"
  - type: edge
    x: 110
    y: 40
    x2: 200
    y2: 200
    class: "edge"
```

- `viewBox`: SVG viewBox attribute. Defaults to `"0 0 800 600"`.
- `interactive`: Boolean flag (reserved for future interactivity).
- `elements`: Array of `SvgElement`.
  - `type`: One of `rect`, `circle`, `text`, `edge`.
  - Coordinates (`x`, `y`, `width`, `height`, `cx`, `cy`, `r`) are optional depending on element type.
  - `class`: Optional CSS class string.
  - `text`: Text content for `text` elements.
  - `marker`: Optional SVG marker id (without `#`) for `edge` elements. Emitted as `marker-end="url(#<marker>)"`.

#### `edge` element

The `edge` element renders an SVG `<line>`. The start point is `(x, y)` and the end point is `(x2, y2)` (absolute coordinates, not offsets). All four of `x`, `y`, `x2`, `y2` must be provided; any missing component drops the corresponding `x1`/`y1`/`x2`/`y2` attribute. The bundled `.edge` class in `assets/css/svg_canvas.css` provides a sensible default stroke. `width` and `height` are ignored on `edge` elements.

### 8. Flowchart

An interactive SVG-based flowchart layout with an associated detail sidebar.

```yaml
type: flowchart
title: "What happens when you git push"
description: "The deploy pipeline for acme/web."
viewBox: "0 0 620 400"
nodes:
  - id: push
    type: terminal         # terminal | rect | diamond
    x: 230
    y: 12
    width: 160
    height: 44
    label: "git push main"
    detail_idx: 0          # optional index into details
  - id: gate
    type: diamond
    x: 268
    y: 262
    width: 84
    height: 48
    label: "pass?"
    detail_idx: 3
edges:
  - from: push
    to: ci
    d: "M310,56 L310,92"
  - from: gate
    to: done
    edge_type: yes         # yes | no | normal
    label: pass
    d: "M310,310 L310,350"
details:
  - title: "git push main"
    meta: "trigger · 0s"
    body: "A push or merge to main fires the deploy workflow."
    code: "on:\n  push:\n    branches: [main]"
```

- `title`: Header title text.
- `description`: Optional subheader text.
- `viewBox`: Viewport configuration.
- `nodes`: Array of flowchart node coordinates, dimensions, types, labels, and details index.
- `edges`: Array of connection lines containing path definitions (`d`), type modifiers, and optional labels.
- `details`: Array of items shown in the detail sidebar when matching nodes are hovered or clicked.

### 9. ModuleMap

A visual dependency/module map representation using SVG blocks and connection edges.

```yaml
type: module-map
title: "Module dependencies"
viewBox: "0 0 600 300"
nodes:
  - id: parser
    label: "parser.rs"
    x: 50
    y: 50
    width: 120
    height: 50
    class: highlight       # highlight | optional other classes
  - id: renderer
    label: "renderer.rs"
    x: 250
    y: 50
    width: 120
    height: 50
edges:
  - from: parser
    to: renderer
    d: "M170,75 L250,75"
```

- `title`: Map section title.
- `viewBox`: SVG viewBox attribute.
- `nodes`: Array of modules including custom styling classes.
- `edges`: Array of directional paths mapping imports or dependencies.

### 10. CodeMap

A spatial "code flow" diagram: labeled group containers and syntax-highlighted
code cards laid out on a dotted canvas, with curved arrows connecting a
highlighted token in one card to another card (or to a token inside it).
Ideal for visualizing call chains across files (entry point → init → services).

```yaml
type: code-map
title: "Startup flow"     # optional heading above the canvas
width: 1240               # canvas width in px. Defaults to 1200.
height: 770               # canvas height in px. Required.
groups:
  - label: "Entry Point"
    variant: amber        # amber | green | blue | clay | plain (default)
    x: 16
    y: 10
    width: 350
    height: 245
cards:
  - id: main              # unique id, used by arrows
    x: 32
    y: 70
    width: 318
    height: 200           # optional; auto-sizes to content if omitted
    title: "src/main.ts"  # optional file-path header
    language: ts          # rust | ts | js | python | ... (default: generic)
    code: |
      main(): void {
        try {
          this.[[startup]]();
        } catch (error) {
          console.error(error.message);
        }
      }
  - id: startup
    x: 432
    y: 110
    width: 340
    language: ts
    code: |
      private async [[startup]](): Promise<void>
        const [services] = this.[[createServices]]();
arrows:
  - from: main.startup    # "cardId.anchorId" (a token) or just "cardId"
    to: startup.startup   # same syntax; plain "cardId" targets the card edge
```

- `width` / `height`: Pixel dimensions of the canvas. All `x`/`y` coordinates
  are absolute within it. The canvas scrolls horizontally if wider than the page.
- `groups`: Decorative labeled containers drawn behind the cards. `variant`
  picks the accent color of the border and label tab.
- `cards`: Code snippet boxes. Code is syntax-highlighted automatically
  (keywords, types, strings, numbers, function calls, comments) based on
  `language`.
- **Anchor tokens**: Inside `code`, wrap a token in `[[...]]` to render it as
  a blue highlighted chip and register it as an arrow endpoint with id
  `cardId.token`. Use `[[myId|display text]]` when the display text is not a
  valid id or appears more than once in the card. Lines containing an anchor
  get a highlighted background.
- `arrows`: Curved connectors drawn at load time by inlined JS. `from`/`to`
  accept `cardId.anchorId` (points at the token) or `cardId` (points at the
  card's nearest edge).

### 11. PromptBox (Legacy)

<!-- ```yaml -->
type: prompt-box
label: My Prompt
content: This is prompt content.
```

- `label`: Header text.
- `content`: Body text (pre-wrap, monospace).

### 12. TriageBoard (Legacy)

```yaml
type: triage-board
eyebrow: Acme / editor / triage
title: Cycle 14 triage
subtitle: Planning board
hintline: drag tickets between columns
```

## Nesting Rules

Only the top-level `children` key inside a fenced YAML block is parsed as nested components. Example:

```yaml
type: card
title: Parent
elevation: 2
children:
  - type: notice
    variant: info
    content: I am a nested notice.
  - type: card
    title: Child Card
    elevation: 1
    content: I am nested too.
```

The parser:
1. Strips the `children` array from the YAML mapping.
2. Deserializes the remaining mapping into the parent component.
3. Recursively parses each child in the `children` array.

**Important**: Nested `children` inside other fields (like `board-layout.columns[].children`) are NOT automatically parsed as component blocks. They remain as raw YAML data inside the parent struct.

## Common Errors

| Symptom | Cause |
|---------|-------|
| `Failed to deserialize YAML component` | Unknown `type` value or missing required field. |
| `children must be an array` | The `children` key is present but not a YAML sequence. |
| `Unsupported child block type` | A child in `children` lacks a `type` key. |
| Missing styles in output | Component CSS not registered in `assets.rs` `resolve_asset()`. |
| Render error comment in HTML | Template name mismatch or missing template registration in `renderer.rs`. |
| Code not being rendered and unexpected formatting change | Using `<` `>` characters in content without escaping them (use either `&lt;` `&gt;` or backquotes) |

The compiler produces a fully self-contained HTML file with zero external asset links.
