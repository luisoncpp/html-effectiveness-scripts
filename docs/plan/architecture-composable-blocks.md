# Architecture: The Composable Block System

## 1. Core Philosophy

The system transitions from a rigid "Markdown prose + single injected component" model to a **Stream of Blocks** (an Abstract Syntax Tree of content). This embraces the compositional nature of generic layouts, allowing for complex, heterogeneous documents where prose and interactive UI primitives freely interleave and nest.

Instead of defining monolithic templates for specific document types (e.g., `pr-writeup`, `slide-deck`), the system parses inputs into a `Vec<Block>` and resolves their layout and asset dependencies at compile time.

## 2. Data Flow & Pipeline

The compilation process is divided into four distinct phases:

### Phase 1: Lexical Analysis & Parsing
1. **Frontmatter Extraction:** The parser first strips standard YAML frontmatter (`---`) at the top of the file to establish the document's global execution context (theme, base layout wrapper).
2. **Block Tokenization:** `pulldown-cmark` traverses the document.
   - Standard Markdown yields `Block::Prose(String)`.
   - Fenced ```yaml `` blocks yield `Block::Component(UiComponent)`.
3. **AST Construction:** The parser constructs a hierarchical `Vec<Block>`. Component blocks can parse `children` arrays, allowing recursive nesting (e.g., a `BoardLayout` containing `Card` blocks, which in turn contain `Markdown` blocks).

### Phase 2: Asset Registry Resolution
Before any HTML is generated, the compiler traverses the AST to build an **Asset Registry**.
- As the visitor visits each `UiComponent`, it queries the component's required assets.
- For example, if a `CodePanel` and a `BoardLayout` are found, the registry accumulates `["css/diff.css", "css/board.css", "js/drag-and-drop.js"]`.
- This ensures `base.html` only injects exactly the `<link>` and `<script>` tags required by the active blocks, preventing CSS bloat and script collisions.
- All the accumulated registry dependencies are inlined into the final html instead of being included as external references. The output will be a self contained html.

### Phase 3: Template Rendering
1. **Component Rendering:** Each `UiComponent` variant executes its specific MiniJinja template (`templates/components/code_panel.html`, etc.), rendering into raw HTML strings. Nested children are rendered recursively and passed into the parent template via slots.
2. **Base Wrapping:** The aggregated `Vec<Block>` (now a single continuous HTML string) is injected into `base.html`.
3. **Asset Injection:** The Asset Registry writes its accumulated script and stylesheet paths into the `<head>` and at the end of the `<body>`.

## 3. Core Data Structures

```rust
// The fundamental unit of the document AST
pub enum Block {
    Prose(String), // Pre-rendered standard HTML from markdown
    Component(UiComponent),
}

// Global context derived from Frontmatter
pub struct DocumentContext {
    pub title: Option<String>,
    pub layout_wrapper: LayoutType, // e.g., Wide, Canvas, ReadingColumn
    pub theme_tokens: String,       // e.g., "clay-slate", "dark-mode"
}

// The Asset Registry
pub struct AssetRegistry {
    pub stylesheets: HashSet<String>,
    pub scripts: HashSet<String>,
}

```

## 4. Addressing Technical Gaps

* **CSS Splitting:** Handled elegantly by the Asset Registry. Components define their dependencies, and the compiler deduplicates them.
* **JavaScript Emission:** Scripts are handled exactly like CSS. Interactive primitives (like `BoardLayout` for drag-and-drop) request their JS controllers, which are injected dynamically.
* **Theming & Styling:** Handled via Frontmatter. A `theme: "olive"` key injects a specific CSS variable scope (`templates/tokens/olive.css`), applying to all generic UI primitives without altering their structural CSS.

## 5. Implementation Plan

### Phase 1: Lexical Analysis & Parsing

**Goal:** The parser produces a `Vec<Block>` AST with correct `DocumentContext` from any valid input.

**Acceptance Criteria**
1. Standard Markdown text becomes `Block::Prose(String)` in document order.
2. Fenced `yaml` blocks become `Block::Component(UiComponent)` in document order.
3. YAML blocks containing `children: [...]` produce recursively nested `Block` structures.
4. Frontmatter (`---`) at the top of a file populates `DocumentContext` (title, layout wrapper, theme).
5. Missing frontmatter is handled gracefully (defaults).
6. Unknown YAML `type` or missing required fields produce a clear deserialization error that halts compilation.

**Tests to Create**
| Test | Type | What it proves |
|---|---|---|
| `parse_basic_markdown` | Unit | Plain text yields a single `Prose` block. |
| `parse_frontmatter_populates_context` | Unit | Correct `DocumentContext` fields extracted. |
| `parse_missing_frontmatter_defaults` | Unit | Missing keys use safe defaults. |
| `parse_single_yaml_block` | Unit | One fenced block yields one `Component`. |
| `parse_multiple_yaml_blocks` | Unit | Blocks appear in the correct vector order. |
| `parse_nested_children` | Unit | A component with `children` contains nested `Block`s. |
| `parse_unknown_type_fails` | Unit | Deserialization error is returned, not panicked. |
| `parse_hybrid_fixture` | Integration | End-to-end: a real `.md` file produces the expected AST shape. |

**Manual Check**
- Create a test file `test_phase1.md` with frontmatter + prose + a component with children.
- Run a debug CLI flag (or temporary `println!`) to dump the AST.
- Verify the printed AST contains the expected variant sequence and nesting depth.

---

### Phase 2: Asset Registry Resolution

**Goal:** The compiler visits the AST and produces a self-contained HTML string containing all required styles and scripts inline.

**Acceptance Criteria**
1. Visiting a `UiComponent` adds its declared CSS/JS paths to the registry.
2. Nested children are visited; their assets are merged into the same registry.
3. Duplicate asset paths across siblings or ancestors are stored only once.
4. The registry resolves each path to its raw source content at compile time (e.g., via `include_str!`).
5. The final HTML contains zero external `<link href="...">` or `<script src="...">` references.
6. Theme tokens (from frontmatter) are also loaded as inline CSS variables.

**Tests to Create**
| Test | Type | What it proves |
|---|---|---|
| `registry_single_component` | Unit | One component with one CSS file populates the stylesheet set. |
| `registry_nested_merge` | Unit | Parent and child components both contribute; sets are merged. |
| `registry_deduplication` | Unit | The same CSS added twice results in one entry. |
| `registry_empty_ast` | Unit | Empty document yields empty registry. |
| `registry_inline_contents` | Unit | A fake asset map returns the correct raw string for a path. |
| `render_no_external_links` | Integration | Output HTML string contains no `href=` or `src=` pointing to `.css`/`.js`. |
| `render_inline_styles_present` | Integration | Output contains a `<style>` block with the expected CSS substring. |
| `render_inline_scripts_present` | Integration | Output contains a `<script>` block with the expected JS substring. |

**Manual Check**
- Compile a markdown file that uses a component requiring `css/board.css` and `js/drag-and-drop.js`.
- Open the resulting `.html` in a browser.
- Open DevTools Network tab; confirm **0** external requests.
- Confirm the page renders correctly (styles active) and interactive features work (scripts active).
- View page source and verify `<style>` and `<script>` blocks exist with the expected content.

---

### Phase 3: Template Rendering

**Goal:** The AST and registry are transformed into a single, wrapped HTML string matching the target output format.

**Acceptance Criteria**
1. Each `UiComponent` variant dispatches to its dedicated MiniJinja template.
2. Component templates receive their own data fields plus a pre-rendered `children` slot string.
3. `Block::Prose` is passed through as raw HTML.
4. The aggregated block stream is injected into the `base.html` layout wrapper selected by `DocumentContext`.
5. Inline styles are injected into `<head>`; inline scripts are injected at the end of `<body>`.
6. `DocumentContext` fields (title, theme tokens) are available inside `base.html`.
7. The existing `basic.md` and `hybrid.md` fixtures still produce valid output (no regressions).

**Tests to Create**
| Test | Type | What it proves |
|---|---|---|
| `render_prompt_box_produces_html` | Unit | A known component renders its expected HTML substring. |
| `render_parent_with_children_slot` | Unit | Parent template contains the rendered child HTML in the correct slot. |
| `render_base_receives_title` | Unit | `base.html` receives the `DocumentContext.title` variable. |
| `render_base_receives_theme` | Unit | Theme CSS is injected when a theme is specified. |
| `render_prose_passes_through` | Unit | `Block::Prose` raw HTML appears unchanged in output. |
| `render_snapshot_basic` | Snapshot | Output of `basic.md` matches the approved snapshot. |
| `render_snapshot_hybrid` | Snapshot | Output of `hybrid.md` matches the approved snapshot. |
| `render_snapshot_triage_board` | Snapshot | A new fixture mimicking `18-editor-triage-board.html` renders to an equivalent structure. |

**Manual Check**
- Compile `18-editor-triage-board.html`'s equivalent markdown.
- Open in browser.
- **Visual:** Confirm the four-column board layout, sticky toolbar, and card styling match the goal.
- **Interaction:** Drag a ticket from "Now" to "Cut"; confirm the count badge updates.
- **Filter:** Click a "bug" tag; confirm non-bug cards dim.
- **Export:** Click "Copy as markdown"; paste into a text editor and verify the markdown structure and point totals are correct.
- **Responsiveness:** Resize window to < 920px; confirm board collapses to 2 columns.

---

### Cross-Phase Regression Policy

- **Snapshot tests for all existing fixtures must pass** before a phase is considered complete.
- If a snapshot changes, the diff must be reviewed to confirm it is an expected consequence of the new phase, not a regression in unrelated output.
- New fixtures should be added for any new component introduced (e.g., `BoardLayout`, `CardGrid`, `SvgThumbnail`) before the component implementation is merged.