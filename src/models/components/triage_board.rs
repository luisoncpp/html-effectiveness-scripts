use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct TriageBoardData {
    pub eyebrow: String,
    pub title: String,
    pub subtitle: String,
    pub hintline: String,
}
