use anyhow::{Context, Result};
use pulldown_cmark::{html, Event, Parser, Tag, TagEnd, CodeBlockKind};

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

fn parse_body(body: &str) -> Result<Vec<Block>> {
    let parser = Parser::new(body);
    let mut in_yaml = false;
    let mut yaml_buffer = String::new();
    let mut prose_events: Vec<Event> = Vec::new();
    let mut blocks: Vec<Block> = Vec::new();

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang)))
                if lang.as_ref() == "yaml" =>
            {
                flush_prose(&mut prose_events, &mut blocks);
                in_yaml = true;
                yaml_buffer.clear();
            }
            Event::End(TagEnd::CodeBlock) if in_yaml => {
                in_yaml = false;
                let component = parse_component_block(&yaml_buffer)
                    .with_context(|| "Failed to deserialize YAML component")?;
                blocks.push(Block::Component(component));
            }
            Event::Text(text) if in_yaml => {
                yaml_buffer.push_str(&text);
            }
            _ if in_yaml => {}
            other => prose_events.push(other),
        }
    }

    flush_prose(&mut prose_events, &mut blocks);
    Ok(blocks)
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
        .with_context(|| "Failed to parse YAML component")?;

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

    let component: UiComponent = serde_yaml::from_value(value)
        .with_context(|| "Failed to deserialize YAML component")?;

    Ok(ComponentBlock { component, children })
}

fn parse_children(value: serde_yaml::Value) -> Result<Vec<Block>> {
    let arr: Vec<serde_yaml::Value> = serde_yaml::from_value(value)
        .with_context(|| "children must be an array")?;

    arr.into_iter().map(parse_child_value).collect()
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

    anyhow::bail!("Unsupported child block type")
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
}
