# mdyaml2html

A Rust CLI compiler that transforms hybrid Markdown files (prose + fenced YAML component blocks) into self-contained, styled HTML files.

This is a fork of [ThariqS/html-effectiveness](https://github.com/ThariqS/html-effectiveness). The `output_goal/` directory contains the original demo output files (20 pre-rendered HTML documents showcasing all supported components).

## Installation

Requires [Rust](https://rustup.rs). Clone the repo and build:

```sh
cargo build --release
```

The binary will be at `target/release/mdyaml2html`.

## Usage

```sh
mdyaml2html --input <INPUT.md> --output <OUTPUT.html>
# or short form:
mdyaml2html -i <INPUT.md> -o <OUTPUT.html>
```

### Input format

A hybrid Markdown file with YAML frontmatter and fenced YAML component blocks:

````markdown
---
title: My Document
layout: wide
theme: clay-slate
---

# Section Heading

Regular prose goes here.

```yaml
type: notice
variant: warning
content: |
  <strong>Heads up:</strong> This is a callout.
```

```yaml
type: card
title: Feature Card
elevation: 2
content: Card body text.
children:
  - type: notice
    variant: info
    content: Nested inside the card.
```
````

The compiler produces a single `.html` file with all CSS and JavaScript inlined — no external asset references.

### Supported components

| Component | Description |
|---|---|
| `notice` | Callout / alert box with variant styling |
| `card` | Atomic boxed container with children nesting |
| `data-grid` | Rich HTML table |
| `timeline` | Sequential milestones or event steps |
| `board-layout` | Kanban, grid, or slides layout |
| `code-panel` | Tabbed code snippet display with diff support |
| `code-map` | Spatial code-flow diagram: grouped, syntax-highlighted code cards connected by arrows |
| `svg-canvas` | Declarative SVG wrapper |
| `prompt-box` | Monospace content box |
| `triage-board` | Planning board |
| `flowchart` | Flowchart diagram |
| `module-map` | Module dependency map |

### Frontmatter

| Field | Values | Default |
|---|---|---|
| `title` | Any string | — |
| `layout` | `reading-column`, `wide`, `canvas` | `reading-column` |
| `theme` | Theme name (e.g. `clay-slate`) | — |

## Demo outputs

Open `output_goal/index.html` in a browser to browse all 20 pre-rendered demo documents, covering code exploration, design systems, prototypes, diagrams, reports, and editor interfaces.

## YAML Components Skill

This project includes an OpenCode skill for AI coding assistants. To install it:

1. Copy `.opencode/skills/yaml-components/` into your project's `.opencode/skills/` directory.
2. The skill activates automatically when writing, editing, or debugging YAML component blocks in hybrid Markdown files.

The skill provides:
- Full syntax reference for all 12 component primitives
- Frontmatter configuration guide
- Children nesting rules
- Common compilation errors and their causes
- File checklist for adding new components

## Development

```sh
cargo test        # Run all tests (integration + snapshot)
cargo build       # Debug build
cargo run -- -i input.md -o output.html
```

### Adding a new component

1. `src/models/components/<name>.rs` — Data struct + `ComponentStrategy` impl
2. `src/models/components/mod.rs` — Re-export
3. `src/models/ui_component.rs` — Enum variant + match arm
4. `templates/components/<name>.html` — MiniJinja template
5. `assets/css/<name>.css` — Styles (add JS if needed)
6. `src/renderer.rs` — Template registration
7. `src/assets.rs` — Asset path registration
8. `tests/fixtures/<name>.md` — Example fixture
9. `tests/integration_test.rs` — Integration test
