use minijinja::{Value, context};
use serde::{Deserialize, Serialize};

use super::ComponentStrategy;
use super::svg_path::{LabelPosition, edge_label_position};

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
    #[serde(default)]
    pub label_x: Option<i32>,
    #[serde(default)]
    pub label_y: Option<i32>,
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
        let edges: Vec<ModuleMapEdgeView> = self
            .edges
            .iter()
            .map(|e| ModuleMapEdgeView::from_edge(e, &self.view_box))
            .collect();
        context! {
            title => super::render_markdown_inline(&self.title),
            view_box => &self.view_box,
            nodes => &self.nodes,
            edges => edges,
            children => children_html,
        }
    }
}

#[derive(Serialize)]
struct ModuleMapEdgeView<'a> {
    d: &'a str,
    label: Option<&'a str>,
    label_pos: Option<LabelPosition>,
}

impl<'a> ModuleMapEdgeView<'a> {
    fn from_edge(edge: &'a ModuleMapEdge, view_box: &str) -> Self {
        let label_pos = edge.label.as_deref().map(|text| {
            let mut pos = edge_label_position(&edge.d, text, view_box);
            if let Some(x) = edge.label_x {
                pos.x = x;
                pos.anchor = "middle";
            }
            if let Some(y) = edge.label_y {
                pos.y = y;
            }
            pos
        });
        Self {
            d: &edge.d,
            label: edge.label.as_deref(),
            label_pos,
        }
    }
}
