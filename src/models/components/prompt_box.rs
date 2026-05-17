use minijinja::{context, Value};
use serde::Deserialize;

use super::ComponentStrategy;

#[derive(Debug, Deserialize, PartialEq)]
pub struct PromptBoxData {
    pub label: String,
    pub content: String,
}

impl ComponentStrategy for PromptBoxData {
    fn required_assets(&self) -> (Vec<&'static str>, Vec<&'static str>) {
        (vec!["css/prompt_box.css"], vec![])
    }

    fn template_name(&self) -> &'static str {
        "prompt_box"
    }

    fn render_context(&self, children_html: &str) -> Value {
        context! {
            label => &self.label,
            content => &self.content,
            children => children_html,
        }
    }
}
