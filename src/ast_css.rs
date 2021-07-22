use crate::ast_html::{element_type, ElementType};
use anyhow::Result;

pub struct StyleSheet {
    pub blocks: Vec<Block>,
}
#[derive(Debug)]
pub struct Block {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

impl Block {
    pub fn new() -> Block {
        Block {
            selectors: vec![],
            declarations: vec![],
        }
    }
    pub fn set_selector(self: &mut Self, input: Selector) {
        self.selectors.push(input);
    }
    pub fn set_declaration(self: &mut Self, declaration: Declaration) {
        self.declarations.push(declaration);
    }
}
#[derive(Debug)]
pub struct Selector {
    pub element: Option<ElementType>,
    pub class: Option<String>,
    pub id: Option<String>,
    pub attribute: Option<String>,
}

impl Selector {
    pub fn new(input: &str) -> Selector {
        let mut selector = Selector {
            element: None,
            class: None,
            id: None,
            attribute: None,
        };
        if !input.contains(".") {
            let first_char = input.chars().next().unwrap();
            match first_char {
                '.' => {
                    selector.class = Some(input.replace(".", "").to_string());
                }
                '#' => {
                    selector.id = Some(input.replace("#", "").to_string());
                }
                _ => selector.element = Some(element_type(input)),
            }
        }
        selector
    }
}

#[derive(Debug)]
pub struct Declaration {
    pub property: CSSProperty,
    pub value: Value,
}

impl Declaration {
    pub fn new(prop: &str, val: &str) -> Result<Declaration> {
        let property = property_type(prop);
        let mut value = Value::Undefined;
        match property {
            CSSProperty::Padding => {
                if !val.contains("px") {
                    return Err(anyhow::anyhow!("padding not px;"));
                }
                let remove_px = val.replace("px", "");
                let float: f32 = remove_px.parse().unwrap();
                value = Value::Length(float, Unit::Px);
            }
            _ => {}
        }
        Ok(Declaration { property, value })
    }
}

#[derive(Debug, PartialEq)]
pub enum CSSProperty {
    Color,
    BackgroundColor,
    Margin,
    Padding,
    Undefined,
}

fn property_type(input: &str) -> CSSProperty {
    match input {
        "padding" => CSSProperty::Padding,
        "margin" => CSSProperty::Margin,
        "color" => CSSProperty::Color,
        "background-color" => CSSProperty::BackgroundColor,
        _ => CSSProperty::Undefined,
    }
}

#[derive(Debug)]
pub enum Value {
    Keyword(String),
    Color,
    Length(f32, Unit),
    Undefined,
}

#[derive(Debug)]
pub enum Unit {
    Px,
}

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    // a: u8,
}
