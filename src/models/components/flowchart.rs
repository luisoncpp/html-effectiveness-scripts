use minijinja::{Value, context};
use serde::{Deserialize, Serialize};

use super::ComponentStrategy;
use super::svg_path::{LabelPosition, edge_label_position};

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
    #[serde(default)]
    pub label_x: Option<i32>,
    #[serde(default)]
    pub label_y: Option<i32>,
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
        let edges: Vec<FlowchartEdgeView> = self
            .edges
            .iter()
            .map(|e| FlowchartEdgeView::from_edge(e, &self.view_box))
            .collect();
        let initial_detail = first_detail_idx(&self.nodes)
            .and_then(|idx| details.get(idx))
            .cloned();
        context! {
            title => super::render_markdown_inline(&self.title),
            description => self.description.as_deref().map(super::render_markdown_inline),
            view_box => &self.view_box,
            nodes => &self.nodes,
            edges => edges,
            details => details,
            initial_detail => initial_detail,
            children => children_html,
        }
    }
}

fn first_detail_idx(nodes: &[FlowchartNode]) -> Option<usize> {
    nodes.iter().find_map(|node| node.detail_idx)
}

#[derive(Serialize)]
struct FlowchartEdgeView<'a> {
    d: &'a str,
    edge_type: &'a str,
    label: Option<&'a str>,
    label_pos: Option<LabelPosition>,
}

impl<'a> FlowchartEdgeView<'a> {
    fn from_edge(edge: &'a FlowchartEdge, view_box: &str) -> Self {
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
            edge_type: &edge.edge_type,
            label: edge.label.as_deref(),
            label_pos,
        }
    }
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

    #[test]
    fn edge_labels_are_positioned_along_their_paths_not_at_origin() {
        let engine = crate::renderer::TemplateEngine::new().unwrap();
        let data = FlowchartData {
            title: "Sample flow".to_string(),
            description: None,
            view_box: "0 0 640 470".to_string(),
            nodes: vec![sample_node("a", None)],
            edges: vec![
                FlowchartEdge {
                    from: "a".into(),
                    to: "b".into(),
                    d: "M320,124 L320,150".into(),
                    edge_type: String::new(),
                    label: Some("when ready".into()),
                    label_x: None,
                    label_y: None,
                },
                FlowchartEdge {
                    from: "b".into(),
                    to: "c".into(),
                    d: "M278,246 L200,246".into(),
                    edge_type: "no".into(),
                    label: Some("no".into()),
                    label_x: None,
                    label_y: None,
                },
                FlowchartEdge {
                    from: "d".into(),
                    to: "a".into(),
                    d: "M440,414 L610,414 L610,32 L420,32".into(),
                    edge_type: String::new(),
                    label: Some("return to start".into()),
                    label_x: None,
                    label_y: None,
                },
            ],
            details: vec![],
        };

        let ctx = data.render_context("");
        let html = engine.render(data.template_name(), ctx).unwrap();

        assert!(!html.contains(r#"class="lbl" x="0" y="0""#));
        assert!(html.contains(r#"x="328" y="140" text-anchor="start">when ready</text>"#));
        assert!(html.contains(r#"x="239" y="240" text-anchor="middle">no</text>"#));
        assert!(html.contains(r#"x="602" y="226" text-anchor="end">return to start</text>"#));
    }
}
