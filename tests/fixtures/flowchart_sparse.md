---
title: Sparse flowchart
---

```yaml
type: flowchart
title: "Sparse detail links"
viewBox: "0 0 400 200"
nodes:
  - id: start
    type: terminal
    x: 160
    y: 12
    width: 80
    height: 40
    label: "Start"
  - id: step-a
    type: rect
    x: 140
    y: 72
    width: 120
    height: 48
    label: "Step A"
    detail_idx: 0
  - id: step-b
    type: rect
    x: 140
    y: 132
    width: 120
    height: 48
    label: "Step B"
    detail_idx: 1
edges:
  - from: start
    to: step-a
    d: "M200,52 L200,72"
  - from: step-a
    to: step-b
    d: "M200,120 L200,132"
details:
  - title: "Step A detail"
    meta: "first described step"
    body: "Body for step A."
  - title: "Step B detail"
    meta: "second described step"
    body: "Body for step B."
```
