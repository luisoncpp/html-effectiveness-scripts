
# Implementation Plan & Testing Strategy

This document outlines the phased development approach for the Rust UI Compiler. Each phase includes specific modules to implement, testing strategies, and strict Definition of Done (DoD) criteria.

## Phase 1: Foundation & CLI Skeleton

**Goal:** Establish the project, CLI argument parsing, and standard Markdown-to-HTML passthrough without YAML interception.

* **Target Classes/Modules:** * `cli.rs`: Implement `struct CliArgs` using the `clap` crate (e.g., `--input`, `--output`).
* `compiler.rs`: Implement `fn run_compilation(args: &CliArgs) -> Result<(), Error>`.
* `parser.rs`: Implement basic `pulldown-cmark` parsing to HTML string.


* **Testing Strategy:**
* *Unit Tests:* Verify `cli.rs` correctly parses valid paths and rejects invalid flags.
* *Integration Tests:* Create `tests/fixtures/basic.md`. Assert that `compiler::run_compilation` outputs an HTML file with matching `<p>` and `<h1>` tags.


* **Acceptance Criteria (DoD):**
* Application compiles without warnings (`cargo clippy -- -D warnings`).
* Running `cargo run -- -i test.md -o out.html` successfully transforms standard markdown.
* Unit test coverage for `cli.rs` and file I/O operations is > 80%.


## Phase 2: YAML Interception & Strategy Router

**Goal:** Implement the AST interceptor to extract YAML blocks and route them to strongly-typed Rust structs using `serde`.

* **Target Classes/Modules:**
* `parser.rs`: Refactor to implement the event stream interceptor state machine.
* `models::ui_component.rs`: Define `pub enum UiComponent` with `#[serde(tag = "type")]`.
* `models::components::prompt_box.rs`: Define `struct PromptBoxData { label: String, content: String }`.


* **Testing Strategy:**
* *Unit Tests (`parser.rs`):* Feed a synthetic event stream containing a fenced YAML block. Assert that the block is swallowed and the `yaml_buffer` correctly contains the extracted string.
* *Unit Tests (`models.rs`):* Write string literals of valid YAML. Use `serde_yaml::from_str` and assert that it successfully matches the `UiComponent::PromptBox` variant. Test invalid YAML to ensure proper `Result::Err` propagation.


* **Acceptance Criteria (DoD):**
* The compiler successfully parses a hybrid `.md` file, swallowing the YAML block so it does not appear as raw code in the output HTML.
* Deserialization errors (e.g., missing fields in the YAML) gracefully halt compilation and print a human-readable error specifying the line/struct failure.



## Phase 3: Templating Engine & Final Assembly

**Goal:** Render the deserialized structs into styled HTML using `minijinja` and wrap them in the global design system template.

* **Target Classes/Modules:**
* `renderer.rs`: Implement `struct TemplateEngine`.
* Add a `trait Renderable` in `models/base.rs` requiring `fn render(&self, engine: &TemplateEngine) -> String`.
* Implement `Renderable` for all variants in `UiComponent`.


* **Testing Strategy:**
* *Snapshot Testing:* Use the `insta` crate. Pass a complex hybrid markdown file through the entire pipeline and use `insta::assert_snapshot!` on the resulting HTML string to prevent layout regressions in future updates.
* *Unit Tests (`renderer.rs`):* Ensure `minijinja::Environment` correctly loads local `.html` files or embedded strings and successfully interpolates variables.


* **Acceptance Criteria (DoD):**
* The output HTML is fully self-contained (no external CSS files required; all CSS variables are injected into the `<head>`).
* Snapshot tests pass.
* The Strategy Pattern successfully maps `type: prompt-box` to `<div class="prompt-box">...</div>` embedded organically within the standard Markdown HTML flow.

