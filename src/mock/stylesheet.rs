use crate::css::ast::{Block, Declaration, Selector, StyleSheet};

pub struct StyleSheetMockOption {
    pub id: String,
    pub class: String,
    pub property: String,
    pub value: String,
}

impl StyleSheetMockOption {
    pub fn new() -> StyleSheetMockOption {
        StyleSheetMockOption {
            id: String::from(""),
            class: String::from(""),
            property: String::from(""),
            value: String::from(""),
        }
    }
}

pub fn gen_mock_stylesheet(option: StyleSheetMockOption) -> StyleSheet {
    let mut style_sheet = StyleSheet::new();
    let mut block = Block::new();
    let mut selector = Selector::new();
    if !option.id.is_empty() {
        selector.id = Some(option.id);
    };
    if !option.class.is_empty() {
        selector.set_class(option.class.as_str());
    }
    if !option.property.is_empty() && !option.value.is_empty() {
        let declaration =
            Declaration::new(option.property.as_str(), option.value.as_str()).unwrap();
        block.declarations.push(declaration);
    }
    block.selectors.push(selector);
    style_sheet.blocks.push(block);
    style_sheet
}
