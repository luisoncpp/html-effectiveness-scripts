use minijinja::{context, Value};
use serde::Deserialize;

use super::ComponentStrategy;

#[derive(Debug, Deserialize, PartialEq)]
pub struct TriageBoardData {
    pub eyebrow: String,
    pub title: String,
    pub subtitle: String,
    pub hintline: String,
}

impl ComponentStrategy for TriageBoardData {
    fn required_assets(&self) -> (Vec<&'static str>, Vec<&'static str>) {
        (vec!["css/triage_board.css"], vec!["js/triage_board.js"])
    }

    fn template_name(&self) -> &'static str {
        "triage_board"
    }

    fn render_context(&self, children_html: &str) -> Value {
        context! {
            eyebrow => &self.eyebrow,
            title => super::render_markdown_inline(&self.title),
            subtitle => super::render_markdown_inline(&self.subtitle),
            hintline => super::render_markdown_inline(&self.hintline),
            children => children_html,
        }
    }
}
