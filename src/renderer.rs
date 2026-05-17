use anyhow::Result;
use minijinja::{context, Environment};

use crate::models::base::Renderable;
use crate::parser::ParsedDocument;

pub struct TemplateEngine {
    env: Environment<'static>,
}

impl TemplateEngine {
    pub fn new() -> Result<Self> {
        let mut env = Environment::new();
        env.add_template("base", include_str!("../templates/base.html"))?;
        env.add_template(
            "prompt_box",
            include_str!("../templates/components/prompt_box.html"),
        )?;
        Ok(Self { env })
    }

    pub fn render(&self, name: &str, ctx: minijinja::Value) -> Result<String> {
        let template = self.env.get_template(name)?;
        Ok(template.render(ctx)?)
    }
}

pub fn render_document(parsed: &ParsedDocument, engine: &TemplateEngine) -> Result<String> {
    let mut body = parsed.html.clone();
    for (index, component) in parsed.components.iter().enumerate() {
        let rendered = component.render(engine);
        let placeholder = format!("<!-- COMPONENT_PLACEHOLDER_{} -->", index);
        body = body.replace(&placeholder, &rendered);
    }
    engine.render("base", context! { content => body })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::components::prompt_box::PromptBoxData;
    use crate::models::ui_component::UiComponent;

    #[test]
    fn template_engine_loads_all_templates() {
        let engine = TemplateEngine::new();
        assert!(engine.is_ok());
    }

    #[test]
    fn renders_prompt_box_with_variables() {
        let engine = TemplateEngine::new().unwrap();
        let html = engine
            .render(
                "prompt_box",
                context! {
                    label => "Test Label",
                    content => "Test Content",
                },
            )
            .unwrap();

        assert!(html.contains("Test Label"));
        assert!(html.contains("Test Content"));
        assert!(html.contains("<div class=\"prompt-box\">"));
    }

    #[test]
    fn render_document_replaces_placeholders() {
        let engine = TemplateEngine::new().unwrap();
        let parsed = ParsedDocument {
            html: "<h1>Title</h1>\n<!-- COMPONENT_PLACEHOLDER_0 -->\n<p>Text</p>".to_string(),
            components: vec![UiComponent::PromptBox(PromptBoxData {
                label: "My Label".to_string(),
                content: "My Content".to_string(),
            })],
        };

        let result = render_document(&parsed, &engine).unwrap();
        assert!(result.contains("<h1>Title</h1>"));
        assert!(result.contains("My Label"));
        assert!(result.contains("My Content"));
        assert!(!result.contains("COMPONENT_PLACEHOLDER"));
        assert!(result.contains("<!DOCTYPE html>"));
    }
}
