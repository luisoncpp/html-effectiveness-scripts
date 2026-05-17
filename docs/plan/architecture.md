# Architecture Specification: Rust UI Compiler (Micro SSG)

## 1. System Overview

This project is a standalone Rust command-line interface (CLI) application that acts as a micro Static Site Generator (SSG). It parses hybrid files containing standard Markdown and embedded YAML blocks, routing the YAML data through a Strategy Pattern to render complex, styled HTML components using a templating engine.

The goal is to being able to convert simple md + YAML files to something like the examples sketched in `output-goal`.

## 2. Directory Structure

```text
src/
├── main.rs               # Application entry point and global error handling
├── cli.rs                # CLI argument definitions (using `clap`)
├── compiler.rs           # Core pipeline orchestration (read -> parse -> render -> write)
├── parser.rs             # Markdown event stream interception (`pulldown-cmark`)
├── renderer.rs           # HTML templating engine initialization (`minijinja`)
└── models/
    ├── mod.rs            # Module exports and shared types
    ├── ui_component.rs   # The core Strategy Router (Internally Tagged Enum)
    └── components/       # Individual component data structures
        ├── mod.rs
        ├── prompt_box.rs
        ├── triage_board.rs
        └── system_diagram.rs

```

## 3. Data Flow & Execution Pipeline

1. **Input Stage:** `main.rs` invokes `cli.rs` to parse the target file path and output destination.
2. **Read Stage:** `compiler.rs` reads the raw `.md` file into a `String` buffer.
3. **Parse Stage:** `parser.rs` initializes a `pulldown_cmark::Parser`. It iterates through the event stream.
* Standard Markdown events are buffered.
* When an `Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced("yaml")))` is detected, the text stream is diverted into a YAML buffer.


4. **Routing Stage:** Upon `Event::End(Tag::CodeBlock)`, the YAML buffer is passed to `serde_yaml::from_str::<UiComponent>`. The internal tag (`type`) dictates which struct is instantiated.
5. **Render Stage:** The instantiated struct implements a `Renderable` trait. `compiler.rs` passes the struct to `renderer.rs`, which looks up the corresponding `minijinja` template and returns the HTML string.
6. **Output Stage:** The rendered component HTML is injected back into the Markdown HTML stream. The final payload is wrapped in the base layout (CSS injection) and written to disk.

## 4. Core Modules and Responsibilities

### Module: `models::ui_component`

Acts as the central Strategy Router using `serde` tagging.

* **Structs/Enums:** * `enum UiComponent`: Uses `#[serde(tag = "type")]`. Contains variants like `PromptBox(PromptBoxData)`, `TriageBoard(TriageBoardData)`.
* **Responsibilities:** Strictly handles deserialization routing. Fails fast if a YAML block lacks a registered `type`.

### Module: `parser`

Handles the AST/Event stream manipulation.

* **Structs/Enums:**
* `struct MarkdownInterceptor`: Maintains state (`in_yaml_block: bool`, `yaml_buffer: String`).


* **Responsibilities:** Prevents YAML blocks from being rendered as standard `<pre><code>` HTML tags. Extracts the raw string for the router.

### Module: `renderer`

Manages the templates and CSS context.

* **Structs/Enums:**
* `struct TemplateEngine`: Wraps `minijinja::Environment`.


* **Responsibilities:** Loads `base.html` (containing CSS variables) and component-specific templates (e.g., `components/prompt_box.html`). Executes templates with the data structures provided by the `UiComponent` variants.
