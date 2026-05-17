use anyhow::Result;
use minijinja::{context, Environment};

use crate::assets::AssetRegistry;
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

pub fn render_document(
    parsed: &ParsedDocument,
    engine: &TemplateEngine,
    registry: &AssetRegistry,
) -> Result<String> {
    let body = render_blocks(&parsed.blocks, engine);
    let title = parsed.context.title.as_deref().unwrap_or("Document");
    engine.render("base", context! {
        content => body,
        title => title,
        inline_styles => registry.inline_styles(),
        inline_scripts => registry.inline_scripts(),
    })
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
        let registry = AssetRegistry::from_blocks(&parsed.blocks).with_base_assets();

        let result = render_document(&parsed, &engine, &registry).unwrap();
        assert!(result.contains("<h1>Title</h1>"));
        assert!(result.contains("My Label"));
        assert!(result.contains("My Content"));
        assert!(!result.contains("COMPONENT_PLACEHOLDER"));
        assert!(result.contains("<!DOCTYPE html>"));
    }

    #[test]
    fn render_no_external_links() {
        let engine = TemplateEngine::new().unwrap();
        let parsed = ParsedDocument {
            blocks: vec![Block::Component(ComponentBlock {
                component: UiComponent::PromptBox(PromptBoxData {
                    label: "Label".to_string(),
                    content: "Content".to_string(),
                }),
                children: vec![],
            })],
            context: DocumentContext::default(),
        };
        let registry = AssetRegistry::from_blocks(&parsed.blocks).with_base_assets();
        let html = render_document(&parsed, &engine, &registry).unwrap();
        assert!(!html.contains(r#"href="*.css""#));
        assert!(!html.contains(r#"src="*.js""#));
        assert!(!html.contains(r#".css""#));
        assert!(!html.contains(r#".js""#));
    }

    #[test]
    fn render_inline_styles_present() {
        let engine = TemplateEngine::new().unwrap();
        let parsed = ParsedDocument {
            blocks: vec![Block::Component(ComponentBlock {
                component: UiComponent::PromptBox(PromptBoxData {
                    label: "Label".to_string(),
                    content: "Content".to_string(),
                }),
                children: vec![],
            })],
            context: DocumentContext::default(),
        };
        let registry = AssetRegistry::from_blocks(&parsed.blocks).with_base_assets();
        let html = render_document(&parsed, &engine, &registry).unwrap();
        assert!(html.contains("<style>"));
        assert!(html.contains(".prompt-box"));
    }

    #[test]
    fn render_inline_scripts_present() {
        let engine = TemplateEngine::new().unwrap();
        let parsed = ParsedDocument {
            blocks: vec![],
            context: DocumentContext::default(),
        };
        let mut registry = AssetRegistry::from_blocks(&parsed.blocks).with_base_assets();
        registry.scripts.insert("js/test.js".to_string());
        let html = render_document(&parsed, &engine, &registry).unwrap();
        assert!(html.contains("<script>"));
        assert!(html.contains("// test script content"));
    }

    #[test]
    fn render_base_receives_title() {
        let engine = TemplateEngine::new().unwrap();
        let parsed = ParsedDocument {
            blocks: vec![],
            context: DocumentContext {
                title: Some("My Title".to_string()),
                ..Default::default()
            },
        };
        let registry = AssetRegistry::from_blocks(&parsed.blocks).with_base_assets();
        let html = render_document(&parsed, &engine, &registry).unwrap();
        assert!(html.contains("<title>My Title</title>"));
    }

    #[test]
    fn render_base_receives_theme() {
        let engine = TemplateEngine::new().unwrap();
        let parsed = ParsedDocument {
            blocks: vec![],
            context: DocumentContext {
                theme_tokens: "clay-slate".to_string(),
                ..Default::default()
            },
        };
        let registry = AssetRegistry::from_blocks(&parsed.blocks)
            .with_theme(&parsed.context.theme_tokens)
            .with_base_assets();
        let html = render_document(&parsed, &engine, &registry).unwrap();
        assert!(html.contains("<style>"));
        assert!(html.contains("--color-primary: #64748b"));
    }
}
