use anyhow::Result;
use minijinja::{context, Environment};

use crate::models::block::Block;
use crate::models::ui_component::ComponentBlock;
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
    let body = render_blocks(&parsed.blocks, engine);
    engine.render("base", context! { content => body })
}

fn render_blocks(blocks: &[Block], engine: &TemplateEngine) -> String {
    let mut result = String::new();
    for block in blocks {
        let rendered = render_block(block, engine);
        if !result.is_empty()
            && !result.ends_with('\n')
            && !rendered.starts_with('\n')
        {
            result.push('\n');
        }
        result.push_str(&rendered);
    }
    result
}

fn render_block(block: &Block, engine: &TemplateEngine) -> String {
    match block {
        Block::Prose(html) => html.clone(),
        Block::Component(comp) => render_component(comp, engine),
    }
}

fn render_component(comp: &ComponentBlock, engine: &TemplateEngine) -> String {
    comp.render(engine)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::components::prompt_box::PromptBoxData;
    use crate::models::document_context::DocumentContext;
    use crate::models::ui_component::{ComponentBlock, UiComponent};

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
    fn render_document_produces_correct_output() {
        let engine = TemplateEngine::new().unwrap();
        let parsed = ParsedDocument {
            blocks: vec![
                Block::Prose("<h1>Title</h1>\n".to_string()),
                Block::Component(ComponentBlock {
                    component: UiComponent::PromptBox(PromptBoxData {
                        label: "My Label".to_string(),
                        content: "My Content".to_string(),
                    }),
                    children: vec![],
                }),
                Block::Prose("<p>Text</p>\n".to_string()),
            ],
            context: DocumentContext::default(),
        };

        let result = render_document(&parsed, &engine).unwrap();
        assert!(result.contains("<h1>Title</h1>"));
        assert!(result.contains("My Label"));
        assert!(result.contains("My Content"));
        assert!(!result.contains("COMPONENT_PLACEHOLDER"));
        assert!(result.contains("<!DOCTYPE html>"));
    }
}
