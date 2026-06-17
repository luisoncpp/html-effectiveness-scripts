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
        let columns: Vec<BoardColumnView> = self
            .columns
            .iter()
            .map(|col| BoardColumnView {
                title: super::render_markdown_inline(&col.title),
                items: col
                    .items
                    .iter()
                    .map(|item| super::render_markdown_inline(item))
                    .collect(),
            })
            .collect();
        context! {
            variant => &self.variant,
            columns => columns,
            children => children_html,
        }
    }
}

#[derive(Serialize)]
struct BoardColumnView {
    title: String,
    items: Vec<String>,
}
