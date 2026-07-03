use minijinja::{Value, context};
use serde::Deserialize;

use super::ComponentStrategy;

#[derive(Debug, Deserialize, PartialEq)]
pub struct NoticeData {
    pub variant: String,
    pub icon: Option<String>,
    pub content: String,
}

impl ComponentStrategy for NoticeData {
    fn required_assets(&self) -> (Vec<&'static str>, Vec<&'static str>) {
        (vec!["css/notice.css"], vec![])
    }

    fn template_name(&self) -> &'static str {
        "notice"
    }

    fn render_context(&self, children_html: &str) -> Value {
        let content_html = super::render_markdown(&self.content);
        context! {
            variant => &self.variant,
            icon => &self.icon,
            content => content_html,
            children => children_html,
        }
    }
}
