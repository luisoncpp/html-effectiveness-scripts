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

- `title` (optional): Appears in `<title>` and can be referenced by layouts.
- `layout` (default: `reading-column`): `reading-column` (65ch max), `wide`, or `canvas`.
- `theme` (optional, default: empty): When set, loads `assets/tokens/<theme>.css`. Omit or leave empty to skip theme tokens.

## Markdown vs. HTML: where each is allowed

This is the single most common source of confusion. There are two completely
separate text contexts in a document:

- **Document prose** — any text *outside* a fenced `yaml` block. This is parsed
  as standard Markdown (via pulldown-cmark). Use `*emphasis*`, `**bold**`,
  `# headings`, `- lists`, `[links](url)`, tables, etc. as normal.
- **Component fields** — text fields *inside* a YAML component block. Most prose
  and label fields are run through the Markdown renderer (see the table below);
  the result is still emitted with `|safe`, so inline HTML works in those fields
  too. A handful of fields remain **raw HTML / plain text** with no Markdown
  processing — chiefly SVG `<text>` content (where HTML tags wouldn't render) and
  literal code blocks.

**Consequence:** inside a raw-HTML-only field, Markdown syntax does *not* work.
Writing `*important*` renders the literal asterisks; you must write
`<em>important</em>`.

### Markdown-aware fields

Two helpers in `src/models/components/mod.rs` do the work:

- `render_markdown` — full **block** Markdown (paragraphs, lists, headings; wraps
  in `<p>`). Used for multi-line content areas.
- `render_markdown_inline` — **inline** Markdown (`**bold**`, `*em*`, `` `code` ``,
  `[links](url)`) with the lone wrapping `<p>` stripped. Used for titles, table
  cells, list items, and other single-line fields. Content with multiple
  paragraphs falls back to the full block render.

| Component | Block-Markdown fields | Inline-Markdown fields |
|-----------|----------------------|------------------------|
| Notice | `content` | — |
| Card | `content` | `title` |
| DataGrid | — | `columns[]`, `rows[][]` (cells) |
| Timeline | — | `steps[].title`, `steps[].description` (both inline Markdown) |
| BoardLayout | — | `columns[].title`, `columns[].items[]` |
| Flowchart | — | `title`, `description`, `details[].title`/`meta`/`body` |
| ModuleMap | — | `title` |
| TriageBoard | — | `title`, `subtitle`, `hintline` |

```yaml
type: notice
variant: warning
content: |
  **Markdown works here** — and so does <strong>inline HTML</strong>.
```

Note: because cells/titles now go through the Markdown renderer, bare `&`, `<`,
`>` are HTML-escaped (e.g. `Drag & Drop` → `Drag &amp; Drop`). Write real entities
or HTML if you need literal markup.

### Fields that are NOT Markdown (raw text / HTML)

These are emitted without Markdown processing — `*foo*` renders literally:

| Component | Raw fields | Why |
|-----------|------------|-----|
| Card | `tags[]` | short chip labels |
| Timeline | `steps[].timestamp`, `steps[].tags[]` | date / chip labels |
| TriageBoard | `eyebrow` | breadcrumb label |
| SvgCanvas | `elements[].text` | SVG `<text>` — HTML wouldn't render |
| Flowchart | node `label`/`sublabel`, edge `label` | SVG `<text>` |
| ModuleMap | node `label`, edge `label` | SVG `<text>` |
| PromptBox | `label`, `content` | pre-wrap monospace, shown verbatim |

A further case is **literal code/`<pre>` fields** — neither Markdown nor
interpreted HTML; the text appears exactly as written:
`code-panel.tabs[].content`, `flowchart.details[].code`, and `code-map.cards[].code`
(the last is additionally syntax-highlighted by language).

## Available Primitives

Throughout this section, **(required)** means deserialization fails if the field
is missing. **(optional)** means it may be omitted. **(default: …)** means the
field is optional and the compiler supplies that value when absent.

### 1. Notice

Callout / alert box with left border accent.

```yaml
type: notice
variant: warning    # info | success | danger | warning | sticky-nav
icon: alert-triangle  # optional
content: |
  <strong>Breaking Change:</strong> The parser now expects multiple blocks.
```

- `variant` (required): CSS modifier class. Determines border/color accent (`info`, `success`, `danger`, `warning`, `sticky-nav`, etc.).
- `content` (required): Block Markdown (and inline HTML). Use `**bold**`, lists, etc.
- `icon` (optional): Rendered as text if present.

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

- `title` (optional): Header. Inline Markdown (`**bold**`, `` `code` ``).
- `elevation` (default: `1`): Visual depth (box shadow). `1`, `2`, or `3`.
- `tags` (default: `[]`): Plain-text chip labels.
- `content` (optional): Block Markdown (and inline HTML) inside the card body.
- `children` (optional): Array of nested component blocks (see Nesting Rules).

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

- `columns` (required): Array of header strings. Inline Markdown supported.
- `rows` (required): Array of row arrays. Cell values support inline Markdown (`**bold**`, `` `code` ``, links) and inline HTML badges. Bare `&`/`<`/`>` are HTML-escaped.

### 4. Timeline

Sequential milestones or incident steps.

```yaml
type: timeline
orientation: vertical   # default
steps:
  - timestamp: "2026-05-18 10:00"
    title: "Initial Outage"
    type: "critical"
    description: "Users unable to reach the API."
    tags:
      - sev-1
  - timestamp: "2026-05-18 10:15"
    title: "Rolled back to v1.2"
    type: "recovery"
    description: "Traffic restored; monitoring continues."
```

- `orientation` (default: `vertical`): Layout modifier (`vertical`, etc.).
- `steps` (required): Array of `TimelineStep`. Each step needs all required fields below.
  - `timestamp` (required): String label (any text — date, week range, etc.). Plain text, not Markdown.
  - `title` (required): Step heading. Inline Markdown supported.
  - `type` (required): Step category string. **Every step must include this** — omitting it fails deserialization. The only built-in visual effect today is `done`, which styles the dot as completed; other values (e.g. `critical`, `recovery`, `info`) are accepted and reserved for future styling.
  - `description` (optional): Body text under the title. Inline Markdown supported.
  - `tags` (optional): Plain-text chip labels.

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

- `variant` (default: `kanban`): Layout mode — `kanban` (flex columns), `grid` (CSS grid), or `slides` (horizontal scroll-snapping flex).
- `columns` (required): Array of `BoardColumn`.
  - `title` (required): Column header. Inline Markdown supported.
  - `items` (default: `[]`): Strings rendered as simple cards inside the column. Inline Markdown supported.

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

- `tabs` (required): Array of `CodeTab`. At least one tab is expected.
  - `name` (required): Tab label shown in the tab bar and file header.
  - `language` (required): Syntax-highlighting language id (e.g. `rust`, `toml`). Emitted as `language-<name>` on `<code>`.
  - `content` (required): Raw code text. In diff tabs, prefix lines with `+` or `-`.
  - `diff` (default: `false`): When `true`, renders a diff view and adds `code-panel--diff` to the panel.
  - `risk` (optional): Risk badge label in the file header (e.g. `attention`, `safe`).
  - `added` (optional): Integer shown as `+<n>` in the file header.
  - `removed` (optional): Integer shown as `-<n>` in the file header.

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

- `viewBox` (default: `"0 0 800 600"`): SVG viewBox attribute.
- `interactive` (default: `false`): Boolean flag (reserved for future interactivity).
- `elements` (required): Array of `SvgElement` (may be empty).
  - `type` (required): One of `rect`, `circle`, `text`, `edge`.
  - Coordinates (`x`, `y`, `width`, `height`, `cx`, `cy`, `r`, `x2`, `y2`) (optional): Depends on element type; see `edge` below.
  - `class` (optional): CSS class string.
  - `text` (optional): Text content for `text` elements. Plain text (not Markdown).
  - `marker` (optional): SVG marker id (without `#`) for `edge` elements. Emitted as `marker-end="url(#<marker>)"`.

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

- `title` (required): Header title. Inline Markdown supported.
- `description` (optional): Subheader text. Inline Markdown supported.
- `viewBox` (required): SVG viewport string (e.g. `"0 0 620 400"`). No default.
- `nodes` (required): Array of flowchart nodes.
  - `id` (required): Unique node id (referenced by `edges[].from` / `edges[].to`).
  - `type` (required): Node shape — `terminal`, `rect`, or `diamond`.
  - `x`, `y`, `width`, `height` (required): Position and size in viewBox coordinates.
  - `label` (required): Primary label. Plain text / raw HTML in SVG `<text>`.
  - `sublabel` (optional): Secondary label below the primary. Plain text / raw HTML.
  - `detail_idx` (optional): Index into `details` shown when the node is selected.
- `edges` (required): Array of connections.
  - `from`, `to` (required): Node ids.
  - `d` (required): SVG path `d` attribute.
  - `edge_type` (default: `normal`): `yes`, `no`, or `normal` — controls arrow color/style.
  - `label` (optional): Edge label. Plain text / raw HTML.
- `details` (required): Sidebar items (may be empty). Referenced by `nodes[].detail_idx`.
  - `title`, `meta`, `body` (required): Inline Markdown supported.
  - `code` (optional): Literal code block shown in the sidebar (not Markdown-processed).

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

- `title` (required): Map section title. Inline Markdown supported.
- `viewBox` (required): SVG viewBox string. No default.
- `nodes` (required): Array of module boxes.
  - `id` (required): Unique node id.
  - `label` (required): Display name. Plain text / raw HTML in SVG `<text>`.
  - `x`, `y`, `width`, `height` (required): Position and size.
  - `class` (optional): CSS class on the node rect (e.g. `highlight`).
- `edges` (required): Array of dependency arrows.
  - `from`, `to` (required): Node ids.
  - `d` (required): SVG path `d` attribute.
  - `label` (optional): Edge label. Plain text / raw HTML.

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

- `title` (optional): Heading above the canvas.
- `width` (default: `1200`): Canvas width in px. Scrolls horizontally when wider than the page.
- `height` (required): Canvas height in px. No default.
- `groups` (default: `[]`): Decorative labeled containers behind the cards.
  - `label`, `x`, `y`, `width`, `height` (required).
  - `variant` (default: `plain`): Accent color — `amber`, `green`, `blue`, `clay`, or `plain`.
- `cards` (default: `[]`): Code snippet boxes.
  - `id`, `x`, `y`, `width`, `code` (required).
  - `height` (optional): Auto-sizes to content when omitted.
  - `title` (optional): File-path header.
  - `language` (default: empty / generic): Syntax-highlighting language id.
- **Anchor tokens**: Inside `code`, wrap a token in `[[...]]` to render it as
  a blue highlighted chip and register it as an arrow endpoint with id
  `cardId.token`. Use `[[myId|display text]]` when the display text is not a
  valid id or appears more than once in the card. Lines containing an anchor
  get a highlighted background.
- `arrows` (default: `[]`): Curved connectors drawn at load time by inlined JS.
  - `from`, `to` (required): `cardId.anchorId` (token) or `cardId` (card edge).

### 11. PromptBox (Legacy)

```yaml
type: prompt-box
label: My Prompt
content: This is prompt content.
```

- `label` (required): Header text. Plain text (not Markdown).
- `content` (required): Body text (pre-wrap, monospace). Shown verbatim.

### 12. TriageBoard (Legacy)

```yaml
type: triage-board
eyebrow: Acme / editor / triage
title: Cycle 14 triage
subtitle: Planning board
hintline: drag tickets between columns
```

- `eyebrow` (required): Breadcrumb label. Plain text (not Markdown).
- `title` (required): Main heading. Inline Markdown supported.
- `subtitle` (required): Subheading. Inline Markdown supported.
- `hintline` (required): Helper text. Inline Markdown supported.

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
| `Failed to deserialize YAML component` | Unknown `type` value or missing required field (e.g. `timeline.steps[].type`, `flowchart.viewBox`, `code-map.height`). |
| `children must be an array` | The `children` key is present but not a YAML sequence. |
| `Unsupported child block type` | A child in `children` lacks a `type` key. |
| Missing styles in output | Component CSS not registered in `assets.rs` `resolve_asset()`. |
| Render error comment in HTML | Template name mismatch or missing template registration in `renderer.rs`. |

The compiler produces a fully self-contained HTML file with zero external asset links.

Unless you are asked otherwise, save the file with the `.yaml.md` extension to indicate it contains YAML component blocks.
