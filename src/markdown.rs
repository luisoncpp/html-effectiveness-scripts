use pulldown_cmark::Options;

/// Shared pulldown-cmark options for all Markdown rendering in the compiler.
pub fn options() -> Options {
    Options::ENABLE_TABLES
}

#[cfg(test)]
mod tests {
    use super::*;
    use pulldown_cmark::{html, Parser};

    #[test]
    fn tables_render_as_html() {
        let md = "| A | B |\n|---|---|\n| 1 | 2 |";
        let parser = Parser::new_ext(md, options());
        let mut html_out = String::new();
        html::push_html(&mut html_out, parser);
        assert!(html_out.contains("<table>"));
        assert!(html_out.contains("<th>A</th>"));
        assert!(html_out.contains("<td>1</td>"));
    }
}
