use minijinja::{context, Value};
use serde::Deserialize;

use super::ComponentStrategy;

fn default_elevation() -> u8 {
    1
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CardData {
    pub title: Option<String>,
    #[serde(default = "default_elevation")]
    pub elevation: u8,
    #[serde(default)]
    pub tags: Vec<String>,
    pub content: Option<String>,
}

impl ComponentStrategy for CardData {
    fn required_assets(&self) -> (Vec<&'static str>, Vec<&'static str>) {
        (vec!["css/card.css"], vec![])
    }

    fn template_name(&self) -> &'static str {
        "card"
    }

    fn render_context(&self, children_html: &str) -> Value {
        let content_html = self.content.as_deref().map(super::render_markdown);
        let title_html = self.title.as_deref().map(super::render_markdown_inline);
        context! {
            title => title_html,
            elevation => &self.elevation,
            tags => &self.tags,
            content => content_html,
            children => children_html,
        }
    }
}
