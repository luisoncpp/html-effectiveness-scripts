use minijinja::{Value, context};
use serde::{Deserialize, Serialize};

use super::ComponentStrategy;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TimelineStep {
    pub timestamp: String,
    pub title: String,
    #[serde(rename = "type")]
    pub step_type: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

fn default_orientation() -> String {
    "vertical".to_string()
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TimelineData {
    #[serde(default = "default_orientation")]
    pub orientation: String,
    pub steps: Vec<TimelineStep>,
}

impl ComponentStrategy for TimelineData {
    fn required_assets(&self) -> (Vec<&'static str>, Vec<&'static str>) {
        (vec!["css/timeline.css"], vec![])
    }

    fn template_name(&self) -> &'static str {
        "timeline"
    }

    fn render_context(&self, children_html: &str) -> Value {
        let steps: Vec<TimelineStepView> = self
            .steps
            .iter()
            .map(|step| TimelineStepView {
                timestamp: &step.timestamp,
                title: super::render_markdown_inline(&step.title),
                step_type: &step.step_type,
                description: step
                    .description
                    .as_deref()
                    .map(super::render_markdown_inline),
                tags: step.tags.as_deref(),
            })
            .collect();
        context! {
            orientation => &self.orientation,
            steps => steps,
            children => children_html,
        }
    }
}

#[derive(Serialize)]
struct TimelineStepView<'a> {
    timestamp: &'a str,
    title: String,
    #[serde(rename = "type")]
    step_type: &'a str,
    description: Option<String>,
    tags: Option<&'a [String]>,
}
