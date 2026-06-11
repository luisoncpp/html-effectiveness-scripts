use minijinja::Value;

pub mod board_layout;
pub mod card;
pub mod code_map;
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

pub fn render_markdown(markdown: &str) -> String {
    let parser = pulldown_cmark::Parser::new(markdown);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
}
