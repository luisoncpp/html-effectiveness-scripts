# Architecture: Rust UI Compiler (Micro SSG)

This document describes the currently implemented architecture of the Rust UI Compiler.

## 1. System Overview

The application is a command-line micro Static Site Generator (SSG). It reads hybrid Markdown files containing YAML frontmatter, standard prose, and embedded fenced YAML component blocks. The parser produces a block-based AST (`Vec<Block>`) with `Prose` (raw HTML) and `Component` (deserialized structs) variants. The renderer walks the AST, dispatching each component to its MiniJinja template with children rendered recursively. CSS and JavaScript assets are tracked per component via an `AssetRegistry`, resolved at build time with `include_str!`, and inlined into the final self-contained HTML file. Frontmatter configures document-level settings (title, layout width, theme tokens).

## 2. Directory Structure

```text
src/
├── main.rs               # Binary entry point; parses CLI args, invokes compiler
├── lib.rs                # Library root; re-exports all public modules
├── cli.rs                # CLI argument definitions (clap derive)
├── compiler.rs           # Orchestrates read -> parse -> render -> write pipeline
├── parser.rs             # Markdown -> AST: frontmatter extraction, block parsing
├── renderer.rs           # Template engine & document assembly (MiniJinja)
├── assets.rs             # Asset registry: collects, deduplicates, resolves CSS/JS
├── models/
│   ├── mod.rs            # Module exports
│   ├── base.rs           # Renderable trait
│   ├── block.rs          # Block enum (Prose | Component)
│   ├── document_context.rs # Frontmatter model (title, layout, theme)
│   ├── ui_component.rs   # Internally-tagged enum; ComponentBlock wrapper
│   └── components/
│       ├── mod.rs
│       ├── prompt_box.rs # Data struct for prompt-box
│       └── triage_board.rs # Data struct for triage-board

assets/
├── css/
│   ├── base.css          # Global layout & typography reset
│   ├── prompt_box.css    # Prompt-box component styles
│   └── triage_board.css  # Triage-board Kanban layout styles
├── js/
│   ├── triage_board.js   # Drag-and-drop board interactivity
│   └── test.js           # Test-only asset (used in unit tests)
└── tokens/
    └── clay-slate.css    # Theme token overrides

templates/
├── base.html             # Global HTML shell (title, layout class, inline slots)
└── components/
    ├── prompt_box.html   # Prompt-box markup with children slot
    └── triage_board.html # Triage-board header/toolbar/board markup + children slot

tests/
├── integration_test.rs   # End-to-end pipeline tests against fixture files
├── snapshot_test.rs      # Layout regression tests using insta
├── fixtures/
│   ├── basic.md            # Standard markdown (no frontmatter, no components)
│   ├── hybrid.md           # Prose + single YAML component
│   ├── triage_board.md     # Frontmatter + triage-board component
│   └── test_phase1.md      # Frontmatter + nested children
└── snapshots/
    ├── snapshot_test__snapshot_basic_markdown.snap
    ├── snapshot_test__snapshot_hybrid_markdown.snap
    └── snapshot_test__snapshot_triage_board.snap
```

## 3. Data Flow & Execution Pipeline

The compiler executes the following stages in sequence inside `compiler::run_compilation`:

1. **Input Stage** (`main.rs` -> `cli.rs`)
   The user invokes the binary with `--input` and `--output` flags. `clap` validates and produces a `CliArgs` struct.

2. **Read Stage** (`compiler.rs`)
   The raw `.md` file is read into a `String` buffer.

3. **Parse Stage** (`parser.rs`)
   * **Frontmatter extraction:** If the document starts with `---`, the delimited YAML block is deserialized into `DocumentContext` (title, `layout_wrapper`, `theme_tokens`). The remainder becomes the body.
   * **Body parsing:** A `pulldown_cmark::Parser` iterates over Markdown events. Standard events are accumulated into a `prose_events` buffer. When a fenced `yaml` code block is detected, prose is flushed to a `Block::Prose(String)` (HTML rendered via `pulldown_cmark::html::push_html`), and the YAML block text is deserialized into a `ComponentBlock` containing a `UiComponent` and its nested `children: Vec<Block>`. Unknown component types or missing required fields produce an error.

   The result is a `ParsedDocument { blocks: Vec<Block>, context: DocumentContext }`.

4. **Asset Collection Stage** (`assets.rs`)
   `AssetRegistry::from_blocks` walks the entire AST, calling each `UiComponent::required_assets()` to collect the set of CSS and JS file paths. `with_theme` adds the token CSS if a theme was specified. `with_base_assets` adds the global `base.css`. `BTreeSet` ensures deduplication.

5. **Render Stage** (`renderer.rs`)
   * `render_blocks` walks the `blocks` vector. `Block::Prose` is emitted as-is (already HTML). `Block::Component` calls `ComponentBlock::render()`, which dispatches to the correct MiniJinja template and recursively renders children into a `children` template variable.
   * `render_document` wraps the assembled body in `templates/base.html`, injecting `{{ content }}`, `{{ title }}`, `{{ layout }}`, `{{ theme }}`, `{{ inline_styles }}`, and `{{ inline_scripts }}`.

6. **Output Stage** (`compiler.rs`)
   The fully self-contained HTML string is written to the output path specified in `CliArgs`.

## 4. Core Modules

### `cli.rs`

* **Responsibility:** Defines the CLI contract using `clap`'s derive macro.
* **Struct:** `CliArgs` with `input: PathBuf` and `output: PathBuf`.
* **Testing:** Unit tests verify parsing of valid flags, long-form flags, and rejection of missing/unknown arguments.

### `parser.rs`

* **Responsibility:** Converts hybrid Markdown into a structured AST (`ParsedDocument`).
* **Structs:** `ParsedDocument { blocks: Vec<Block>, context: DocumentContext }`
* **Frontmatter:** `extract_frontmatter` detects `---` delimiters and deserializes `DocumentContext` (with `LayoutType` enum: `ReadingColumn`, `Wide`, `Canvas`). Missing or empty frontmatter defaults to `reading-column` layout with no title and no theme.
* **State Machine:** Tracks `in_yaml: bool` and a `yaml_buffer: String`. Standard events are buffered; on fenced `yaml` block boundaries, prose is flushed to HTML and the YAML is deserialized. Children inside YAML blocks are parsed recursively via `parse_children`.
* **Key Design Decision:** Block-based AST preserves the exact document order (prose and components interleaved) without placeholder tokens. Prose is pre-rendered to HTML at parse time.

### `models::block`

* **Responsibility:** Defines the document AST node.
* **Enum:** `Block` with variants:
  * `Prose(String)` — Pre-rendered HTML from standard Markdown.
  * `Component(ComponentBlock)` — A component with its nested children.

### `models::document_context`

* **Responsibility:** Carries document-level configuration from YAML frontmatter.
* **Struct:** `DocumentContext { title: Option<String>, layout_wrapper: LayoutType, theme_tokens: String }`
* **LayoutType:** `ReadingColumn` (default, max-width 65ch), `Wide`, `Canvas`. Applied as `class="layout-{{ layout }}"` on `<body>`. Theme tokens are loaded from `assets/tokens/<theme>.css`.

### `models::ui_component`

* **Responsibility:** Central Strategy Router for YAML deserialization and asset declarations.
* **Enum:** `UiComponent` uses `#[serde(tag = "type")]` for internally tagged polymorphism.
* **Variants:** `PromptBox(PromptBoxData)`, `TriageBoard(TriageBoardData)`.
* **Struct:** `ComponentBlock { component: UiComponent, children: Vec<Block> }` wraps a component with its nested AST.
* **`required_assets()`** returns `(Vec<&str>, Vec<&str>)` — CSS paths and JS paths declared by each variant. Used by the `AssetRegistry` to collect only the assets actually needed for a given document.
* **Fail-fast:** Unknown types or missing required fields result in a deserialization `Err`, which propagates up and halts compilation.

### `models::base.rs`

* **Responsibility:** Defines the contract between data models and the templating engine.
* **Trait:** `Renderable { fn render(&self, engine: &TemplateEngine) -> String; }`
* **Implementation:** Implemented on `UiComponent` and `ComponentBlock` (which recursively renders children and passes them as a `children` template variable).

### `renderer.rs`

* **Responsibility:** Manages template loading, block rendering, and final document assembly.
* **Struct:** `TemplateEngine` wraps `minijinja::Environment<'static>`.
* **Templates:** Loaded at build time via `include_str!`:
  * `base.html` — Global shell with `{{ content }}`, `{{ title }}`, `{{ inline_styles }}`, `{{ inline_scripts }}`, and layout class.
  * `prompt_box.html` — Component markup with `{% if children %}` conditional slot.
  * `triage_board.html` — Component markup with `{{ children }}` slot for nested content.
* **`render_document`**: Walks blocks, renders components recursively, wraps result in base template.

### `assets.rs`

* **Responsibility:** Collects, deduplicates, resolves, and inlines component CSS and JS.
* **Struct:** `AssetRegistry { stylesheets: BTreeSet<String>, scripts: BTreeSet<String> }`.
* **`from_blocks()`**: Walks the AST, calling `required_assets()` on each component variant.
* **`with_theme()`**: Adds token CSS (e.g., `tokens/clay-slate.css`) if a theme is set in frontmatter.
* **`with_base_assets()`**: Adds the global `css/base.css`.
* **`inline_styles()` / `inline_scripts()`**: Reads each asset file via `include_str!` (resolved in `resolve_asset`), concatenates, and wraps in `<style>` / `<script>` tags.
* **Key Design Decision:** Asset paths are declared per component, resolved at build time, and collected lazily — only the assets needed by the document's components are inlined. This avoids bundling unused CSS/JS.

## 5. Templating & CSS Strategy

The output HTML is fully self-contained. Templates live in `templates/` and are embedded at build time via `include_str!`. CSS and JS assets live separately under `assets/css/`, `assets/js/`, and `assets/tokens/` and are also embedded at build time.

* **base.css** defines `:root` CSS custom properties and global typography. The default layout is a `max-width: 65ch` reading column.
* **Theme tokens** (e.g., `clay-slate.css`) override `:root` variables to customize the color palette. Activated via frontmatter `theme` field.
* **Component CSS** (e.g., `prompt_box.css`, `triage_board.css`) use the CSS variables for colors, spacing, borders, and typography.
* **Component JS** (e.g., `triage_board.js`) provides interactive behavior — the triage board includes full drag-and-drop Kanban logic, tag filtering, and clipboard export, all self-contained in a single `<script>` tag.

## 6. Component Strategy Pattern

Adding a new UI component requires these steps:

1. **Model:** Create a data struct in `src/models/components/`, add it as a variant to `UiComponent`, and declare its `required_assets()`.
2. **Template:** Add a `.html` file in `templates/components/` with a `{{ children }}` slot.
3. **Assets:** Add `.css` (and optionally `.js`) files in `assets/css/` and `assets/js/`, and register the paths in `assets.rs`'s `resolve_asset()` function.
4. **Render:** Implement rendering for the new variant in `ComponentBlock::render()` and `UiComponent::render()`.

## 7. Testing Strategy

* **Unit Tests:** Each module has inline `#[cfg(test)]` tests covering happy paths and error paths:
  * `cli.rs`: Flag parsing, missing arguments, unknown flags.
  * `parser.rs`: Basic prose, frontmatter extraction, single/multiple/nested YAML blocks, unknown types, missing required fields, empty input.
  * `renderer.rs`: Template loading, component rendering, children recursion, prose passthrough, title/theme/layout injection, no external links.
  * `assets.rs`: Empty AST, single/multiple/nested components, deduplication, theme loading, inline content generation.
  * `ui_component.rs`: YAML deserialization of known/unknown types, missing fields, asset declarations.
  * `compiler.rs`: File I/O, missing input errors.
* **Integration Tests:** `tests/integration_test.rs` exercises the full `run_compilation` pipeline against fixture files (`basic.md`, `hybrid.md`), asserting on output HTML content and the absence of raw YAML.
* **Snapshot Tests:** `tests/snapshot_test.rs` uses the `insta` crate to capture the entire rendered HTML output of `basic.md`, `hybrid.md`, and `triage_board.md`. Any layout/Markup/CSS regression fails until the snapshot is explicitly reviewed.

## 8. Key Design Decisions

| Decision | Rationale |
|----------|-----------|
| **Block-based AST** (`Block::Prose` / `Block::Component`) | Preserves document order without placeholder tokens; prose is pre-rendered to HTML at parse time. Enables arbitrary nesting via `children: Vec<Block>`. |
| **YAML frontmatter for document config** | Cleanly separates document-level settings (title, layout, theme) from content. Uses standard `---` delimited blocks compatible with common Markdown tools. |
| **`include_str!` for all templates and assets** | Produces a single static binary with no runtime file I/O for templates or assets. |
| **Internally tagged enum** (`#[serde(tag = "type")]`) | Keeps YAML concise and human-readable; maps cleanly to the Strategy Pattern in Rust. |
| **Self-contained CSS & JS** | Eliminates external asset dependencies; the generated HTML is a single portable file. |
| **Asset registry with BTreeSet deduplication** | Only the assets actually used by the document's components are inlined. Theme tokens are additive on top of the base CSS cascade. |
| **Layout system** (reading-column / wide / canvas) | Allows components like triage-board to override the default reading-width constraint via frontmatter, enabling full-width dashboards within the same compiler. |
| **Fakes over mocks** | Unit tests verify observable output (HTML strings, file contents) rather than internal call sequences. |
