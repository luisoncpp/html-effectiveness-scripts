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

**Output Goal References:**
- [`output_goal/17-pr-writeup.html`](../../output_goal/17-pr-writeup.html) — Expandable file-tour panels with diff highlighting (`.file` / `.code`)
- [`output_goal/01-exploration-code-approaches.html`](../../output_goal/01-exploration-code-approaches.html) — Approach cards containing dark code blocks
- [`output_goal/16-implementation-plan.html`](../../output_goal/16-implementation-plan.html) — Side-by-side code grid (`.code-grid`) with SQL and TypeScript snippets
- [`output_goal/12-incident-report.html`](../../output_goal/12-incident-report.html) — Inline diff panel (`.code-panel`) showing root-cause config change

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

**Output Goal References:**
- [`output_goal/13-flowchart-diagram.html`](../../output_goal/13-flowchart-diagram.html) — Interactive pipeline flowchart with nodes, edges, arrow markers, and a sticky detail panel
- [`output_goal/16-implementation-plan.html`](../../output_goal/16-implementation-plan.html) — Data-flow diagram (`.diagram svg`) showing optimistic-write paths
- [`output_goal/11-status-report.html`](../../output_goal/11-status-report.html) — Inline SVG bar chart inside a chart panel
- [`output_goal/10-svg-illustrations.html`](../../output_goal/10-svg-illustrations.html) — Standalone vector illustrations and iconography

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

**Output Goal References:**
- [`output_goal/11-status-report.html`](../../output_goal/11-status-report.html) — Shipped table (`.shipped`) with PR links, authors, and risk-dot badges
- [`output_goal/12-incident-report.html`](../../output_goal/12-incident-report.html) — Impact table and action-items grid with avatars, checkboxes, and due dates
- [`output_goal/16-implementation-plan.html`](../../output_goal/16-implementation-plan.html) — Risks & mitigations table with severity badges (`.sev.high`, `.sev.med`, `.sev.low`)
- [`output_goal/06-component-variants.html`](../../output_goal/06-component-variants.html) — Variant matrix showing different component states side-by-side

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

**Output Goal References:**
- [`output_goal/18-editor-triage-board.html`](../../output_goal/18-editor-triage-board.html) — Kanban board (`.board`) with four columns, drag-and-drop, and sticky column headers
- [`output_goal/09-slide-deck.html`](../../output_goal/09-slide-deck.html) — Full-screen scroll-snapping slide deck (`.slide`) with six distinct layouts
- [`output_goal/02-exploration-visual-designs.html`](../../output_goal/02-exploration-visual-designs.html) — CSS artboard grid for visual design explorations

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

**Output Goal References:**
- [`output_goal/18-editor-triage-board.html`](../../output_goal/18-editor-triage-board.html) — Ticket cards (`.ticket`) with tags, estimates, owners, and hover states inside a kanban board
- [`output_goal/11-status-report.html`](../../output_goal/11-status-report.html) — Stat cards (`.stat-card`) in a four-column summary band, including a warning variant
- [`output_goal/01-exploration-code-approaches.html`](../../output_goal/01-exploration-code-approaches.html) — Approach cards containing code panels, tradeoff tables, and chip footers
- [`output_goal/16-implementation-plan.html`](../../output_goal/16-implementation-plan.html) — Summary cells (`.summary .cell`) and mockup cards (`.mock`) with labeled borders

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

**Output Goal References:**
- [`output_goal/16-implementation-plan.html`](../../output_goal/16-implementation-plan.html) — Milestone timeline (`.milestones`) with colored dots, connecting lines, tags, and date ranges
- [`output_goal/12-incident-report.html`](../../output_goal/12-incident-report.html) — Incident timeline (`.timeline`) with impact/recovery dot states and timestamp labels

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

**Output Goal References:**
- [`output_goal/01-exploration-code-approaches.html`](../../output_goal/01-exploration-code-approaches.html) — Recommendation callout (`.reco`) with left border accent
- [`output_goal/17-pr-writeup.html`](../../output_goal/17-pr-writeup.html) — TL;DR box (`.tldr`) with left border accent and prompt box (`.prompt-box`) with label
- [`output_goal/16-implementation-plan.html`](../../output_goal/16-implementation-plan.html) — Prompt box and open-question callouts (`.q`) with left border accent
- [`output_goal/12-incident-report.html`](../../output_goal/12-incident-report.html) — Dark TL;DR panel (`.tldr`) with inverted colors and fixed TOC sidebar (`.toc`)



