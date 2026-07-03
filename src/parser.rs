use anyhow::{Context, Result};
use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag, TagEnd, html};

use crate::markdown;
use crate::models::block::Block;
use crate::models::document_context::DocumentContext;
use crate::models::ui_component::{ComponentBlock, UiComponent};

pub struct ParsedDocument {
    pub blocks: Vec<Block>,
    pub context: DocumentContext,
}

pub fn parse(input: &str) -> Result<ParsedDocument> {
    let (context, body) = extract_frontmatter(input);
    let blocks = parse_body(body)?;
    Ok(ParsedDocument { blocks, context })
}

fn extract_frontmatter(input: &str) -> (DocumentContext, &str) {
    if !input.starts_with("---") {
        return (DocumentContext::default(), input);
    }

    let after_open = &input[3..];
    let end_idx = after_open
        .find("\n---")
        .or_else(|| after_open.find("\r\n---").map(|i| i + 1));

    if let Some(idx) = end_idx {
        let fm = after_open[..idx].trim();
        let body = &after_open[idx + 4..];
        let body = body.trim_start_matches(['\n', '\r']).trim_start();

        let ctx = if fm.is_empty() {
            DocumentContext::default()
        } else {
            serde_yaml::from_str(fm).unwrap_or_default()
        };
        return (ctx, body);
    }

    (DocumentContext::default(), input)
}

const COMPONENT_FENCE_LANGS: &[&str] = &[
    "notice",
    "card",
    "data-grid",
    "timeline",
    "board-layout",
    "code-panel",
    "code-map",
    "svg-canvas",
    "flowchart",
    "module-map",
    "prompt-box",
    "triage-board",
];

fn component_fence_type(lang: &str) -> Option<&'static str> {
    COMPONENT_FENCE_LANGS
        .iter()
        .copied()
        .find(|&name| name == lang)
}

fn component_yaml_text(fence_type: Option<&str>, body: &str) -> String {
    match fence_type {
        Some(t) => format!("type: {t}\n{body}"),
        None => body.to_string(),
    }
}

fn parse_body(body: &str) -> Result<Vec<Block>> {
    let parser = Parser::new_ext(body, markdown::options()).into_offset_iter();
    let mut in_component_fence = false;
    let mut implicit_fence_type: Option<&'static str> = None;
    let mut yaml_buffer = String::new();
    let mut yaml_start_byte = 0usize;
    let mut block_ordinal = 0usize;
    let mut prose_events: Vec<Event> = Vec::new();
    let mut blocks: Vec<Block> = Vec::new();

    for (event, range) in parser {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                let lang = lang.as_ref();
                let fence_type = if lang == "yaml" {
                    None
                } else {
                    component_fence_type(lang)
                };
                if lang == "yaml" || fence_type.is_some() {
                    flush_prose(&mut prose_events, &mut blocks);
                    in_component_fence = true;
                    implicit_fence_type = fence_type;
                    yaml_buffer.clear();
                    yaml_start_byte = range.start;
                }
            }
            Event::End(TagEnd::CodeBlock) if in_component_fence => {
                in_component_fence = false;
                block_ordinal += 1;
                let line = line_number_at(body, yaml_start_byte);
                let yaml = component_yaml_text(implicit_fence_type, &yaml_buffer);
                implicit_fence_type = None;
                let component = parse_component_block(&yaml).with_context(|| {
                    format!(
                        "Failed to compile YAML component (block #{block_ordinal}, \
                         starting at line {line}):\n{}",
                        indent_snippet(&yaml_buffer)
                    )
                })?;
                blocks.push(Block::Component(component));
            }
            Event::Text(text) if in_component_fence => {
                yaml_buffer.push_str(&text);
            }
            _ if in_component_fence => {}
            other => prose_events.push(other),
        }
    }

    flush_prose(&mut prose_events, &mut blocks);
    Ok(blocks)
}

/// 1-based line number in `source` for the given byte offset.
fn line_number_at(source: &str, byte_offset: usize) -> usize {
    let end = byte_offset.min(source.len());
    source[..end].bytes().filter(|&b| b == b'\n').count() + 1
}

/// Indent each line of a YAML snippet so it reads as a quoted block in errors.
/// Long snippets are truncated to keep error output readable.
fn indent_snippet(yaml: &str) -> String {
    const MAX_LINES: usize = 12;
    let trimmed = yaml.trim_end();
    let mut out: String = trimmed
        .lines()
        .take(MAX_LINES)
        .map(|l| format!("    | {l}\n"))
        .collect();
    if trimmed.lines().count() > MAX_LINES {
        out.push_str("    | ...\n");
    }
    out.trim_end().to_string()
}

fn flush_prose(events: &mut Vec<Event>, blocks: &mut Vec<Block>) {
    if events.is_empty() {
        return;
    }
    let mut html = String::new();
    html::push_html(&mut html, events.drain(..));
    if !html.is_empty() {
        blocks.push(Block::Prose(html));
    }
}

fn parse_component_block(yaml_str: &str) -> Result<ComponentBlock> {
    let mut value: serde_yaml::Value = serde_yaml::from_str(yaml_str)
        .map_err(|e| anyhow::anyhow!("invalid YAML syntax{}: {e}", location_suffix(&e)))?;

    // Surface the declared `type` so the error names the component being compiled.
    let type_name = value
        .as_mapping()
        .and_then(|m| m.get(serde_yaml::Value::String("type".to_string())))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let children = if let Some(mapping) = value.as_mapping_mut() {
        let key = serde_yaml::Value::String("children".to_string());
        if let Some(children_val) = mapping.remove(&key) {
            parse_children(children_val)?
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    let component: UiComponent = serde_yaml::from_value(value).map_err(|e| {
        let what = match &type_name {
            Some(t) => format!("component type \"{t}\""),
            None => "component (no `type` field found)".to_string(),
        };
        anyhow::anyhow!("could not build {what}{}: {e}", location_suffix(&e))
    })?;

    Ok(ComponentBlock {
        component,
        children,
    })
}

/// Format a serde_yaml error's line/column (relative to the block) if available.
fn location_suffix(err: &serde_yaml::Error) -> String {
    match err.location() {
        Some(loc) => format!(
            " at line {} column {} of the block",
            loc.line(),
            loc.column()
        ),
        None => String::new(),
    }
}

fn parse_children(value: serde_yaml::Value) -> Result<Vec<Block>> {
    let arr: Vec<serde_yaml::Value> = serde_yaml::from_value(value)
        .with_context(|| "`children` must be a YAML sequence (a list of `- type: ...` items)")?;

    arr.into_iter()
        .enumerate()
        .map(|(idx, child)| {
            parse_child_value(child).with_context(|| format!("in child #{} of `children`", idx + 1))
        })
        .collect()
}

fn parse_child_value(value: serde_yaml::Value) -> Result<Block> {
    if let Some(mapping) = value.as_mapping() {
        let type_key = serde_yaml::Value::String("type".to_string());
        if mapping.contains_key(&type_key) {
            let yaml_str = serde_yaml::to_string(&value)?;
            let block = parse_component_block(&yaml_str)?;
            return Ok(Block::Component(block));
        }
    }

    anyhow::bail!("child block is missing a `type` field; every nested component needs `type:`")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::components::prompt_box::PromptBoxData;
    use crate::models::components::triage_board::TriageBoardData;
    use crate::models::document_context::LayoutType;

    #[test]
    fn parse_basic_markdown() {
        let result = parse("# Hello").unwrap();
        assert_eq!(result.blocks.len(), 1);
        assert!(matches!(&result.blocks[0], Block::Prose(_)));
        if let Block::Prose(html) = &result.blocks[0] {
            assert!(html.contains("<h1>Hello</h1>"));
        }
    }

    #[test]
    fn parse_markdown_table() {
        let markdown = "| Col A | Col B |\n|-------|-------|\n| one   | two   |";
        let result = parse(markdown).unwrap();
        assert_eq!(result.blocks.len(), 1);
        if let Block::Prose(html) = &result.blocks[0] {
            assert!(html.contains("<table>"));
            assert!(html.contains("<th>Col A</th>"));
            assert!(html.contains("<td>one</td>"));
            assert!(!html.contains('|'));
        } else {
            panic!("Expected prose block");
        }
    }

    #[test]
    fn parse_frontmatter_populates_context() {
        let markdown = r#"---
title: My Document
layout_wrapper: wide
theme_tokens: clay-slate
---
# Hello
"#;
        let result = parse(markdown).unwrap();
        assert_eq!(result.context.title, Some("My Document".to_string()));
        assert_eq!(result.context.layout_wrapper, LayoutType::Wide);
        assert_eq!(result.context.theme_tokens, "clay-slate");
        assert_eq!(result.blocks.len(), 1);
        assert!(matches!(&result.blocks[0], Block::Prose(_)));
    }

    #[test]
    fn parse_missing_frontmatter_defaults() {
        let result = parse("Just text.").unwrap();
        assert_eq!(result.context.title, None);
        assert_eq!(result.context.layout_wrapper, LayoutType::ReadingColumn);
        assert_eq!(result.context.theme_tokens, "");
    }

    #[test]
    fn parse_single_yaml_block() {
        let markdown = r#"```yaml
type: prompt-box
label: Test Label
content: Test Content
```"#;

        let result = parse(markdown).unwrap();
        assert_eq!(result.blocks.len(), 1);
        assert!(matches!(&result.blocks[0], Block::Component(_)));
        if let Block::Component(comp) = &result.blocks[0] {
            assert_eq!(
                comp.component,
                UiComponent::PromptBox(PromptBoxData {
                    label: "Test Label".to_string(),
                    content: "Test Content".to_string(),
                })
            );
            assert!(comp.children.is_empty());
        }
    }

    #[test]
    fn parse_multiple_yaml_blocks() {
        let markdown = r#"```yaml
type: prompt-box
label: First
content: First content
```

```yaml
type: prompt-box
label: Second
content: Second content
```"#;

        let result = parse(markdown).unwrap();
        assert_eq!(result.blocks.len(), 2);
        assert!(matches!(&result.blocks[0], Block::Component(_)));
        assert!(matches!(&result.blocks[1], Block::Component(_)));

        if let Block::Component(first) = &result.blocks[0] {
            assert_eq!(
                first.component,
                UiComponent::PromptBox(PromptBoxData {
                    label: "First".to_string(),
                    content: "First content".to_string(),
                })
            );
        }
        if let Block::Component(second) = &result.blocks[1] {
            assert_eq!(
                second.component,
                UiComponent::PromptBox(PromptBoxData {
                    label: "Second".to_string(),
                    content: "Second content".to_string(),
                })
            );
        }
    }

    #[test]
    fn parse_nested_children() {
        let markdown = r#"```yaml
type: prompt-box
label: Parent
content: Parent content
children:
  - type: prompt-box
    label: Child
    content: Child content
```"#;

        let result = parse(markdown).unwrap();
        assert_eq!(result.blocks.len(), 1);
        let child = match &result.blocks[0] {
            Block::Component(comp) => {
                assert_eq!(
                    comp.component,
                    UiComponent::PromptBox(PromptBoxData {
                        label: "Parent".to_string(),
                        content: "Parent content".to_string(),
                    })
                );
                assert_eq!(comp.children.len(), 1);
                match &comp.children[0] {
                    Block::Component(child_comp) => child_comp,
                    _ => panic!("Expected nested component"),
                }
            }
            _ => panic!("Expected component block"),
        };
        assert_eq!(
            child.component,
            UiComponent::PromptBox(PromptBoxData {
                label: "Child".to_string(),
                content: "Child content".to_string(),
            })
        );
        assert!(child.children.is_empty());
    }

    #[test]
    fn parse_deep_nesting() {
        let markdown = r#"```yaml
type: prompt-box
label: Grandparent
content: A
children:
  - type: prompt-box
    label: Parent
    content: B
    children:
      - type: prompt-box
        label: Child
        content: C
```"#;

        let result = parse(markdown).unwrap();
        assert_eq!(result.blocks.len(), 1);
        let grandparent = match &result.blocks[0] {
            Block::Component(comp) => comp,
            _ => panic!("Expected component block"),
        };
        assert_eq!(grandparent.children.len(), 1);

        let parent = match &grandparent.children[0] {
            Block::Component(comp) => comp,
            _ => panic!("Expected nested component"),
        };
        assert_eq!(
            parent.component,
            UiComponent::PromptBox(PromptBoxData {
                label: "Parent".to_string(),
                content: "B".to_string(),
            })
        );
        assert_eq!(parent.children.len(), 1);

        let child = match &parent.children[0] {
            Block::Component(comp) => comp,
            _ => panic!("Expected grandchild component"),
        };
        assert_eq!(
            child.component,
            UiComponent::PromptBox(PromptBoxData {
                label: "Child".to_string(),
                content: "C".to_string(),
            })
        );
        assert!(child.children.is_empty());
    }

    #[test]
    fn parse_cross_component_nesting() {
        let markdown = r#"```yaml
type: triage-board
eyebrow: Sprint 1
title: Board
subtitle: Tasks
hintline: Drag items
children:
  - type: prompt-box
    label: Note
    content: Something to remember
```"#;

        let result = parse(markdown).unwrap();
        assert_eq!(result.blocks.len(), 1);
        let board = match &result.blocks[0] {
            Block::Component(comp) => comp,
            _ => panic!("Expected component block"),
        };
        assert_eq!(
            board.component,
            UiComponent::TriageBoard(TriageBoardData {
                eyebrow: "Sprint 1".to_string(),
                title: "Board".to_string(),
                subtitle: "Tasks".to_string(),
                hintline: "Drag items".to_string(),
            })
        );
        assert_eq!(board.children.len(), 1);

        let child = match &board.children[0] {
            Block::Component(comp) => comp,
            _ => panic!("Expected nested component"),
        };
        assert_eq!(
            child.component,
            UiComponent::PromptBox(PromptBoxData {
                label: "Note".to_string(),
                content: "Something to remember".to_string(),
            })
        );
        assert!(child.children.is_empty());
    }

    #[test]
    fn parse_unknown_type_fails() {
        let markdown = r#"```yaml
type: unknown-component
label: Test
```"#;

        let result = parse(markdown);
        assert!(result.is_err());
    }

    #[test]
    fn parse_hybrid_fixture() {
        let markdown = std::fs::read_to_string("tests/fixtures/hybrid.md").unwrap();
        let result = parse(&markdown).unwrap();

        assert_eq!(result.context, DocumentContext::default());

        let comp = result
            .blocks
            .iter()
            .find_map(|b| match b {
                Block::Component(c) => Some(c),
                _ => None,
            })
            .expect("Expected a component block");

        assert_eq!(
            comp.component,
            UiComponent::PromptBox(PromptBoxData {
                label: "My Prompt".to_string(),
                content: "This is prompt content.".to_string(),
            })
        );

        let prose_count = result
            .blocks
            .iter()
            .filter(|b| matches!(b, Block::Prose(_)))
            .count();
        assert!(prose_count >= 2);
    }

    #[test]
    fn empty_input_produces_empty_blocks() {
        let result = parse("").unwrap();
        assert!(result.blocks.is_empty());
        assert_eq!(result.context, DocumentContext::default());
    }

    #[test]
    fn returns_error_for_missing_required_field() {
        let markdown = r#"```yaml
type: prompt-box
label: Test Label
```"#;

        let result = parse(markdown);
        assert!(result.is_err());
    }

    #[test]
    fn parse_hybrid_fixture_with_frontmatter_and_children() {
        let markdown = std::fs::read_to_string("tests/fixtures/test_phase1.md").unwrap();
        let result = parse(&markdown).unwrap();

        assert_eq!(result.context.title, Some("Phase 1 Test".to_string()));
        assert_eq!(result.context.layout_wrapper, LayoutType::Wide);
        assert_eq!(result.context.theme_tokens, "clay-slate");
        assert!(result.blocks.len() >= 2);

        let comp = result
            .blocks
            .iter()
            .find_map(|b| match b {
                Block::Component(c) => Some(c),
                _ => None,
            })
            .expect("Expected a component block");
        assert_eq!(comp.children.len(), 1);
    }

    // --- Error message tests ---------------------------------------------

    /// Render the full anyhow cause chain into one string, the way `main` prints it.
    fn error_chain(err: anyhow::Error) -> String {
        let mut s = format!("{err}");
        for cause in err.chain().skip(1) {
            s.push_str(&format!("\n{cause}"));
        }
        s
    }

    #[test]
    fn error_reports_block_number_and_source_line() {
        // The first block is valid; the bad second block starts on line 8 of the body.
        let markdown = r#"# Heading

```yaml
type: notice
variant: info
content: ok
```

```yaml
type: notice
icon alert
content: broken
```"#;

        let err = parse(markdown).err().unwrap();
        let msg = error_chain(err);
        assert!(msg.contains("block #2"), "missing block ordinal: {msg}");
        assert!(msg.contains("line 9"), "wrong/missing source line: {msg}");
    }

    #[test]
    fn error_includes_yaml_snippet() {
        let markdown = r#"```yaml
type: notice
icon alert
content: broken
```"#;

        let msg = error_chain(parse(markdown).err().unwrap());
        assert!(msg.contains("| type: notice"), "snippet not shown: {msg}");
        assert!(msg.contains("| icon alert"), "snippet not shown: {msg}");
    }

    #[test]
    fn error_reports_yaml_syntax_location() {
        let markdown = r#"```yaml
type: notice
icon alert
content: broken
```"#;

        let msg = error_chain(parse(markdown).err().unwrap());
        assert!(
            msg.contains("invalid YAML syntax"),
            "missing syntax label: {msg}"
        );
        assert!(msg.contains("of the block"), "missing location: {msg}");
    }

    #[test]
    fn error_names_unknown_component_type() {
        let markdown = r#"```yaml
type: bogus-component
label: x
```"#;

        let msg = error_chain(parse(markdown).err().unwrap());
        assert!(
            msg.contains("component type \"bogus-component\""),
            "type not named: {msg}"
        );
        // The serde error should list the valid variants.
        assert!(msg.contains("notice"), "valid variants not listed: {msg}");
    }

    #[test]
    fn error_reports_missing_required_field_with_type() {
        let markdown = r#"```yaml
type: prompt-box
label: Only a label
```"#;

        let msg = error_chain(parse(markdown).err().unwrap());
        assert!(
            msg.contains("component type \"prompt-box\""),
            "type not named: {msg}"
        );
    }

    #[test]
    fn error_reports_child_index_and_missing_type() {
        let markdown = r#"```yaml
type: prompt-box
label: Parent
content: body
children:
  - type: prompt-box
    label: ok child
    content: c
  - label: no type here
```"#;

        let msg = error_chain(parse(markdown).err().unwrap());
        assert!(msg.contains("child #2"), "child index not reported: {msg}");
        assert!(
            msg.contains("missing a `type` field"),
            "missing-type message absent: {msg}"
        );
    }

    #[test]
    fn error_reports_children_not_a_sequence() {
        let markdown = r#"```yaml
type: prompt-box
label: Parent
content: body
children: not-a-list
```"#;

        let msg = error_chain(parse(markdown).err().unwrap());
        assert!(
            msg.contains("`children` must be a YAML sequence"),
            "children-not-sequence message absent: {msg}"
        );
    }

    #[test]
    fn line_number_at_counts_newlines() {
        let src = "a\nb\nc";
        assert_eq!(line_number_at(src, 0), 1);
        assert_eq!(line_number_at(src, 2), 2);
        assert_eq!(line_number_at(src, 4), 3);
        // Out-of-range offset is clamped, not a panic.
        assert_eq!(line_number_at(src, 999), 3);
    }

    #[test]
    fn indent_snippet_truncates_long_blocks() {
        let yaml: String = (0..20).map(|i| format!("line{i}\n")).collect();
        let out = indent_snippet(&yaml);
        assert!(out.contains("| line0"));
        assert!(out.contains("| ..."), "long snippet not truncated: {out}");
        assert!(!out.contains("line19"), "truncation failed: {out}");
    }

    #[test]
    fn parse_primitive_fence_compiles_component() {
        let markdown = r#"```card
title: Fence Card
content: Body text
```"#;

        let result = parse(markdown).unwrap();
        assert_eq!(result.blocks.len(), 1);
        assert!(matches!(&result.blocks[0], Block::Component(_)));
    }

    #[test]
    fn parse_primitive_fence_yaml_syntax_error_fails() {
        let markdown = r#"```card
broken: [unclosed
```"#;

        let err = parse(markdown).err().unwrap();
        let msg = error_chain(err);
        assert!(msg.contains("block #1"), "missing block ordinal: {msg}");
        assert!(
            msg.contains("invalid YAML syntax"),
            "missing syntax label: {msg}"
        );
    }

    #[test]
    fn parse_primitive_fence_schema_error_fails() {
        let markdown = r#"```prompt-box
label: Only label
```"#;

        let err = parse(markdown).err().unwrap();
        let msg = error_chain(err);
        assert!(
            msg.contains("component type \"prompt-box\""),
            "type not named: {msg}"
        );
    }

    #[test]
    fn non_component_fence_stays_prose() {
        let markdown = r#"```rust
fn main() {}
```"#;

        let result = parse(markdown).unwrap();
        assert_eq!(result.blocks.len(), 1);
        assert!(matches!(&result.blocks[0], Block::Prose(_)));
    }
}
