use minijinja::{Value, context};
use serde::{Deserialize, Serialize};

use super::ComponentStrategy;
use crate::highlight::highlight_code_line;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CodeTab {
    pub name: String,
    pub language: String,
    #[serde(default)]
    pub diff: bool,
    pub content: String,
    pub risk: Option<String>,
    pub added: Option<u32>,
    pub removed: Option<u32>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CodePanelData {
    pub tabs: Vec<CodeTab>,
}

#[derive(Serialize)]
struct DiffLineView {
    kind: &'static str,
    mark: &'static str,
    html: String,
}

#[derive(Serialize)]
struct CodeTabView<'a> {
    name: &'a str,
    language: &'a str,
    diff: bool,
    risk: Option<&'a str>,
    added: Option<u32>,
    removed: Option<u32>,
    /// Syntax-highlighted lines for plain (non-diff) tabs, joined with `\n`.
    code_html: String,
    /// Per-line views for diff tabs; empty for plain tabs.
    diff_lines: Vec<DiffLineView>,
}

impl ComponentStrategy for CodePanelData {
    fn required_assets(&self) -> (Vec<&'static str>, Vec<&'static str>) {
        (
            vec!["css/syntax.css", "css/code_panel.css"],
            vec!["js/tabs.js"],
        )
    }

    fn template_name(&self) -> &'static str {
        "code_panel"
    }

    fn render_context(&self, children_html: &str) -> Value {
        let tabs: Vec<CodeTabView> = self
            .tabs
            .iter()
            .map(|tab| {
                let lang = tab.language.as_str();
                let (code_html, diff_lines) = if tab.diff {
                    let lines = tab
                        .content
                        .split('\n')
                        .map(|line| {
                            if let Some(rest) = line.strip_prefix('+') {
                                DiffLineView {
                                    kind: "add",
                                    mark: "+",
                                    html: highlight_code_line(rest, lang),
                                }
                            } else if let Some(rest) = line.strip_prefix('-') {
                                DiffLineView {
                                    kind: "del",
                                    mark: "-",
                                    html: highlight_code_line(rest, lang),
                                }
                            } else {
                                DiffLineView {
                                    kind: "ctx",
                                    mark: " ",
                                    html: highlight_code_line(line, lang),
                                }
                            }
                        })
                        .collect();
                    (String::new(), lines)
                } else {
                    let html = tab
                        .content
                        .split('\n')
                        .map(|line| highlight_code_line(line, lang))
                        .collect::<Vec<_>>()
                        .join("\n");
                    (html, Vec::new())
                };
                CodeTabView {
                    name: &tab.name,
                    language: lang,
                    diff: tab.diff,
                    risk: tab.risk.as_deref(),
                    added: tab.added,
                    removed: tab.removed,
                    code_html,
                    diff_lines,
                }
            })
            .collect();

        context! {
            tabs => tabs,
            children => children_html,
        }
    }
}
