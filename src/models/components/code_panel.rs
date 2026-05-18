use minijinja::{context, Value};
use serde::{Deserialize, Serialize};

use super::ComponentStrategy;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CodeTab {
    pub name: String,
    pub language: String,
    #[serde(default)]
    pub diff: bool,
    pub content: String,
    pub risk: Option<String>,
    pub added: Option<u32>,
    pub removed: Option<u32>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CodePanelData {
    pub tabs: Vec<CodeTab>,
}

impl ComponentStrategy for CodePanelData {
    fn required_assets(&self) -> (Vec<&'static str>, Vec<&'static str>) {
        (vec!["css/code_panel.css"], vec!["js/tabs.js"])
    }

    fn template_name(&self) -> &'static str {
        "code_panel"
    }

    fn render_context(&self, children_html: &str) -> Value {
        context! {
            tabs => &self.tabs,
            children => children_html,
        }
    }
}
