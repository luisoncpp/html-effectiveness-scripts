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

/// Render block-level Markdown to HTML. Use for multi-paragraph content areas
/// (e.g. `notice.content`, `card.content`) where `<p>`, lists, etc. are wanted.
pub fn render_markdown(markdown: &str) -> String {
    let parser = pulldown_cmark::Parser::new_ext(markdown, crate::markdown::options());
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
}

/// Render Markdown for a single-line context (titles, table cells, labels)
/// where a wrapping `<p>` would be unwanted. Inline formatting (`**bold**`,
/// `` `code` ``, `[links](…)`, `*em*`) is honored; a lone wrapping paragraph is
/// unwrapped. Content with multiple blocks falls back to the full block render.
pub fn render_markdown_inline(markdown: &str) -> String {
    let html = render_markdown(markdown);
    let trimmed = html.trim();
    if let Some(inner) = trimmed
        .strip_prefix("<p>")
        .and_then(|s| s.strip_suffix("</p>"))
    {
        if !inner.contains("<p>") {
            return inner.to_string();
        }
    }
    trimmed.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inline_unwraps_lone_paragraph_and_keeps_formatting() {
        assert_eq!(render_markdown_inline("Hello **world**"), "Hello <strong>world</strong>");
        assert_eq!(render_markdown_inline("`code`"), "<code>code</code>");
    }

    #[test]
    fn inline_escapes_ampersand() {
        assert_eq!(render_markdown_inline("Drag & Drop"), "Drag &amp; Drop");
    }

    #[test]
    fn inline_falls_back_to_blocks_for_multi_paragraph() {
        let out = render_markdown_inline("one\n\ntwo");
        assert!(out.contains("<p>one</p>"));
        assert!(out.contains("<p>two</p>"));
    }
}
