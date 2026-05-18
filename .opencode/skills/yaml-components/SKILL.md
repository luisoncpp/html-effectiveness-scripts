---
name: yaml-components
description: Use when the user is writing, editing, or debugging YAML component blocks in hybrid Markdown files for the Rust UI Compiler. Covers all 9 available primitives (notice, card, data-grid, timeline, board-layout, code-panel, svg-canvas, prompt-box, triage-board), frontmatter config, children nesting rules, and common compilation errors.
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
```

- `viewBox`: SVG viewBox attribute. Defaults to `"0 0 800 600"`.
- `interactive`: Boolean flag (reserved for future interactivity).
- `elements`: Array of `SvgElement`.
  - `type`: One of `rect`, `circle`, `text`, `edge`.
  - Coordinates (`x`, `y`, `width`, `height`, `cx`, `cy`, `r`) are optional depending on element type.
  - `class`: Optional CSS class string.
  - `text`: Text content for `text` elements.

### 8. PromptBox (Legacy)

```yaml
type: prompt-box
label: My Prompt
content: This is prompt content.
```

- `label`: Header text.
- `content`: Body text (pre-wrap, monospace).

### 9. TriageBoard (Legacy)

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

## File Locations

When adding a new primitive, touch these files:

1. `src/models/components/<name>.rs` — Data struct + `ComponentStrategy` impl.
2. `src/models/components/mod.rs` — Add `pub mod <name>;`.
3. `src/models/ui_component.rs` — Add enum variant + match arm + tests.
4. `templates/components/<name>.html` — MiniJinja template.
5. `assets/css/<name>.css` — Component styles (and `assets/js/<name>.js` if needed).
6. `src/renderer.rs` — Register template in `TemplateEngine::new()`.
7. `src/assets.rs` — Register asset path in `resolve_asset()`.
8. `tests/fixtures/<name>.md` — Example Markdown file.
9. `tests/integration_test.rs` — Integration test for the fixture.

The compiler produces a fully self-contained HTML file with zero external asset links.
