use crate::ast_html::{element_type, ElementType};
use anyhow::Result;

pub struct StyleSheet {
    pub blocks: Vec<Block>,
}
impl StyleSheet {
    pub fn new() -> StyleSheet {
        StyleSheet { blocks: vec![] }
    }
    pub fn append_block(self: &mut Self, block: Block) {
        self.blocks.push(block);
    }
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
    pub fn new() -> Selector {
        Selector {
            element: None,
            class: None,
            id: None,
            attribute: None,
        }
    }
    pub fn set_class(self: &mut Self, input: &str) {
        self.class = Some(input.to_string());
    }
    pub fn set_id(self: &mut Self, input: &str) {
        self.id = Some(input.to_string());
    }
    pub fn set_element(self: &mut Self, input: &str) {
        self.element = Some(element_type(input));
    }
}

#[derive(Debug)]
pub struct Declaration {
    pub property: Property,
    pub value: Value,
}

fn get_px(input: &str) -> Result<f32> {
    if !input.contains("px") {
        return Err(anyhow::anyhow!("padding not px;"));
    }
    let remove_px = input.replace("px", "");
    let float: f32 = remove_px.parse().unwrap();
    Ok(float)
}

impl Declaration {
    pub fn new(prop: &str, val: &str) -> Result<Declaration> {
        let property = property_type(prop);
        let mut value = Value::Undefined;
        match property {
            Property::Padding => {
                let px = get_px(val);
                match px {
                    Ok(px) => value = Value::Length(px, Unit::Px),
                    Err(err) => value = Value::Keyword(val.to_string()),
                }
            }
            Property::Margin => {
                let px = get_px(val).unwrap();
                value = Value::Length(px, Unit::Px);
            }
            Property::Display => {
                value = Value::Keyword(val.to_string());
            }
            Property::Width => {
                let px = get_px(val).unwrap();
                value = Value::Length(px, Unit::Px);
            }
            Property::Height => {
                let px = get_px(val).unwrap();
                value = Value::Length(px, Unit::Px);
            }
            Property::FontSize => {
                let px = get_px(val).unwrap();
                value = Value::Length(px, Unit::Px);
            }
            Property::BackgroundColor => {
                // FIX: colorに修正
                value = Value::Keyword(val.to_string());
            }
            _ => {}
        }
        Ok(Declaration { property, value })
    }
}

#[derive(Debug, PartialEq)]
pub enum Property {
    Color,
    BackgroundColor,
    Margin,
    Padding,
    Width,
    Height,
    Display,
    FontSize,
    Undefined,
}

fn property_type(input: &str) -> Property {
    match input {
        "padding" => Property::Padding,
        "margin" => Property::Margin,
        "color" => Property::Color,
        "background-color" => Property::BackgroundColor,
        "width" => Property::Width,
        "height" => Property::Height,
        "font-size" => Property::FontSize,
        "display" => Property::Display,
        _ => Property::Undefined,
    }
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Keyword(String),
    Color,
    Length(f32, Unit),
    Undefined,
}

#[derive(Debug, PartialEq)]
pub enum Unit {
    Px,
}

// pub struct Color {
//     r: u8,
//     g: u8,
//     b: u8,
//     a: u8,
// }
