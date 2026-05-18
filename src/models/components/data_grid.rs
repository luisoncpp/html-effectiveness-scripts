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
        context! {
            columns => &self.columns,
            rows => &self.rows,
            children => children_html,
        }
    }
}
