pub mod assets;
pub mod cli;
pub mod compiler;
pub mod highlight;
pub mod html_options;
pub mod markdown;
pub mod models;
pub mod parser;
pub mod renderer;

use assets::AssetRegistry;
pub use html_options::{ColorMode, CssLength, CssUnit, HtmlOptions, Margins};
use models::document_context::DocumentContext;

pub type CompileError = anyhow::Error;
pub type Metadata = DocumentContext;

#[derive(Debug, Default)]
pub struct CompileOptions {
    pub html: Option<HtmlOptions>,
}

#[derive(Debug)]
pub struct CompiledDocument {
    pub html: String,
    pub metadatata: Option<Metadata>,
}

pub fn compile(source: &str, options: &CompileOptions) -> Result<CompiledDocument, CompileError> {
    let parsed = parser::parse(source)?;
    let engine = renderer::TemplateEngine::new()?;
    let registry = AssetRegistry::from_blocks(&parsed.blocks)
        .with_theme(&parsed.context.theme_tokens)
        .with_base_assets();
    let render_context = renderer::DocumentRenderContext {
        assets: &registry,
        html: options.html.as_ref(),
    };
    let html = renderer::render_document_configured(&parsed, &engine, render_context)?;
    let metadatata = source.starts_with("---").then_some(parsed.context);

    Ok(CompiledDocument { html, metadatata })
}

#[cfg(test)]
mod api_tests {
    use super::*;

    #[test]
    fn compiles_source_and_returns_frontmatter_metadata() {
        let source = "---\ntitle: Example\n---\n# Hello";

        let document = compile(source, &CompileOptions::default()).unwrap();

        assert!(document.html.contains("<h1>Hello</h1>"));
        assert_eq!(
            document.metadatata.and_then(|metadata| metadata.title),
            Some("Example".to_string())
        );
    }

    #[test]
    fn omits_metadata_when_source_has_no_frontmatter() {
        let document = compile("# Hello", &CompileOptions::default()).unwrap();

        assert!(document.metadatata.is_none());
    }

    #[test]
    fn blocks_only_custom_source_scripts() {
        let options = CompileOptions {
            html: Some(HtmlOptions {
                enable_custom_scripts: Some(false),
                ..Default::default()
            }),
        };
        let source =
            "<script>custom()</script>\n\n```yaml\ntype: data-grid\ncolumns: [A]\nrows: [[B]]\n```";

        let document = compile(source, &options).unwrap();

        assert!(!document.html.contains("custom()"));
        assert!(document.html.contains("querySelector"));
    }

    #[test]
    fn zero_margins_remove_page_spacing() {
        let options = CompileOptions {
            html: Some(HtmlOptions {
                margins: Some(Margins::ZERO),
                ..Default::default()
            }),
        };

        let document = compile("# Compact", &options).unwrap();

        assert!(document.html.contains(
            r#"style="padding-top: 0px; padding-right: 0px; padding-bottom: 0px; padding-left: 0px;""#
        ));
    }

    #[test]
    fn fixed_color_modes_do_not_render_a_toggle() {
        for mode in [ColorMode::Light, ColorMode::Dark] {
            let options = CompileOptions {
                html: Some(HtmlOptions {
                    mode: Some(mode),
                    ..Default::default()
                }),
            };
            let document = compile("# Fixed", &options).unwrap();

            assert!(!document.html.contains("id=\"theme-toggle\""));
            assert_eq!(
                document
                    .html
                    .contains(r#"<html lang="en" data-theme="dark">"#),
                mode == ColorMode::Dark
            );
        }
    }
}
