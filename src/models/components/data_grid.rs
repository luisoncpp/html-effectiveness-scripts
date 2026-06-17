use minijinja::{context, Value};
use serde::Deserialize;

use super::ComponentStrategy;

#[derive(Debug, Deserialize, PartialEq)]
pub struct DataGridData {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl ComponentStrategy for DataGridData {
    fn required_assets(&self) -> (Vec<&'static str>, Vec<&'static str>) {
        (vec!["css/data_grid.css"], vec!["js/data_grid.js"])
    }

    fn template_name(&self) -> &'static str {
        "data_grid"
    }

    fn render_context(&self, children_html: &str) -> Value {
        let columns: Vec<String> = self
            .columns
            .iter()
            .map(|c| super::render_markdown_inline(c))
            .collect();
        let rows: Vec<Vec<String>> = self
            .rows
            .iter()
            .map(|row| row.iter().map(|c| super::render_markdown_inline(c)).collect())
            .collect();
        context! {
            columns => columns,
            rows => rows,
            children => children_html,
        }
    }
}
