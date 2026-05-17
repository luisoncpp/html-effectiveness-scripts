use serde::Deserialize;

use super::base::Renderable;
use super::block::Block;
use super::components::prompt_box::PromptBoxData;
use crate::renderer::TemplateEngine;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum UiComponent {
    #[serde(rename = "prompt-box")]
    PromptBox(PromptBoxData),
}

#[derive(Debug, PartialEq)]
pub struct ComponentBlock {
    pub component: UiComponent,
    pub children: Vec<Block>,
}

impl UiComponent {
    pub fn required_assets(&self) -> (Vec<&'static str>, Vec<&'static str>) {
        match self {
            UiComponent::PromptBox(_) => (vec!["css/prompt_box.css"], vec![]),
        }
    }
}

impl ComponentBlock {
    pub fn render(&self, engine: &TemplateEngine) -> String {
        let children_html: String = self
            .children
            .iter()
            .map(|child| match child {
                Block::Prose(html) => html.clone(),
                Block::Component(comp) => comp.render(engine),
            })
            .collect();

        match &self.component {
            UiComponent::PromptBox(data) => {
                let ctx = minijinja::context! {
                    label => &data.label,
                    content => &data.content,
                    children => children_html,
                };
                engine
                    .render("prompt_box", ctx)
                    .unwrap_or_else(|e| format!("<!-- render error: {} -->", e))
            }
        }
    }
}

impl Renderable for UiComponent {
    fn render(&self, engine: &TemplateEngine) -> String {
        match self {
            UiComponent::PromptBox(data) => {
                let ctx = minijinja::context! {
                    label => &data.label,
                    content => &data.content,
                };
                engine
                    .render("prompt_box", ctx)
                    .unwrap_or_else(|e| format!("<!-- render error: {} -->", e))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_prompt_box_from_valid_yaml() {
        let yaml = r#"
type: prompt-box
label: Test Label
content: Test Content
"#;
        let result: Result<UiComponent, _> = serde_yaml::from_str(yaml);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            UiComponent::PromptBox(PromptBoxData {
                label: "Test Label".to_string(),
                content: "Test Content".to_string(),
            })
        );
    }

    #[test]
    fn returns_error_for_unknown_type() {
        let yaml = r#"
type: unknown-component
label: Test
"#;
        let result: Result<UiComponent, _> = serde_yaml::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn returns_error_for_missing_required_fields() {
        let yaml = r#"
type: prompt-box
label: Test Label
"#;
        let result: Result<UiComponent, _> = serde_yaml::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn prompt_box_declares_required_assets() {
        let comp = UiComponent::PromptBox(PromptBoxData {
            label: "Test".to_string(),
            content: "Content".to_string(),
        });
        let (css, js) = comp.required_assets();
        assert_eq!(css, vec!["css/prompt_box.css"]);
        assert!(js.is_empty());
    }
}
