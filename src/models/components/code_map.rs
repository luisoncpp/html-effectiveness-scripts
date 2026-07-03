use minijinja::{Value, context};
use serde::{Deserialize, Serialize};

use super::ComponentStrategy;
use crate::highlight::{escape_html, highlight_code_line};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CodeMapGroup {
    pub label: String,
    #[serde(default = "default_group_variant")]
    pub variant: String, // "amber" | "green" | "blue" | "clay" | "plain"
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CodeMapCard {
    pub id: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: Option<u32>,
    pub title: Option<String>,
    #[serde(default)]
    pub language: String,
    pub code: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CodeMapArrow {
    /// Source reference: "cardId" or "cardId.anchorId".
    pub from: String,
    /// Target reference: "cardId" or "cardId.anchorId".
    pub to: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CodeMapData {
    pub title: Option<String>,
    #[serde(default = "default_canvas_width")]
    pub width: u32,
    pub height: u32,
    #[serde(default)]
    pub groups: Vec<CodeMapGroup>,
    #[serde(default)]
    pub cards: Vec<CodeMapCard>,
    #[serde(default)]
    pub arrows: Vec<CodeMapArrow>,
}

fn default_group_variant() -> String {
    "plain".to_string()
}

fn default_canvas_width() -> u32 {
    1200
}

#[derive(Serialize)]
struct CodeMapLineView {
    html: String,
    hl: bool,
}

#[derive(Serialize)]
struct CodeMapCardView<'a> {
    id: &'a str,
    x: u32,
    y: u32,
    width: u32,
    height: Option<u32>,
    title: Option<&'a str>,
    lines: Vec<CodeMapLineView>,
}

/// Render one source line, turning `[[anchor]]` / `[[id|text]]` markers into
/// `<mark data-anchor="card.id">` tokens and syntax-highlighting the rest.
/// Returns the HTML plus whether the line contains an anchor (highlighted row).
fn render_line(line: &str, lang: &str, card_id: &str) -> (String, bool) {
    let mut html = String::new();
    let mut has_anchor = false;
    let mut rest = line;

    while !rest.is_empty() {
        let Some(start) = rest.find("[[") else {
            html.push_str(&highlight_code_line(rest, lang));
            break;
        };
        let after = &rest[start + 2..];
        let Some(len) = after.find("]]") else {
            html.push_str(&highlight_code_line(rest, lang));
            break;
        };
        html.push_str(&highlight_code_line(&rest[..start], lang));
        let inner = &after[..len];
        let (id, text) = match inner.split_once('|') {
            Some((id, text)) => (id, text),
            None => (inner, inner),
        };
        html.push_str(&format!(
            "<mark class=\"code-map__token\" data-anchor=\"{}.{}\">{}</mark>",
            escape_html(card_id),
            escape_html(id),
            escape_html(text)
        ));
        has_anchor = true;
        rest = &after[len + 2..];
    }

    (html, has_anchor)
}

impl ComponentStrategy for CodeMapData {
    fn required_assets(&self) -> (Vec<&'static str>, Vec<&'static str>) {
        (
            vec!["css/syntax.css", "css/code_map.css"],
            vec!["js/code_map.js"],
        )
    }

    fn template_name(&self) -> &'static str {
        "code_map"
    }

    fn render_context(&self, children_html: &str) -> Value {
        let cards: Vec<CodeMapCardView> = self
            .cards
            .iter()
            .map(|card| CodeMapCardView {
                id: &card.id,
                x: card.x,
                y: card.y,
                width: card.width,
                height: card.height,
                title: card.title.as_deref(),
                lines: card
                    .code
                    .lines()
                    .map(|line| {
                        let (html, hl) = render_line(line, &card.language, &card.id);
                        CodeMapLineView { html, hl }
                    })
                    .collect(),
            })
            .collect();

        context! {
            title => &self.title,
            width => &self.width,
            height => &self.height,
            groups => &self.groups,
            cards => cards,
            arrows => &self.arrows,
            children => children_html,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_line_wraps_anchor_in_mark() {
        let (html, hl) = render_line("this.[[startup]]();", "ts", "main");
        assert!(hl);
        assert!(html.contains(
            "<mark class=\"code-map__token\" data-anchor=\"main.startup\">startup</mark>"
        ));
        assert!(!html.contains("[["));
    }

    #[test]
    fn render_line_supports_custom_anchor_id() {
        let (html, _) = render_line("await this.[[init|initServices]](env);", "ts", "boot");
        assert!(html.contains("data-anchor=\"boot.init\">initServices</mark>"));
    }

    #[test]
    fn render_line_without_anchor_is_not_highlighted() {
        let (html, hl) = render_line("const x = 1;", "ts", "main");
        assert!(!hl);
        assert!(html.contains("tok-kw"));
        assert!(!html.contains("<mark"));
    }

    #[test]
    fn render_line_leaves_unterminated_marker_as_code() {
        let (html, hl) = render_line("weird [[ stuff", "ts", "main");
        assert!(!hl);
        assert!(html.contains("[["));
    }
}
