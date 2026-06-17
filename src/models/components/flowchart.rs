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
        let details: Vec<FlowchartDetailView> = self
            .details
            .iter()
            .map(|d| FlowchartDetailView {
                title: super::render_markdown_inline(&d.title),
                meta: super::render_markdown_inline(&d.meta),
                body: super::render_markdown_inline(&d.body),
                code: d.code.as_deref(),
            })
            .collect();
        context! {
            title => super::render_markdown_inline(&self.title),
            description => self.description.as_deref().map(super::render_markdown_inline),
            view_box => &self.view_box,
            nodes => &self.nodes,
            edges => &self.edges,
            details => details,
            children => children_html,
        }
    }
}

#[derive(Serialize)]
struct FlowchartDetailView<'a> {
    title: String,
    meta: String,
    body: String,
    code: Option<&'a str>,
}
