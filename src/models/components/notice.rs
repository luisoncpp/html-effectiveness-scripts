use minijinja::{context, Value};
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
        context! {
            variant => &self.variant,
            icon => &self.icon,
            content => &self.content,
            children => children_html,
        }
    }
}
