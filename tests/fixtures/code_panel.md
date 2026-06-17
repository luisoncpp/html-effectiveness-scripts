# Code Panel Fixture

This fixture tests the code-panel component.

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
  - name: "Repo.cs"
    language: csharp
    content: |
      public List<Dictionary<string, int>> Load() {
          return new List<Dictionary<string, int>>();
      }
```

Some trailing paragraph.
