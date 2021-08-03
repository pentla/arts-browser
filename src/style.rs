use crate::{
    css::ast::{Block, Selector, Specificity, StyleSheet, Value},
    html::ast::Element,
};
use std::collections::HashMap;

type PropertyMap = HashMap<String, Value>;

struct StyleNode<'a> {
    node: &'a Element,
    specified_values: PropertyMap,
    children: Vec<StyleNode<'a>>,
}

fn matches(elem: &Element, selector: &Selector) -> bool {
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

fn match_block<'a>(elem: &Element, block: &'a Block) -> Option<MatchedBlock<'a>> {
    block
        .selectors
        .iter()
        .find(|selector| matches(elem, *selector))
        .map(|selector| (selector.specificity(), block))
}

fn matching_blocks<'a>(elem: &Element, style_sheet: &'a StyleSheet) -> Vec<MatchedBlock<'a>> {
    style_sheet
        .blocks
        .iter()
        .filter_map(|block| match_block(elem, block))
        .collect()
}

fn specified_values(elem: &Element, style_sheet: &StyleSheet) -> PropertyMap {
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
