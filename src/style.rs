use crate::css::ast::{Block, Selector, Specificity, StyleSheet, Unit, Value};
use crate::html::ast::{Element, ElementData, ElementType};
use crate::mock::element::{gen_mock_element, ElementMockOption};
use crate::mock::stylesheet::{gen_mock_stylesheet, StyleSheetMockOption};
use std::collections::HashMap;

type PropertyMap = HashMap<String, Value>;

pub enum Display {
    Inline,
    Block,
    None,
}

#[derive(Debug)]
pub struct StyledNode<'a> {
    pub node: &'a Element,
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
    pub fn lookup(&self, name: &str, fallback: &str, default: &Value) -> Value {
        self.value(name)
            .unwrap_or_else(|| self.value(fallback).unwrap_or_else(|| default.clone()))
    }
}

// 要素が一致するselectorを見つけたらtrue, そうでなければfalseを返す
fn exist_match_selector(elem: &ElementData, selector: &Selector) -> bool {
    if selector.element.is_some() && selector.element == Some(elem.name) {
        return true;
    }
    if selector.id.is_some() && selector.id == Some(elem.id.clone()) {
        return true;
    }
    if selector
        .class
        .iter()
        .any(|class| elem.class.contains(class))
    {
        return true;
    }
    false
}

type MatchedBlock<'a> = (Specificity, &'a Block);

fn match_block<'a>(elem: &ElementData, block: &'a Block) -> Option<MatchedBlock<'a>> {
    block
        .selectors
        .iter()
        .find(|selector| exist_match_selector(elem, *selector))
        .map(|selector| (selector.specificity(), block))
}

fn matching_blocks<'a>(elem: &ElementData, style_sheet: &'a StyleSheet) -> Vec<MatchedBlock<'a>> {
    style_sheet
        .blocks
        .iter()
        .filter_map(|block| match_block(elem, block))
        .collect()
}

fn get_property_map(elem: &ElementData, style_sheet: &StyleSheet) -> PropertyMap {
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
    // textにCSSを直接指定できない(親タグに付与する)ため、textの場合は処理をスキップ
    let specified: PropertyMap = match root.element_data.name {
        ElementType::Text => HashMap::new(),
        _ => get_property_map(&root.element_data, style_sheet),
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

#[test]
fn test_property_map() {
    // id: test1, width: 20pxにして、propertyMapが正常に作られているか
    let mut elem_option = ElementMockOption::new();
    elem_option.id = String::from("test1");
    let elem = gen_mock_element(elem_option);
    let mut stylesheet_option = StyleSheetMockOption::new();
    stylesheet_option.id = String::from("test1");
    stylesheet_option.property = String::from("width");
    stylesheet_option.value = String::from("20px");
    let style_sheet = gen_mock_stylesheet(stylesheet_option);
    let property_map = get_property_map(&elem.element_data, &style_sheet);

    let width = String::from("width");
    assert_eq!(
        *property_map.get(&width).unwrap(),
        Value::Length(20.0, Unit::Px)
    );
}

#[test]
fn test_exist_match_selector() {
    let test_element = ElementData {
        name: ElementType::Div,
        text: String::from("hello"),
        id: String::from("test_element"),
        class: String::from("test"),
    };
    let mut selector = Selector::new();

    // class, idの指定がない場合
    let test0 = exist_match_selector(&test_element, &selector);
    assert_eq!(test0, false);

    // 一致するclassがある場合
    selector.class = vec![String::from("test")];
    let test1 = exist_match_selector(&test_element, &selector);
    assert_eq!(test1, true);

    // 一致するclassと一致しないclassがある場合
    selector.class = vec![String::from("test"), String::from("q")];
    let test2 = exist_match_selector(&test_element, &selector);
    assert_eq!(test2, true);

    // idが一致する場合
    selector.class = vec![];
    selector.id = Some(String::from("test_element"));
    let test3 = exist_match_selector(&test_element, &selector);
    assert_eq!(test3, true);
}
