use std::collections::BTreeSet;

use crate::models::block::Block;
use crate::models::ui_component::ComponentBlock;

pub struct AssetRegistry {
    pub stylesheets: BTreeSet<String>,
    pub scripts: BTreeSet<String>,
}

impl AssetRegistry {
    pub fn new() -> Self {
        Self {
            stylesheets: BTreeSet::new(),
            scripts: BTreeSet::new(),
        }
    }

    pub fn from_blocks(blocks: &[Block]) -> Self {
        let mut reg = Self::new();
        for block in blocks {
            reg.visit_block(block);
        }
        reg
    }

    pub fn with_theme(mut self, theme: &str) -> Self {
        if !theme.is_empty() {
            self.stylesheets.insert(format!("tokens/{}.css", theme));
        }
        self
    }

    pub fn with_base_assets(mut self) -> Self {
        self.stylesheets.insert("css/base.css".to_string());
        self
    }

    fn visit_block(&mut self, block: &Block) {
        match block {
            Block::Prose(_) => {}
            Block::Component(comp) => self.visit_component(comp),
        }
    }

    fn visit_component(&mut self, comp: &ComponentBlock) {
        let (css, js) = comp.component.required_assets();
        for path in css {
            self.stylesheets.insert(path.to_string());
        }
        for path in js {
            self.scripts.insert(path.to_string());
        }
        for child in &comp.children {
            self.visit_block(child);
        }
    }

    pub fn inline_styles(&self) -> String {
        let content = self.resolve_content(&self.stylesheets);
        if content.is_empty() {
            String::new()
        } else {
            format!("<style>\n{}</style>", content)
        }
    }

    pub fn inline_scripts(&self) -> String {
        let content = self.resolve_content(&self.scripts);
        if content.is_empty() {
            String::new()
        } else {
            format!("<script>\n{}</script>", content)
        }
    }

    fn resolve_content(&self, paths: &BTreeSet<String>) -> String {
        let mut content = String::new();
        for path in paths {
            if let Some(source) = resolve_asset(path) {
                content.push_str(source);
                content.push('\n');
            }
        }
        content
    }
}

fn resolve_asset(path: &str) -> Option<&'static str> {
    if cfg!(test) && path == "js/test.js" {
        return Some(include_str!("../assets/js/test.js"));
    }
    match path {
        "css/base.css" => Some(include_str!("../assets/css/base.css")),
        "css/prompt_box.css" => Some(include_str!("../assets/css/prompt_box.css")),
        "css/svg_canvas.css" => Some(include_str!("../assets/css/svg_canvas.css")),
        "css/triage_board.css" => Some(include_str!("../assets/css/triage_board.css")),
        "css/notice.css" => Some(include_str!("../assets/css/notice.css")),
        "css/card.css" => Some(include_str!("../assets/css/card.css")),
        "css/data_grid.css" => Some(include_str!("../assets/css/data_grid.css")),
        "css/timeline.css" => Some(include_str!("../assets/css/timeline.css")),
        "css/code_panel.css" => Some(include_str!("../assets/css/code_panel.css")),
        "css/code_map.css" => Some(include_str!("../assets/css/code_map.css")),
        "css/syntax.css" => Some(include_str!("../assets/css/syntax.css")),
        "js/code_map.js" => Some(include_str!("../assets/js/code_map.js")),
        "js/tabs.js" => Some(include_str!("../assets/js/tabs.js")),
        "css/board_layout.css" => Some(include_str!("../assets/css/board_layout.css")),
        "js/data_grid.js" => Some(include_str!("../assets/js/data_grid.js")),
        "js/triage_board.js" => Some(include_str!("../assets/js/triage_board.js")),
        "css/flowchart.css" => Some(include_str!("../assets/css/flowchart.css")),
        "js/flowchart.js" => Some(include_str!("../assets/js/flowchart.js")),
        "css/module_map.css" => Some(include_str!("../assets/css/module_map.css")),
        "tokens/clay-slate.css" => Some(include_str!("../assets/tokens/clay-slate.css")),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::components::prompt_box::PromptBoxData;
    use crate::models::components::triage_board::TriageBoardData;
    use crate::models::ui_component::{ComponentBlock, UiComponent};

    #[test]
    fn registry_empty_ast() {
        let reg = AssetRegistry::from_blocks(&[]);
        assert!(reg.stylesheets.is_empty());
        assert!(reg.scripts.is_empty());
    }

    #[test]
    fn registry_single_component() {
        let blocks = vec![Block::Component(ComponentBlock {
            component: UiComponent::PromptBox(PromptBoxData {
                label: "Test".to_string(),
                content: "Content".to_string(),
            }),
            children: vec![],
        })];
        let reg = AssetRegistry::from_blocks(&blocks);
        assert!(reg.stylesheets.contains("css/prompt_box.css"));
        assert!(reg.scripts.is_empty());
    }

    #[test]
    fn registry_nested_merge() {
        let blocks = vec![Block::Component(ComponentBlock {
            component: UiComponent::PromptBox(PromptBoxData {
                label: "Parent".to_string(),
                content: "Parent content".to_string(),
            }),
            children: vec![Block::Component(ComponentBlock {
                component: UiComponent::PromptBox(PromptBoxData {
                    label: "Child".to_string(),
                    content: "Child content".to_string(),
                }),
                children: vec![],
            })],
        })];
        let reg = AssetRegistry::from_blocks(&blocks);
        assert!(reg.stylesheets.contains("css/prompt_box.css"));
    }

    #[test]
    fn registry_cross_component_assets() {
        let blocks = vec![Block::Component(ComponentBlock {
            component: UiComponent::TriageBoard(TriageBoardData {
                eyebrow: "Sprint".to_string(),
                title: "Board".to_string(),
                subtitle: "Items".to_string(),
                hintline: "Drag".to_string(),
            }),
            children: vec![Block::Component(ComponentBlock {
                component: UiComponent::PromptBox(PromptBoxData {
                    label: "Note".to_string(),
                    content: "Text".to_string(),
                }),
                children: vec![],
            })],
        })];
        let reg = AssetRegistry::from_blocks(&blocks);
        assert!(reg.stylesheets.contains("css/triage_board.css"));
        assert!(reg.stylesheets.contains("css/prompt_box.css"));
        assert!(reg.scripts.contains("js/triage_board.js"));
    }

    #[test]
    fn registry_deduplication() {
        let blocks = vec![
            Block::Component(ComponentBlock {
                component: UiComponent::PromptBox(PromptBoxData {
                    label: "A".to_string(),
                    content: "A content".to_string(),
                }),
                children: vec![],
            }),
            Block::Component(ComponentBlock {
                component: UiComponent::PromptBox(PromptBoxData {
                    label: "B".to_string(),
                    content: "B content".to_string(),
                }),
                children: vec![],
            }),
        ];
        let reg = AssetRegistry::from_blocks(&blocks);
        let prompt_count = reg.stylesheets.iter().filter(|s| s.contains("prompt_box")).count();
        assert_eq!(prompt_count, 1);
    }

    #[test]
    fn registry_inline_contents() {
        let mut reg = AssetRegistry::new();
        reg.stylesheets.insert("css/base.css".to_string());
        let html = reg.inline_styles();
        assert!(html.contains("<style>"));
        assert!(html.contains("--ivory:"));
    }

    #[test]
    fn registry_inline_scripts() {
        let mut reg = AssetRegistry::new();
        reg.scripts.insert("js/test.js".to_string());
        let html = reg.inline_scripts();
        assert!(html.contains("<script>"));
        assert!(html.contains("// test script content"));
    }

    #[test]
    fn registry_with_theme() {
        let reg = AssetRegistry::new().with_theme("clay-slate");
        assert!(reg.stylesheets.contains("tokens/clay-slate.css"));
    }
}
