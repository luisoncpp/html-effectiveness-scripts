---
title: All Primitives Demo
layout: wide
theme: clay-slate
---

# UI Primitives Showcase

This document demonstrates every available YAML component primitive.

## Notice

```yaml
type: notice
variant: warning
icon: alert-triangle
content: |
  <strong>Breaking Change:</strong> The parser now expects multiple blocks. **A**
```

## Card

```yaml
type: card
title: Feature Card
elevation: 2
tags:
  - rust
  - urgent
content: |
  This card demonstrates tags, elevation, and content.**A**
children:
  - type: notice
    variant: info
    content: Nested notice inside a card.
```

## Data Grid

```yaml
type: data-grid
columns:
  - Feature
  - Status
  - Risk
rows:
  - ["AST Traversal", "Shipped", "Low"]
  - ["Drag & Drop", "WIP", "High"]
  - ["Syntax Highlighting", "Planned", "Medium"]
```

## Timeline

```yaml
type: timeline
orientation: vertical
steps:
  - timestamp: "Week 1 Â· Monâ€“Tue"
    title: "Schema & API contract"
    type: "critical"
    description: "New comments and comment_reads tables, migrations, and the tRPC router stubs."
    tags:
      - packages/db
      - packages/api
      - migration 0042
  - timestamp: "Week 2 Â· Wedâ€“Fri"
    title: "UI polish & edge cases"
    type: "recovery"
    description: "Thread nesting, optimistic updates, and empty-state screens."
    tags:
      - packages/ui
      - ux
  - timestamp: "Week 3 Â· Mon"
    title: "Ship to beta"
    type: "info"
    description: "Feature flag enabled for 10% of users."
    tags:
      - release
      - beta
```

## Board Layout

```yaml
type: board-layout
variant: kanban
columns:
  - title: "To Do"
    items:
      - "Implement SVG canvas"
      - "Write integration tests"
  - title: "In Progress"
    items:
      - "Refactor parser"
  - title: "Done"
    items:
      - "Setup CI"
```

## Code Panel

```yaml
type: code-panel
tabs:
  - name: "src/compiler.rs"
    language: rust
    diff: true
    risk: attention
    added: 58
    removed: 0
    content: |
      - let tree = parse_single();
      + let ast = parse_blocks();
  - name: "Cargo.toml"
    language: toml
    risk: safe
    added: 2
    removed: 1
    content: |
      [dependencies]
      pulldown-cmark = "0.9"
```

## SVG Canvas

```yaml
type: svg-canvas
viewBox: "0 0 400 200"
interactive: false
elements:
  - type: rect
    x: 10
    y: 10
    width: 100
    height: 60
    class: "node-primary"
  - type: circle
    cx: 200
    cy: 100
    r: 40
    class: "node-secondary"
  - type: text
    x: 10
    y: 100
    text: "Hello SVG"
  - type: edge
    x: 110
    y: 40
    x2: 200
    y2: 100
    class: "edge"
```

## Flowchart

```yaml
type: flowchart
title: "What happens when you git push"
description: "The deploy pipeline for acme/web."
viewBox: "0 0 620 400"
nodes:
  - id: push
    type: terminal
    x: 230
    y: 12
    width: 160
    height: 44
    label: "git push main"
    detail_idx: 0
  - id: ci
    type: rect
    x: 210
    y: 92
    width: 200
    height: 48
    label: "CI Â· lint + typecheck"
    sublabel: "~2 min Â· ci.yml"
    detail_idx: 1
  - id: test
    type: rect
    x: 210
    y: 176
    width: 200
    height: 48
    label: "Unit + integration tests"
    sublabel: "~6 min Â· 3 shards"
    detail_idx: 2
  - id: gate
    type: diamond
    x: 268
    y: 262
    width: 84
    height: 48
    label: "pass?"
    detail_idx: 3
  - id: done
    type: terminal
    x: 230
    y: 350
    width: 160
    height: 44
    label: "Deploy complete"
    detail_idx: 4
edges:
  - from: push
    to: ci
    d: "M310,56 L310,92"
  - from: ci
    to: test
    d: "M310,140 L310,176"
  - from: test
    to: gate
    d: "M310,224 L310,262"
  - from: gate
    to: done
    edge_type: yes
    label: pass
    d: "M310,310 L310,350"
details:
  - title: "git push main"
    meta: "trigger Â· 0s"
    body: "A push or merge to main fires the deploy workflow."
    code: "on:\n  push:\n    branches: [main]"
  - title: "CI Â· lint + typecheck"
    meta: "github actions Â· ~2 min"
    body: "Runs ESLint and tsc --noEmit in parallel."
    code: "jobs:\n  lint:\n    run: pnpm lint && pnpm typecheck"
  - title: "Unit + integration tests"
    meta: "github actions Â· ~6 min"
    body: "Vitest unit suite plus the API integration tests."
    code: "run: pnpm test"
  - title: "Tests pass?"
    meta: "decision"
    body: "Any shard failing short-circuits here."
    code: "needs: [test]\nif: success()"
  - title: "Deploy complete"
    meta: "terminal Â· ~30 min total"
    body: "Commit status flips to success."
    code: "âœ“ web@a1b9e3f live on prod"
```

## Module Map

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
    class: highlight
  - id: renderer
    label: "renderer.rs"
    x: 250
    y: 50
    width: 120
    height: 50
  - id: compiler
    label: "compiler.rs"
    x: 450
    y: 50
    width: 120
    height: 50
  - id: assets
    label: "assets.rs"
    x: 250
    y: 180
    width: 120
    height: 50
edges:
  - from: parser
    to: renderer
    d: "M170,75 L250,75"
  - from: renderer
    to: compiler
    d: "M370,75 L450,75"
  - from: assets
    to: renderer
    d: "M310,180 L310,100"
```

## Triage Board (Legacy)

```yaml
type: triage-board
eyebrow: Demo / sprint / triage
title: Cycle 15 triage
subtitle: Twenty-four open tickets, pre-sorted into a best guess.
hintline: drag tickets between columns
```

## Code Map

```yaml
type: code-map
width: 900
height: 360
groups:
  - label: Entry Point
    variant: amber
    x: 16
    y: 10
    width: 320
    height: 200
  - label: Initialization
    variant: green
    x: 380
    y: 10
    width: 500
    height: 330
cards:
  - id: main
    x: 32
    y: 60
    width: 280
    language: ts
    code: |
      main(): void {
        this.[[run]]();
      }
  - id: run
    x: 400
    y: 80
    width: 300
    language: ts
    code: |
      async [[run]](): Promise<void> {
        const config = this.loadConfig();
arrows:
  - from: main.run
    to: run.run
```

---

That's all primitives in a single document.