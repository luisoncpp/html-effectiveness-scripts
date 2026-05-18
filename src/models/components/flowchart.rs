use minijinja::{context, Value};
use serde::{Deserialize, Serialize};

use super::ComponentStrategy;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FlowchartNode {
    pub id: String,
    #[serde(rename = "type")]
    pub node_type: String, // "rect", "diamond", "terminal"
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub label: String,
    pub sublabel: Option<String>,
    pub detail_idx: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FlowchartEdge {
    pub from: String,
    pub to: String,
    pub d: String,
    #[serde(default)]
    pub edge_type: String, // "yes", "no", "normal"
    pub label: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FlowchartDetail {
    pub title: String,
    pub meta: String,
    pub body: String,
    pub code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FlowchartData {
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "viewBox")]
    pub view_box: String,
    pub nodes: Vec<FlowchartNode>,
    pub edges: Vec<FlowchartEdge>,
    pub details: Vec<FlowchartDetail>,
}

impl ComponentStrategy for FlowchartData {
    fn required_assets(&self) -> (Vec<&'static str>, Vec<&'static str>) {
        (vec!["css/flowchart.css"], vec!["js/flowchart.js"])
    }

    fn template_name(&self) -> &'static str {
        "flowchart"
    }

    fn render_context(&self, children_html: &str) -> Value {
        context! {
            title => &self.title,
            description => &self.description,
            view_box => &self.view_box,
            nodes => &self.nodes,
            edges => &self.edges,
            details => &self.details,
            children => children_html,
        }
    }
}
