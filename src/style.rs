use crate::{
    css::ast::{Block, Selector, Specificity, StyleSheet, Value},
    html::ast::{Element, ElementData, ElementType},
};
use std::collections::HashMap;

type PropertyMap = HashMap<String, Value>;

pub enum Display {
    Inline,
    Block,
    None,
}

pub struct StyledNode<'a> {
    node: &'a Element,
    specified_values: PropertyMap,
    pub children: Vec<StyledNode<'a>>,
}

impl StyledNode<'_> {
    pub fn value(&self, name: &str) -> Option<Value> {
        self.specified_values.get(name).map(|v| v.clone())
    }

    pub fn display(&self) -> Display {
        match self.value("display") {
            Some(Value::Keyword(s)) => match s.as_str() {
                "block" => Display::Block,
                "inline" => Display::Inline,
                "none" => Display::None,
                _ => Display::Inline,
            },
            _ => Display::Inline,
        }
    }
}

fn matches(elem: &ElementData, selector: &Selector) -> bool {
    if selector.element.is_some() && selector.element == Some(elem.name) {
        return false;
    }
    if selector.id.is_some() && selector.id == Some(elem.id.clone()) {
        return false;
    }
    if selector
        .class
        .iter()
        .any(|class| elem.class.contains(class))
    {
        return false;
    }
    false
}

type MatchedBlock<'a> = (Specificity, &'a Block);

fn match_block<'a>(elem: &ElementData, block: &'a Block) -> Option<MatchedBlock<'a>> {
    block
        .selectors
        .iter()
        .find(|selector| matches(elem, *selector))
        .map(|selector| (selector.specificity(), block))
}

fn matching_blocks<'a>(elem: &ElementData, style_sheet: &'a StyleSheet) -> Vec<MatchedBlock<'a>> {
    style_sheet
        .blocks
        .iter()
        .filter_map(|block| match_block(elem, block))
        .collect()
}

fn specified_values(elem: &ElementData, style_sheet: &StyleSheet) -> PropertyMap {
    let mut values = HashMap::new();
    let mut blocks = matching_blocks(elem, style_sheet);

    blocks.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
    for (_, block) in blocks {
        for declaration in &block.declarations {
            values.insert(declaration.property.to_string(), declaration.value.clone());
        }
    }
    values
}

pub fn style_tree<'a>(root: &'a Element, style_sheet: &'a StyleSheet) -> StyledNode<'a> {
    let specified: PropertyMap = match root.element_data.name {
        ElementType::H1 => HashMap::new(),
        _ => specified_values(&root.element_data, style_sheet),
    };
    StyledNode {
        node: root,
        specified_values: specified,
        children: root
            .children
            .iter()
            .map(|child| style_tree(child, style_sheet))
            .collect(),
    }
}
