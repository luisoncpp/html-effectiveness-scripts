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
        let initial_detail = first_detail_idx(&self.nodes)
            .and_then(|idx| details.get(idx))
            .cloned();
        context! {
            title => super::render_markdown_inline(&self.title),
            description => self.description.as_deref().map(super::render_markdown_inline),
            view_box => &self.view_box,
            nodes => &self.nodes,
            edges => &self.edges,
            details => details,
            initial_detail => initial_detail,
            children => children_html,
        }
    }
}

fn first_detail_idx(nodes: &[FlowchartNode]) -> Option<usize> {
    nodes.iter().find_map(|node| node.detail_idx)
}

#[derive(Clone, Serialize)]
struct FlowchartDetailView<'a> {
    title: String,
    meta: String,
    body: String,
    code: Option<&'a str>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_node(id: &str, detail_idx: Option<usize>) -> FlowchartNode {
        FlowchartNode {
            id: id.to_string(),
            node_type: "rect".to_string(),
            x: 0,
            y: 0,
            width: 10,
            height: 10,
            label: id.to_string(),
            sublabel: None,
            detail_idx,
        }
    }

    #[test]
    fn first_detail_idx_skips_nodes_without_descriptions() {
        let nodes = vec![
            sample_node("start", None),
            sample_node("step", Some(0)),
            sample_node("end", Some(1)),
        ];
        assert_eq!(first_detail_idx(&nodes), Some(0));
    }
}
