use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct PromptBoxData {
    pub label: String,
    pub content: String,
}
