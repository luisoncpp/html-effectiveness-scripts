use anyhow::{Context, Result};
use pulldown_cmark::{html, Event, Parser, Tag, TagEnd, CodeBlockKind};

use crate::models::ui_component::UiComponent;

pub struct ParsedDocument {
    pub html: String,
    pub components: Vec<UiComponent>,
}

pub fn parse(markdown: &str) -> Result<ParsedDocument> {
    let parser = Parser::new(markdown);
    let mut in_yaml = false;
    let mut yaml_buffer = String::new();
    let mut components = Vec::new();
    let mut events = Vec::new();

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) if lang.as_ref() == "yaml" => {
                in_yaml = true;
                yaml_buffer.clear();
            }
            Event::End(TagEnd::CodeBlock) if in_yaml => {
                in_yaml = false;
                let component = serde_yaml::from_str(&yaml_buffer)
                    .with_context(|| "Failed to deserialize YAML component")?;
                let index = components.len();
                components.push(component);
                events.push(Event::Html(
                    format!("<!-- COMPONENT_PLACEHOLDER_{} -->", index).into(),
                ));
            }
            Event::Text(text) if in_yaml => {
                yaml_buffer.push_str(&text);
            }
            _ if in_yaml => {}
            other => events.push(other),
        }
    }

    let mut html = String::new();
    html::push_html(&mut html, events.into_iter());

    Ok(ParsedDocument { html, components })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::components::prompt_box::PromptBoxData;

    #[test]
    fn converts_h1_to_html() {
        let result = parse("# Hello").unwrap();
        assert!(result.html.contains("<h1>Hello</h1>"));
        assert!(result.components.is_empty());
    }

    #[test]
    fn converts_paragraph_to_html() {
        let result = parse("Some text.").unwrap();
        assert!(result.html.contains("<p>Some text.</p>"));
    }

    #[test]
    fn empty_input_produces_empty_output() {
        let result = parse("").unwrap();
        assert_eq!(result.html, "");
    }

    #[test]
    fn swallows_yaml_block_and_extracts_component() {
        let markdown = r#"# Title

```yaml
type: prompt-box
label: Test Label
content: Test Content
```

Some paragraph.
"#;

        let result = parse(markdown).unwrap();
        assert!(!result.html.contains("<code"));
        assert!(!result.html.contains("type: prompt-box"));
        assert!(result.html.contains("<h1>Title</h1>"));
        assert!(result.html.contains("<p>Some paragraph.</p>"));

        assert_eq!(result.components.len(), 1);
        assert_eq!(
            result.components[0],
            UiComponent::PromptBox(PromptBoxData {
                label: "Test Label".to_string(),
                content: "Test Content".to_string(),
            })
        );
    }

    #[test]
    fn returns_error_for_invalid_component_type() {
        let markdown = r#"```yaml
type: unknown-component
label: Test
```"#;

        let result = parse(markdown);
        assert!(result.is_err());
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
}
