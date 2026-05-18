use minijinja::Value;

pub mod board_layout;
pub mod card;
pub mod code_panel;
pub mod data_grid;
pub mod flowchart;
pub mod module_map;
pub mod notice;
pub mod prompt_box;
pub mod svg_canvas;
pub mod timeline;
pub mod triage_board;

pub trait ComponentStrategy {
    fn required_assets(&self) -> (Vec<&'static str>, Vec<&'static str>);
    fn template_name(&self) -> &'static str;
    fn render_context(&self, children_html: &str) -> Value;
}
