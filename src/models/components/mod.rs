use minijinja::Value;

pub mod prompt_box;
pub mod triage_board;

pub trait ComponentStrategy {
    fn required_assets(&self) -> (Vec<&'static str>, Vec<&'static str>);
    fn template_name(&self) -> &'static str;
    fn render_context(&self, children_html: &str) -> Value;
}
