use serde::Deserialize;

use super::base::Renderable;
use super::block::Block;
use super::components::board_layout::BoardLayoutData;
use super::components::card::CardData;
use super::components::code_map::CodeMapData;
use super::components::code_panel::CodePanelData;
use super::components::data_grid::DataGridData;
use super::components::flowchart::FlowchartData;
use super::components::module_map::ModuleMapData;
use super::components::notice::NoticeData;
use super::components::prompt_box::PromptBoxData;
use super::components::svg_canvas::SvgCanvasData;
use super::components::timeline::TimelineData;
use super::components::triage_board::TriageBoardData;
use super::components::ComponentStrategy;
use crate::renderer::TemplateEngine;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum UiComponent {
    #[serde(rename = "card")]
    Card(CardData),
    #[serde(rename = "notice")]
    Notice(NoticeData),
    #[serde(rename = "prompt-box")]
    PromptBox(PromptBoxData),
    #[serde(rename = "svg-canvas")]
    SvgCanvas(SvgCanvasData),
    #[serde(rename = "data-grid")]
    DataGrid(DataGridData),
    #[serde(rename = "timeline")]
    Timeline(TimelineData),
    #[serde(rename = "code-panel")]
    CodePanel(CodePanelData),
    #[serde(rename = "code-map")]
    CodeMap(CodeMapData),
    #[serde(rename = "board-layout")]
    BoardLayout(BoardLayoutData),
    #[serde(rename = "triage-board")]
    TriageBoard(TriageBoardData),
    #[serde(rename = "flowchart")]
    Flowchart(FlowchartData),
    #[serde(rename = "module-map")]
    ModuleMap(ModuleMapData),
}

#[derive(Debug, PartialEq)]
pub struct ComponentBlock {
    pub component: UiComponent,
    pub children: Vec<Block>,
}

impl UiComponent {
    fn strategy(&self) -> &dyn ComponentStrategy {
        match self {
            UiComponent::Card(data) => data,
            UiComponent::Notice(data) => data,
            UiComponent::PromptBox(data) => data,
            UiComponent::SvgCanvas(data) => data,
            UiComponent::DataGrid(data) => data,
            UiComponent::Timeline(data) => data,
            UiComponent::CodePanel(data) => data,
            UiComponent::CodeMap(data) => data,
            UiComponent::BoardLayout(data) => data,
            UiComponent::TriageBoard(data) => data,
            UiComponent::Flowchart(data) => data,
            UiComponent::ModuleMap(data) => data,
        }
    }

    pub fn required_assets(&self) -> (Vec<&'static str>, Vec<&'static str>) {
        self.strategy().required_assets()
    }

    fn template_name(&self) -> &'static str {
        self.strategy().template_name()
    }

    fn render_context(&self, children_html: &str) -> minijinja::Value {
        self.strategy().render_context(children_html)
    }
}

impl ComponentBlock {
    pub fn render(&self, engine: &TemplateEngine) -> String {
        let children_html: String = self
            .children
            .iter()
            .map(|child| match child {
                Block::Prose(html) => html.clone(),
                Block::Component(comp) => comp.render(engine),
            })
            .collect();

        let ctx = self.component.render_context(&children_html);
        let template_name = self.component.template_name();
        engine
            .render(template_name, ctx)
            .unwrap_or_else(|e| format!("<!-- render error: {} -->", e))
    }
}

impl Renderable for UiComponent {
    fn render(&self, engine: &TemplateEngine) -> String {
        let ctx = self.render_context("");
        let template_name = self.template_name();
        engine
            .render(template_name, ctx)
            .unwrap_or_else(|e| format!("<!-- render error: {} -->", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::components::board_layout::{BoardColumn, BoardLayoutData};
    use crate::models::components::code_panel::{CodePanelData, CodeTab};
    use crate::models::components::svg_canvas::{SvgCanvasData, SvgElement};
    use crate::models::components::timeline::TimelineStep;

    #[test]
    fn deserializes_notice_from_valid_yaml() {
        let yaml = r#"
type: notice
variant: warning
icon: alert-triangle
content: <strong>Breaking Change:</strong> The parser now expects multiple blocks.
"#;
        let result: Result<UiComponent, _> = serde_yaml::from_str(yaml);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            UiComponent::Notice(NoticeData {
                variant: "warning".to_string(),
                icon: Some("alert-triangle".to_string()),
                content: "<strong>Breaking Change:</strong> The parser now expects multiple blocks.".to_string(),
            })
        );
    }

    #[test]
    fn deserializes_notice_without_icon() {
        let yaml = r#"
type: notice
variant: info
content: Some informational message.
"#;
        let result: Result<UiComponent, _> = serde_yaml::from_str(yaml);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            UiComponent::Notice(NoticeData {
                variant: "info".to_string(),
                icon: None,
                content: "Some informational message.".to_string(),
            })
        );
    }

    #[test]
    fn notice_declares_required_assets() {
        let comp = UiComponent::Notice(NoticeData {
            variant: "warning".to_string(),
            icon: None,
            content: "Content".to_string(),
        });
        let (css, js) = comp.required_assets();
        assert_eq!(css, vec!["css/notice.css"]);
        assert!(js.is_empty());
    }

    #[test]
    fn deserializes_prompt_box_from_valid_yaml() {
        let yaml = r#"
type: prompt-box
label: Test Label
content: Test Content
"#;
        let result: Result<UiComponent, _> = serde_yaml::from_str(yaml);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            UiComponent::PromptBox(PromptBoxData {
                label: "Test Label".to_string(),
                content: "Test Content".to_string(),
            })
        );
    }

    #[test]
    fn returns_error_for_unknown_type() {
        let yaml = r#"
type: unknown-component
label: Test
"#;
        let result: Result<UiComponent, _> = serde_yaml::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn returns_error_for_missing_required_fields() {
        let yaml = r#"
type: prompt-box
label: Test Label
"#;
        let result: Result<UiComponent, _> = serde_yaml::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn prompt_box_declares_required_assets() {
        let comp = UiComponent::PromptBox(PromptBoxData {
            label: "Test".to_string(),
            content: "Content".to_string(),
        });
        let (css, js) = comp.required_assets();
        assert_eq!(css, vec!["css/prompt_box.css"]);
        assert!(js.is_empty());
    }

    #[test]
    fn deserializes_code_panel_from_valid_yaml() {
        let yaml = r#"
type: code-panel
tabs:
  - name: src/compiler.rs
    language: rust
    diff: true
    content: "- let tree = parse_single();\n+ let ast = parse_blocks();"
  - name: Cargo.toml
    language: toml
    content: "[dependencies]\npulldown-cmark = \"0.9\""
"#;
        let result: Result<UiComponent, _> = serde_yaml::from_str(yaml);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            UiComponent::CodePanel(CodePanelData {
                tabs: vec![
                    CodeTab {
                        name: "src/compiler.rs".to_string(),
                        language: "rust".to_string(),
                        diff: true,
                        content: "- let tree = parse_single();\n+ let ast = parse_blocks();".to_string(),
                        risk: None,
                        added: None,
                        removed: None,
                    },
                    CodeTab {
                        name: "Cargo.toml".to_string(),
                        language: "toml".to_string(),
                        diff: false,
                        content: "[dependencies]\npulldown-cmark = \"0.9\"".to_string(),
                        risk: None,
                        added: None,
                        removed: None,
                    },
                ],
            })
        );
    }

    #[test]
    fn code_panel_declares_required_assets() {
        let comp = UiComponent::CodePanel(CodePanelData {
            tabs: vec![CodeTab {
                name: "test.rs".to_string(),
                language: "rust".to_string(),
                diff: false,
                content: "fn main() {}".to_string(),
                risk: None,
                added: None,
                removed: None,
            }],
        });
        let (css, js) = comp.required_assets();
        assert_eq!(css, vec!["css/code_panel.css"]);
        assert_eq!(js, vec!["js/tabs.js"]);
    }

    #[test]
    fn deserializes_card_from_valid_yaml() {
        let yaml = r#"
type: card
title: Card Title
elevation: 2
tags:
  - rust
  - urgent
content: Some content inside the card.
"#;
        let result: Result<UiComponent, _> = serde_yaml::from_str(yaml);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            UiComponent::Card(CardData {
                title: Some("Card Title".to_string()),
                elevation: 2,
                tags: vec!["rust".to_string(), "urgent".to_string()],
                content: Some("Some content inside the card.".to_string()),
            })
        );
    }

    #[test]
    fn deserializes_card_with_defaults() {
        let yaml = r#"
type: card
content: Minimal card.
"#;
        let result: Result<UiComponent, _> = serde_yaml::from_str(yaml);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            UiComponent::Card(CardData {
                title: None,
                elevation: 1,
                tags: vec![],
                content: Some("Minimal card.".to_string()),
            })
        );
    }

    #[test]
    fn card_declares_required_assets() {
        let comp = UiComponent::Card(CardData {
            title: None,
            elevation: 1,
            tags: vec![],
            content: None,
        });
        let (css, js) = comp.required_assets();
        assert_eq!(css, vec!["css/card.css"]);
        assert!(js.is_empty());
    }

    #[test]
    fn deserializes_data_grid_from_valid_yaml() {
        let yaml = r#"
type: data-grid
columns:
  - Feature
  - Status
  - Risk
rows:
  - ["AST Traversal", "Shipped", "Low"]
  - ["Drag & Drop", "WIP", "High"]
"#;
        let result: Result<UiComponent, _> = serde_yaml::from_str(yaml);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            UiComponent::DataGrid(DataGridData {
                columns: vec!["Feature".to_string(), "Status".to_string(), "Risk".to_string()],
                rows: vec![
                    vec!["AST Traversal".to_string(), "Shipped".to_string(), "Low".to_string()],
                    vec!["Drag & Drop".to_string(), "WIP".to_string(), "High".to_string()],
                ],
            })
        );
    }

    #[test]
    fn data_grid_declares_required_assets() {
        let comp = UiComponent::DataGrid(DataGridData {
            columns: vec!["A".to_string()],
            rows: vec![vec!["B".to_string()]],
        });
        let (css, js) = comp.required_assets();
        assert_eq!(css, vec!["css/data_grid.css"]);
        assert_eq!(js, vec!["js/data_grid.js"]);
    }

    #[test]
    fn deserializes_timeline_from_valid_yaml() {
        let yaml = r#"
type: timeline
orientation: vertical
steps:
  - timestamp: "2026-05-18 10:00"
    title: "Initial Outage"
    type: "critical"
  - timestamp: "2026-05-18 10:15"
    title: "Rolled back to v1.2"
    type: "recovery"
"#;
        let result: Result<UiComponent, _> = serde_yaml::from_str(yaml);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            UiComponent::Timeline(TimelineData {
                orientation: "vertical".to_string(),
                steps: vec![
                    TimelineStep {
                        timestamp: "2026-05-18 10:00".to_string(),
                        title: "Initial Outage".to_string(),
                        step_type: "critical".to_string(),
                        description: None,
                        tags: None,
                    },
                    TimelineStep {
                        timestamp: "2026-05-18 10:15".to_string(),
                        title: "Rolled back to v1.2".to_string(),
                        step_type: "recovery".to_string(),
                        description: None,
                        tags: None,
                    },
                ],
            })
        );
    }

    #[test]
    fn deserializes_timeline_with_defaults() {
        let yaml = r#"
type: timeline
steps:
  - timestamp: "2026-05-18 10:00"
    title: "Step One"
    type: "info"
"#;
        let result: Result<UiComponent, _> = serde_yaml::from_str(yaml);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            UiComponent::Timeline(TimelineData {
                orientation: "vertical".to_string(),
                steps: vec![TimelineStep {
                    timestamp: "2026-05-18 10:00".to_string(),
                    title: "Step One".to_string(),
                    step_type: "info".to_string(),
                    description: None,
                    tags: None,
                }],
            })
        );
    }

    #[test]
    fn timeline_declares_required_assets() {
        let comp = UiComponent::Timeline(TimelineData {
            orientation: "vertical".to_string(),
            steps: vec![TimelineStep {
                timestamp: "2026-05-18 10:00".to_string(),
                title: "Step".to_string(),
                step_type: "info".to_string(),
                description: None,
                tags: None,
            }],
        });
        let (css, js) = comp.required_assets();
        assert_eq!(css, vec!["css/timeline.css"]);
        assert!(js.is_empty());
    }

    #[test]
    fn deserializes_board_layout_from_valid_yaml() {
        let yaml = r#"
type: board-layout
variant: kanban
columns:
  - title: "To Do"
    items:
      - "Task A"
      - "Task B"
  - title: "Done"
    items:
      - "Task C"
"#;
        let result: Result<UiComponent, _> = serde_yaml::from_str(yaml);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            UiComponent::BoardLayout(BoardLayoutData {
                variant: "kanban".to_string(),
                columns: vec![
                    BoardColumn {
                        title: "To Do".to_string(),
                        items: vec!["Task A".to_string(), "Task B".to_string()],
                    },
                    BoardColumn {
                        title: "Done".to_string(),
                        items: vec!["Task C".to_string()],
                    },
                ],
            })
        );
    }

    #[test]
    fn deserializes_board_layout_with_defaults() {
        let yaml = r#"
type: board-layout
columns:
  - title: "Backlog"
"#;
        let result: Result<UiComponent, _> = serde_yaml::from_str(yaml);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            UiComponent::BoardLayout(BoardLayoutData {
                variant: "kanban".to_string(),
                columns: vec![BoardColumn {
                    title: "Backlog".to_string(),
                    items: vec![],
                }],
            })
        );
    }

    #[test]
    fn board_layout_declares_required_assets() {
        let comp = UiComponent::BoardLayout(BoardLayoutData {
            variant: "grid".to_string(),
            columns: vec![BoardColumn {
                title: "Col".to_string(),
                items: vec![],
            }],
        });
        let (css, js) = comp.required_assets();
        assert_eq!(css, vec!["css/board_layout.css"]);
        assert!(js.is_empty());
    }

    #[test]
    fn deserializes_svg_canvas_from_valid_yaml() {
        let yaml = r#"
type: svg-canvas
viewBox: "0 0 800 600"
interactive: true
elements:
  - type: rect
    x: 10
    y: 10
    width: 100
    height: 60
    class: "node-primary"
  - type: circle
    cx: 200
    cy: 200
    r: 50
    class: "node-secondary"
  - type: text
    x: 10
    y: 10
    text: "Hello SVG"
"#;
        let result: Result<UiComponent, _> = serde_yaml::from_str(yaml);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            UiComponent::SvgCanvas(SvgCanvasData {
                view_box: "0 0 800 600".to_string(),
                interactive: true,
                elements: vec![
                    SvgElement {
                        element_type: "rect".to_string(),
                        x: Some(10),
                        y: Some(10),
                        x2: None,
                        y2: None,
                        width: Some(100),
                        height: Some(60),
                        cx: None,
                        cy: None,
                        r: None,
                        class: Some("node-primary".to_string()),
                        text: None,
                        marker: None,
                    },
                    SvgElement {
                        element_type: "circle".to_string(),
                        x: None,
                        y: None,
                        x2: None,
                        y2: None,
                        width: None,
                        height: None,
                        cx: Some(200),
                        cy: Some(200),
                        r: Some(50),
                        class: Some("node-secondary".to_string()),
                        text: None,
                        marker: None,
                    },
                    SvgElement {
                        element_type: "text".to_string(),
                        x: Some(10),
                        y: Some(10),
                        x2: None,
                        y2: None,
                        width: None,
                        height: None,
                        cx: None,
                        cy: None,
                        r: None,
                        class: None,
                        text: Some("Hello SVG".to_string()),
                        marker: None,
                    },
                ],
            })
        );
    }

    #[test]
    fn deserializes_svg_canvas_with_defaults() {
        let yaml = r#"
type: svg-canvas
elements:
  - type: rect
    x: 5
    y: 5
    width: 50
    height: 30
"#;
        let result: Result<UiComponent, _> = serde_yaml::from_str(yaml);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            UiComponent::SvgCanvas(SvgCanvasData {
                view_box: "0 0 800 600".to_string(),
                interactive: false,
                elements: vec![SvgElement {
                    element_type: "rect".to_string(),
                    x: Some(5),
                    y: Some(5),
                    x2: None,
                    y2: None,
                    width: Some(50),
                    height: Some(30),
                    cx: None,
                    cy: None,
                    r: None,
                    class: None,
                    text: None,
                    marker: None,
                }],
            })
        );
    }

    #[test]
    fn svg_canvas_declares_required_assets() {
        let comp = UiComponent::SvgCanvas(SvgCanvasData {
            view_box: "0 0 800 600".to_string(),
            interactive: false,
            elements: vec![],
        });
        let (css, js) = comp.required_assets();
        assert_eq!(css, vec!["css/svg_canvas.css"]);
        assert!(js.is_empty());
    }
}
