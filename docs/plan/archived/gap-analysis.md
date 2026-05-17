# Gap Analysis: Missing Parts to Reach the Output Goal (20 Files)

> **Scope correction:** The `output_goal/` directory contains **20 distinct HTML archetypes**, not the 2 components shown in `docs/example.md`. The current system supports only `prompt-box`. This document inventories every missing layer required to generate all 20 output files from hybrid Markdown + YAML input.

---

## 1. The Real Gap: Page Archetypes, Not Just Components

The current architecture treats every input as "generic Markdown prose + a few injected components." The 20 output files are **fundamentally different document types**, each with its own full-page layout, CSS variables, typography rules, and often substantial JavaScript. A single `base.html` with one color palette cannot produce:

- Slide decks with `scroll-snap-type: y mandatory`
- Interactive drag-and-drop triage boards
- Animation sandboxes with runtime easing switching
- Design-system reference pages with color swatches and type scales

**Missing architectural concept:** A **Document Archetype Router** that selects the correct base layout, CSS token set, and JS bundle based on a top-level YAML declaration or file convention.

---

## 2. Twenty Document Archetypes to Support

Each row maps an `output_goal/` file to the structural capabilities it requires that the current system lacks.

| # | File | Archetype | What's Missing |
|---|------|-----------|----------------|
| 01 | `exploration-code-approaches` | **Approach Comparison** | 3-column grid layout, code panels with syntax highlighting, tradeoff tables, chip/badge rows, recommendation callout box |
| 02 | `exploration-visual-designs` | **Visual Design Exploration** | Sticky toolbar, segmented controls (light/dark toggle), artboard grid, scoped CSS variants per artboard, inline SVG mockups |
| 03 | `code-review-pr` | **Annotated PR Review** | Risk-map chips with color coding, file cards with diff blocks, line-numbered diffs, review comment bubbles, collapsible file summaries, checklists |
| 04 | `code-understanding` | **Module Map / Architecture Note** | SVG flow diagrams with boxes and arrows, step-by-step walkthrough with numbered badges, collapsible code snippets, sticky sidebar with "Key files" and "Gotchas" |
| 05 | `design-system` | **Living Design System** | Color swatch grids, typography scale ruler, spacing token bars, radius/elevation cards, live component contact sheets (buttons, inputs, checkboxes, badges) |
| 06 | `component-variants` | **Component Variant Matrix** | Interactive toolbar (sliders, radio groups, checkboxes), responsive card grid, variant-scoped CSS classes, snippet preview panel |
| 07 | `prototype-animation` | **Animation Sandbox** | Interactive stage with CSS transitions, easing switcher panel, keyframe timeline visualization, copy-paste CSS output panel |
| 08 | `prototype-interaction` | **Clickable Flow Prototype** | Draggable list items with dragover indicators, annotation panels, open-question lists |
| 09 | `slide-deck` | **Slide Deck** | Full-viewport `scroll-snap` sections, slide counter, keyboard navigation (arrow keys), inverted slide themes, metric big-number layouts, sparkline charts |
| 10 | `svg-illustrations` | **SVG Illustration Sheet** | Inline `<svg>` figures with `<marker>` arrows, `<text>` labels, canvas containers with download buttons, palette swatch reference, usage notes |
| 11 | `status-report` | **Weekly Status Report** | Summary stat-card band (4-up grid), highlight lists with custom bullets, shipped-item table with risk dots, inline SVG bar chart, carryover panel |
| 12 | `incident-report` | **Incident Timeline** | Colored severity pills, dark TL;DR panel, vertical timeline with dots, code diff panel with +/- lines, impact table, action-item rows with avatars/checkboxes, fixed TOC |
| 13 | `flowchart-diagram` | **Annotated Flowchart** | SVG nodes (rect, diamond, terminal), edges with arrow markers, click-to-reveal side panel, legend with shape chips |
| 14 | `research-feature-explainer` | **Feature Explainer** | Sticky side nav, collapsible `<details>` blocks, tabbed code panels, callout boxes, definition lists (FAQ) |
| 15 | `research-concept-explainer` | **Concept Explainer** | Interactive SVG demo (consistent-hashing ring), range sliders that mutate SVG, comparison table, hover-linked glossary sidebar |
| 16 | `implementation-plan` | **Implementation Plan** | Numbered section headers, summary strip, milestone timeline with dots/lines, SVG data-flow diagram, mockup panels (comment threads, digests), side-by-side code blocks, risk table with severity badges, open-question cards |
| 17 | `pr-writeup` | **PR Writeup** | PR meta row (files, +/- stats, branch, author), TL;DR box, before/after comparison panels, file tour with collapsible `<details>` and code blocks, numbered review-focus items, test checklist, rollout timeline steps, sticky TOC |
| 18 | `editor-triage-board` | **Ticket Triage Board** | Kanban columns with drag-and-drop (HTML5 DnD API), ticket cards with tags/estimates/owners, toolbar with filter badge and export button, tag filtering with dimming, markdown export |
| 19 | `editor-feature-flags` | **Feature Flag Editor** | Grouped toggle switches with dependency warnings, banner alerts, sticky sidebar with diff preview and copy buttons, JSON export |
| 20 | `editor-prompt-tuner` | **Prompt Tuner** | Contenteditable template editor with regex-based slot highlighting, live preview panels (3 samples), token counter, clipboard copy, reset button |

---

## 3. Cross-Cutting Missing Capabilities

### 3.1 JavaScript Emission

**Current state:** The compiler emits zero JavaScript.  
**Required:** 14 of the 20 archetypes require non-trivial inline `<script>` for interactivity:

- Tab switching (feature explainer)
- Drag-and-drop (triage board, interaction prototype)
- Live preview / slot filling (prompt tuner)
- Toggle / filter state (feature flags, component variants)
- Clipboard copy (illustration sheet, prompt tuner, triage board, feature flags)
- Scroll-snapping + keyboard nav (slide deck)
- SVG mutation from sliders (concept explainer)
- Node click handlers with side-panel updates (flowchart)
- Easing switchers with CSS variable updates (animation sandbox)

**Gap:** No mechanism in `compiler.rs` → `renderer.rs` → `base.html` to inject per-document JavaScript.

### 3.2 SVG Generation & Embedding

**Current state:** No SVG support whatsoever.  
**Required:** At least 5 archetypes rely heavily on inline SVG:

- Module map diagrams (04)
- SVG illustration figures (10)
- Inline bar charts (11)
- Flowcharts with nodes and edges (13)
- Interactive demos (15)

**Gap:** No `SvgComponent` or template helpers for generating `<svg>`, `<rect>`, `<path>`, `<marker>`, `<text>` elements from YAML data.

### 3.3 Syntax-Highlighted Code Blocks with Diff Annotations

**Current state:** Standard Markdown `<pre><code>` passthrough.  
**Required:** Many archetypes need:

- Inline syntax highlighting (keywords, strings, comments, functions in distinct colors)
- Diff annotations (`+` / `-` lines with background colors)
- Line-numbered diffs with gutter marks
- File path headers above code panels

**Gap:** No code-highlighting pipeline (e.g., `syntect` integration) and no `diff` code-block handler.

### 3.4 Tables with Rich Cell Content

**Current state:** Basic Markdown table → `<table>`.  
**Required:**

- Risk tables with colored severity badges inside cells
- Impact tables with monospace values
- Shipped-item tables with PR links, author names, and colored risk dots
- Comparison tables with `good`/`bad` cell styling

**Gap:** No table cell renderer that can inject HTML components (badges, dots, links) from YAML-structured data.

### 3.5 Design System Token System

**Current state:** `base.html` has generic blue/gray CSS variables.  
**Required:** The 20 files share a coherent palette (ivory, slate, clay, olive, oat, rust, gray-150/300/500/700) with serif/sans/mono font stacks. Each archetype may need additional tokens (shadows, radii, spacing scale).

**Gap:** The base template system needs per-archetype token injection, or a shared token file that archetypes can reference/extend.

### 3.6 Component Composition & Nesting

**Current state:** `UiComponent` enum routes to a single flat template per variant.  
**Required:** Complex archetypes like `implementation-plan` and `pr-writeup` are compositions of many sub-patterns:

- Milestones + Data-flow diagram + Mockups + Code blocks + Risk table + Open questions

**Gap:** No sub-component system. Each `UiComponent` variant currently maps 1:1 to one `.html` template. We need composite components that can render child components recursively.

---

## 4. New Component Inventory Needed

Based on the 20 archetypes, the following **new YAML component types** would need to be defined:

```
approach-comparison      # 01: side-by-side cards with code + tradeoffs
visual-exploration       # 02: artboards with scoped CSS variants
annotated-pr             # 03: risk map + file cards + diffs + comments
module-map               # 04: SVG diagram + step walkthrough + sidebar
design-system            # 05: swatches + type scale + spacing + component stage
variant-matrix           # 06: interactive grid with toolbar controls
animation-sandbox        # 07: stage + easing panel + keyframe timeline
interaction-prototype      # 08: draggable lists + annotation panels
slide-deck               # 09: scroll-snap slides with metrics/svg-list
svg-sheet                # 10: inline SVG figures with download buttons
status-report            # 11: stat band + table + SVG chart + carryover
incident-report          # 12: timeline + diff + impact table + action items
flowchart                # 13: SVG nodes/edges with click-to-reveal panel
feature-explainer        # 14: collapsible steps + tabs + callouts + FAQ
concept-explainer        # 15: interactive SVG demo + comparison table + glossary
implementation-plan      # 16: milestones + diagram + mockups + code + risks + questions
pr-writeup               # 17: meta + tldr + file tour + focus + tests + rollout
triage-board             # 18: kanban columns with DnD + filter + export
feature-flags            # 19: grouped toggles + warnings + diff + JSON export
prompt-tuner             # 20: contenteditable + live preview + slot highlighting
```

> Note: Some of these are **page-level** archetypes that would define the entire document structure, not just an inline component.

---

## 5. Renderer & Template Gaps

### 5.1 MiniJinja Limitations

The current `TemplateEngine` wraps MiniJinja with static `include_str!` templates. For the 20 archetypes, we need:

- **Template inheritance / extension:** `base.html` should be archetype-specific.
- **Macro libraries:** Shared patterns (pills, badges, code blocks, avatars) should be reusable macros, not duplicated per template.
- **Raw HTML passthrough:** SVG markup and `<script>` blocks must not be escaped.

### 5.2 Template Registration Explosion

`TemplateEngine::new()` currently registers 2 templates. To support 20 archetypes + shared macros, we would need ~25–30 template files. The current manual registration pattern does not scale.

**Gap:** No dynamic template discovery or directory-walking registration.

---

## 6. Parser Gaps

### 6.1 Multi-Block Documents

The current parser intercepts a single fenced `yaml` block. Several archetypes (especially 16, 17) would logically contain **multiple** YAML blocks of different types in one document:

```markdown
# Implementation Plan

```yaml
type: implementation-plan
...
```

## Milestones

```yaml
type: milestone-timeline
phases: [...]
```

## Data Flow

```yaml
type: svg-diagram
boxes: [...]
```
```

**Gap:** The parser supports multiple YAML blocks technically (each gets a placeholder), but the `UiComponent` enum and `render_document` logic are not designed for heterogeneous multi-component documents with page-level orchestration.

### 6.2 Frontmatter Support

Many archetypes need document-level metadata (title, author, date, repo, branch) that should live in YAML frontmatter at the top of the `.md` file, not inside a fenced block.

**Gap:** No frontmatter parser (`---
... 
---`) to extract page-level metadata before the Markdown body is processed.

---

## 7. CSS Strategy Gap

### 7.1 Per-Archetype Stylesheets

The current `base.html` embeds all CSS for `prompt-box` and basic Markdown. The 20 archetypes together would require **~3,000+ lines** of CSS. Embedding all of it in one `base.html` is unmaintainable and produces bloated output for simple documents.

**Gap:** No CSS code-splitting. The architecture needs either:
- Per-archetype CSS injection (only include the CSS needed for that archetype)
- Or a shared design-system CSS file + archetype-specific overrides

### 7.2 Interactive CSS States

Many archetypes rely on CSS for interactivity without JS (e.g., `<details>` elements, `:hover` states, checkbox hacks). The current base template has none of these utilities.

---

## 8. Testing Gaps

### 8.1 Fixture Coverage

The current test suite has 2 fixtures (`basic.md`, `hybrid.md`). To cover the 20 archetypes meaningfully, we would need:

- 20 new fixture files (one per archetype)
- 20 snapshot tests with `insta`
- Integration tests asserting on interactivity (e.g., "script tag is present", "SVG contains N nodes")

### 8.2 JavaScript Testing

**Gap:** No infrastructure for testing emitted JavaScript (even basic smoke tests like "script parses without syntax errors").

---

## 9. Summary Checklist of Missing Parts

| # | Missing Part | Scope |
|---|--------------|-------|
| 1 | **Document Archetype Router** — selects layout, tokens, JS per document type | Architecture |
| 2 | **20 new YAML component/page types** | `models::components/*`, `models::ui_component.rs` |
| 3 | **20+ new MiniJinja templates** | `templates/archetypes/*.html` |
| 4 | **JavaScript emission pipeline** — inject `<script>` into final HTML | `renderer.rs`, `base.html` |
| 5 | **SVG generation helpers / raw passthrough** | Templates, parser |
| 6 | **Syntax-highlighted code blocks + diff annotations** | Parser or post-processor |
| 7 | **Rich table renderer** — badges, dots, links inside cells | Template macros |
| 8 | **Shared design-system token CSS** — ivory/slate/clay/olive palette | `templates/tokens.css` or per-archetype CSS |
| 9 | **Template macro library** — reusable pills, avatars, badges, code panels | `templates/macros.html` |
| 10 | **Frontmatter parser** for document-level metadata | `parser.rs` |
| 11 | **Multi-component orchestration** — page-level composition | `compiler.rs`, `renderer.rs` |
| 12 | **Per-archetype CSS code-splitting** | `renderer.rs`, templates |
| 13 | **20 new fixture + snapshot tests** | `tests/fixtures/`, `tests/snapshot_test.rs` |
| 14 | **JavaScript smoke-test infrastructure** | `tests/` |
| 15 | **Dynamic template discovery** instead of manual `env.add_template` | `renderer.rs` |
| 16 | **Component nesting / sub-component rendering** | `models::base.rs`, templates |

---

## 10. Architectural Decision Needed

Before implementing, a critical choice must be made:

> **Option A — Expand the SSG:** Keep the micro-SSG model, add 20 component variants, frontmatter, JS injection, and SVG support. This keeps the "Markdown + YAML → HTML" philosophy but requires massive template growth.

> **Option B — Page Archetype Model:** Recognize that many of the 20 files are **not** "Markdown with components" but rather **standalone document types** that happen to be authored in YAML. Restructure the compiler to route the entire document through an archetype-specific pipeline (layout + CSS + JS + template), where standard Markdown is just one optional section within the archetype.

**Recommendation:** Option B. The `exploration-code-approaches`, `editor-triage-board`, and `slide-deck` archetypes make little sense as "Markdown prose with a injected component." They are full applications in a single file. The architecture should support a top-level `type: slide-deck` that takes over the entire page rendering, not just a paragraph slot.
