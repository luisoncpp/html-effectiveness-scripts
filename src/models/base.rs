use crate::renderer::TemplateEngine;

pub trait Renderable {
    fn render(&self, engine: &TemplateEngine) -> String;
}
