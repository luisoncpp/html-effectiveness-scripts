use minijinja::{context, Value};
use serde::{Deserialize, Serialize};

use super::ComponentStrategy;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SvgElement {
    #[serde(rename = "type")]
    pub element_type: String,
    pub x: Option<u32>,
    pub y: Option<u32>,
    pub x2: Option<u32>,
    pub y2: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub cx: Option<u32>,
    pub cy: Option<u32>,
    pub r: Option<u32>,
    pub class: Option<String>,
    pub text: Option<String>,
    pub marker: Option<String>,
}

fn default_view_box() -> String {
    "0 0 800 600".to_string()
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SvgCanvasData {
    #[serde(default = "default_view_box", rename = "viewBox")]
    pub view_box: String,
    #[serde(default)]
    pub interactive: bool,
    pub elements: Vec<SvgElement>,
}

impl ComponentStrategy for SvgCanvasData {
    fn required_assets(&self) -> (Vec<&'static str>, Vec<&'static str>) {
        (vec!["css/svg_canvas.css"], vec![])
    }

    fn template_name(&self) -> &'static str {
        "svg_canvas"
    }

    fn render_context(&self, children_html: &str) -> Value {
        context! {
            view_box => &self.view_box,
            interactive => &self.interactive,
            elements => &self.elements,
            children => children_html,
        }
    }
}
