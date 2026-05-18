use minijinja::{context, Value};
use serde::{Deserialize, Serialize};

use super::ComponentStrategy;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BoardColumn {
    pub title: String,
    #[serde(default)]
    pub items: Vec<String>,
}

fn default_variant() -> String {
    "kanban".to_string()
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BoardLayoutData {
    #[serde(default = "default_variant")]
    pub variant: String,
    pub columns: Vec<BoardColumn>,
}

impl ComponentStrategy for BoardLayoutData {
    fn required_assets(&self) -> (Vec<&'static str>, Vec<&'static str>) {
        (vec!["css/board_layout.css"], vec![])
    }

    fn template_name(&self) -> &'static str {
        "board_layout"
    }

    fn render_context(&self, children_html: &str) -> Value {
        context! {
            variant => &self.variant,
            columns => &self.columns,
            children => children_html,
        }
    }
}
