use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct DocumentContext {
    pub title: Option<String>,
    #[serde(default, alias = "layout")]
    pub layout_wrapper: LayoutType,
    #[serde(default, alias = "theme")]
    pub theme_tokens: String,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub enum LayoutType {
    #[default]
    #[serde(rename = "reading-column")]
    ReadingColumn,
    #[serde(rename = "wide")]
    Wide,
    #[serde(rename = "canvas")]
    Canvas,
}
