---
title: Phase 1 Test
theme: clay-slate
layout: wide
---

# Introduction

This is a test document for Phase 1.

```yaml
type: prompt-box
label: Parent Box
content: This box contains children.
children:
  - type: prompt-box
    label: Child Box
    content: Nested content here.
```

## Conclusion

The parser should produce a nested AST.
