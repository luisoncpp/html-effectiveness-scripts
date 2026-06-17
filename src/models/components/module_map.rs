use minijinja::{context, Value};
use serde::{Deserialize, Serialize};

use super::ComponentStrategy;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ModuleMapNode {
    pub id: String,
    pub label: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub class: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ModuleMapEdge {
    pub from: String,
    pub to: String,
    pub d: String,
    pub label: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ModuleMapData {
    pub title: String,
    #[serde(rename = "viewBox")]
    pub view_box: String,
    pub nodes: Vec<ModuleMapNode>,
    pub edges: Vec<ModuleMapEdge>,
}

impl ComponentStrategy for ModuleMapData {
    fn required_assets(&self) -> (Vec<&'static str>, Vec<&'static str>) {
        (vec!["css/module_map.css"], vec![])
    }

    fn template_name(&self) -> &'static str {
        "module_map"
    }

    fn render_context(&self, children_html: &str) -> Value {
        context! {
            title => super::render_markdown_inline(&self.title),
            view_box => &self.view_box,
            nodes => &self.nodes,
            edges => &self.edges,
            children => children_html,
        }
    }
}
