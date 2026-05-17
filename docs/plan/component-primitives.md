# Component Primitives Specification

By deconstructing the 20 target output files, we can synthesize the required layouts into **7 core UI primitives**. These building blocks are highly parameterized and can be composed to recreate everything from a PR Review to an Animation Sandbox.

---

## 1. `CodePanel`

**Purpose:** Displays code snippets with advanced formatting, diffing, and tabbed interfaces.
**Used in:** PR Writeups, Feature Explainers, Implementation Plans.
**Asset Dependencies:** `css/code-panel.css`, `js/tabs.js`, `syntect` (server-side).

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
    content: ...

```

## 2. `SvgCanvas`

**Purpose:** A declarative wrapper for vector graphics. Can be used for flowchart nodes, architecture diagrams, or interactive math/physics sandboxes.
**Used in:** Flowcharts, Concept Explainers, Module Maps.
**Asset Dependencies:** `css/svg-canvas.css`, (optional) `js/canvas-interactions.js`.

```yaml
type: svg-canvas
viewBox: "0 0 800 600"
interactive: true
elements:
  - type: rect
    x: 10
    y: 10
    class: "node-primary"
  - type: edge
    from: "node_1"
    to: "node_2"
    marker: "arrow"

```

## 3. `DataGrid` (Rich Table)

**Purpose:** A tabular layout that supports rendering sub-components (badges, risk dots, icons) inside its cells, rather than just plain text.
**Used in:** Incident Reports, Status Reports, Variant Matrices.
**Asset Dependencies:** `css/data-grid.css`.

```yaml
type: data-grid
columns: ["Feature", "Status", "Risk"]
rows:
  - ["AST Traversal", { type: badge, label: "Shipped", color: "green" }, { type: dot, color: "low" }]
  - ["Drag & Drop", { type: badge, label: "WIP", color: "yellow" }, { type: dot, color: "high" }]

```

## 4. `BoardLayout` (Spatial Container)

**Purpose:** A flex/grid container for organizing `Card` components. Depending on its configuration, it becomes a Kanban board, a CSS Artboard grid, or a full-screen scroll-snapping Slide Deck.
**Used in:** Triage Boards, Slide Decks, Visual Design Explorations.
**Asset Dependencies:** `css/board.css`, `js/drag-and-drop.js` (if interactive).

```yaml
type: board-layout
variant: kanban # or 'slides', 'grid'
columns:
  - title: "To Do"
    children:
      - type: card
        title: "Implement MiniJinja Macros"
  - title: "In Progress"
    children: ...

```

## 5. `Card` (Atomic Unit)

**Purpose:** The standard boxed container. Often nested inside a `BoardLayout` or `DataGrid`. Can contain standard markdown prose or smaller UI elements.
**Used in:** Almost everywhere.
**Asset Dependencies:** `css/card.css`.

```yaml
type: card
elevation: 2
tags: ["rust", "urgent"]
children:
  - type: markdown
    content: "Requires restructuring the `pulldown-cmark` event loop."

```

## 6. `Timeline`

**Purpose:** Sequential visualization of milestones, incident steps, or rollout plans.
**Used in:** Implementation Plans, Incident Reports.
**Asset Dependencies:** `css/timeline.css`.

```yaml
type: timeline
orientation: vertical
steps:
  - timestamp: "2026-05-18 10:00"
    title: "Initial Outage"
    type: "critical"
  - timestamp: "2026-05-18 10:15"
    title: "Rolled back to v1.2"
    type: "recovery"

```

## 7. `Notice` (Callout / Sticky Nav)

**Purpose:** Contextual UI alerts, definition boxes, or sticky navigation sidebars.
**Used in:** Code Approaches, Feature Explainers.
**Asset Dependencies:** `css/notice.css`.

```yaml
type: notice
variant: warning # info, success, danger, sticky-nav
icon: "alert-triangle"
content: |
  **Breaking Change:** The parser now expects multiple blocks.

```



