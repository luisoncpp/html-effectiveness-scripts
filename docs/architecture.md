# Architecture: Rust UI Compiler (Micro SSG)

This document describes the currently implemented architecture of the Rust UI Compiler.

## 1. System Overview

The application is a command-line micro Static Site Generator (SSG). It reads hybrid Markdown files containing standard prose and embedded YAML blocks, deserializes the YAML into strongly-typed component structs, and renders them into styled HTML using the MiniJinja templating engine. The final output is a single, self-contained HTML file with all CSS injected inline.

## 2. Directory Structure

```text
src/
├── main.rs               # Entry point; parses CLI args and invokes the compiler
├── cli.rs                # CLI argument definitions (clap derive)
├── compiler.rs           # Orchestrates the read -> parse -> render -> write pipeline
├── parser.rs             # Markdown event stream interception (pulldown-cmark)
├── renderer.rs           # HTML templating engine (minijinja)
├── models/
│   ├── mod.rs            # Module exports
│   ├── base.rs           # The Renderable trait
│   ├── ui_component.rs   # Strategy Router: internally tagged enum for deserialization
│   └── components/
│       ├── mod.rs
│       └── prompt_box.rs # Data struct for the prompt-box component

tests/
├── integration_test.rs     # End-to-end tests for basic and hybrid markdown
├── snapshot_test.rs      # Layout regression tests using insta
└── fixtures/
    ├── basic.md            # Standard markdown without components
    └── hybrid.md           # Markdown containing a fenced YAML component block

templates/
├── base.html               # Global layout with injected CSS variables
└── components/
    └── prompt_box.html     # Template for the prompt-box component
```

## 3. Data Flow & Execution Pipeline

The compiler executes the following stages in sequence inside `compiler::run_compilation`:

1. **Input Stage** (`main.rs` -> `cli.rs`)
   The user invokes the binary with `--input` and `--output` flags. `clap` validates and produces a `CliArgs` struct.

2. **Read Stage** (`compiler.rs`)
   The raw `.md` file is read into a `String` buffer.

3. **Parse Stage** (`parser.rs`)
   A `pulldown_cmark::Parser` iterates over the Markdown event stream.
   * Standard events (headings, paragraphs, lists, etc.) are buffered into a vector.
   * When a fenced `yaml` code block is detected (`Event::Start(Tag::CodeBlock(...))`), the state machine switches into interception mode.
   * All text inside the YAML block is diverted into a `yaml_buffer`.
   * When the block ends (`Event::End(TagEnd::CodeBlock)`), the buffered YAML string is passed to `serde_yaml::from_str::<UiComponent>`. A placeholder HTML comment (`<!-- COMPONENT_PLACEHOLDER_{index} -->`) is injected into the standard event stream at the exact position the block occupied.

4. **Render Stage** (`renderer.rs`)
   The buffered standard events are converted to HTML via `pulldown_cmark::html::push_html`.
   The `render_document` function then:
   * Iterates over the deserialized `components` vector.
   * Calls `component.render(&template_engine)` for each one. This uses the `Renderable` trait to dispatch to the correct MiniJinja template.
   * Replaces each `COMPONENT_PLACEHOLDER_{index}` in the HTML body with the rendered component string.
   * Wraps the final body in `templates/base.html`, which injects all CSS variables into the `<head>`.

5. **Output Stage** (`compiler.rs`)
   The fully self-contained HTML string is written to the output path specified in `CliArgs`.

## 4. Core Modules

### `cli.rs`

* **Responsibility:** Defines the CLI contract using `clap`'s derive macro.
* **Struct:** `CliArgs` with `input: PathBuf` and `output: PathBuf`.
* **Testing:** Unit tests verify parsing of valid flags, long-form flags, and rejection of missing/unknown arguments.

### `parser.rs`

* **Responsibility:** Intercepts the Markdown AST event stream to swallow YAML blocks and extract their raw text.
* **Struct:** `ParsedDocument { html: String, components: Vec<UiComponent> }`
* **State Machine:** Uses two mutable flags (`in_yaml: bool`, `yaml_buffer: String`) and a component accumulator. When interception is active, standard events are suppressed; on block close, the YAML is deserialized and a placeholder comment is emitted.
* **Key Design Decision:** Placeholders ensure components render at their original document position, maintaining organic flow within standard Markdown HTML.

### `models::ui_component`

* **Responsibility:** Acts as the central Strategy Router for YAML deserialization.
* **Enum:** `UiComponent` uses `#[serde(tag = "type")]` for internally tagged polymorphism.
* **Variants:** Currently routes `type: prompt-box` to `PromptBox(PromptBoxData)`.
* **Fail-fast:** Unknown types or missing required fields result in a deserialization `Err`, which propagates up and halts compilation with a human-readable message.

### `models::base.rs`

* **Responsibility:** Defines the contract between data models and the templating engine.
* **Trait:** `Renderable`
  ```rust
  pub trait Renderable {
      fn render(&self, engine: &TemplateEngine) -> String;
  }
  ```
* **Purpose:** Decouples the shape of the data from the rendering mechanism. Adding a new component type only requires implementing this single trait method.

### `renderer.rs`

* **Responsibility:** Manages template loading, rendering, and final document assembly.
* **Struct:** `TemplateEngine` wraps `minijinja::Environment<'static>`.
* **Templates:** Loaded at build time via `include_str!` so the binary has zero external template dependencies at runtime.
  * `base.html` — Global layout containing all CSS variables and component styles.
  * `prompt_box.html` — Component-specific markup.
* **Function:** `render_document` performs placeholder replacement and base-template wrapping.

## 5. Templating & CSS Strategy

The output HTML is fully self-contained. All styling is achieved through CSS custom properties (variables) defined in `:root` inside `templates/base.html`. Component templates use these variables for colors, spacing, borders, and typography. No external `.css` files are required, and the compiled binary itself embeds the templates.

## 6. Component Strategy Pattern

Adding a new UI component requires exactly three steps:

1. **Model:** Create a new struct in `models::components` and add it as a variant to `UiComponent`.
2. **Template:** Add a corresponding `.html` file in `templates/components/`.
3. **Render:** Implement `Renderable` for the new `UiComponent` variant, mapping its fields to the template context.

This pattern keeps the parser, compiler, and base layout completely agnostic to individual component shapes.

## 7. Testing Strategy

* **Unit Tests:** Each module has inline `#[cfg(test)]` tests covering happy paths and error paths (e.g., unknown YAML types, missing fields, CLI flag rejection).
* **Integration Tests:** `tests/integration_test.rs` exercises the full `run_compilation` pipeline against fixture files (`basic.md`, `hybrid.md`), asserting on the presence/absence of specific HTML tags and the swallowing of raw YAML.
* **Snapshot Tests:** `tests/snapshot_test.rs` uses the `insta` crate to capture the entire rendered HTML output of both fixture files. Any future change that alters layout, CSS, or markup will fail until the snapshot is explicitly reviewed and accepted, preventing regressions.

## 8. Key Design Decisions

| Decision | Rationale |
|----------|-----------|
| **Placeholder comments** (`<!-- COMPONENT_PLACEHOLDER_N -->`) | Preserves the exact insertion point of YAML blocks within standard Markdown flow, without coupling the parser to rendering logic. |
| **`include_str!` for templates** | Produces a single static binary with no runtime template file I/O, simplifying deployment. |
| **Internally tagged enum (`#[serde(tag = "type")]`)** | Keeps YAML concise and human-readable; maps cleanly to the Strategy Pattern in Rust. |
| **Self-contained CSS** | Eliminates external asset dependencies; the generated HTML is a single portable file. |
| **Fakes over mocks** | Unit tests verify observable output (HTML strings, file contents) rather than internal call sequences. |
