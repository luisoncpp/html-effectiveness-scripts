use super::ui_component::ComponentBlock;

#[derive(Debug, PartialEq)]
pub enum Block {
    Prose(String),
    Component(ComponentBlock),
}
