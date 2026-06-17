---
title: Regression test
theme: clay-slate
---

```yaml
type: card
title: Card Markdown Test
content: |
  * Item 1
  * Item 2 with `inline code`
```

```yaml
type: notice
variant: info
content: |
  This is notice text with **bold formatting**.
```

```yaml
type: card
title: Card with `code` title
content: Body
```

```yaml
type: data-grid
columns: ["**Feature**", "Status"]
rows:
  - ["`AST`", "Shipped & live"]
```

```yaml
type: board-layout
variant: kanban
columns:
  - title: "**Now**"
    items:
      - "Ship `parser`"
```

```yaml
type: timeline
orientation: vertical
steps:
  - timestamp: "Day 1"
    title: "Build **parser**"
    type: "critical"
    description: "Wire up the `AST` walker."
```
