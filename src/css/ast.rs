use crate::css::color::Color;
use crate::css::property::{property_type, Property};
use crate::html::{element_type, ElementType};
use anyhow::Result;

#[derive(Debug)]
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
    pub class: Vec<String>,
    pub id: Option<String>,
    pub attribute: Option<String>,
}

pub type Specificity = (usize, usize, usize);

impl Selector {
    pub fn new() -> Selector {
        Selector {
            element: None,
            class: vec![],
            id: None,
            attribute: None,
        }
    }
    pub fn specificity(&self) -> Specificity {
        let a = self.id.iter().count();
        let b = self.class.len();
        let c = self.element.iter().count();
        (a, b, c)
    }
    pub fn set_class(self: &mut Self, input: &str) {
        self.class.push(input.to_string());
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
        let value: Value = match property {
            Property::Padding => {
                let px = get_px(val);
                match px {
                    Ok(px) => Value::Length(px, Unit::Px),
                    Err(err) => Value::Keyword(val.to_string()),
                }
            }
            Property::Margin => {
                let px = get_px(val);
                match px {
                    Ok(px) => Value::Length(px, Unit::Px),
                    Err(err) => Value::Keyword(val.to_string()),
                }
            }
            Property::MarginTop => {
                let px = get_px(val);
                match px {
                    Ok(px) => Value::Length(px, Unit::Px),
                    Err(err) => Value::Keyword(val.to_string()),
                }
            }
            Property::MarginLeft => {
                let px = get_px(val);
                match px {
                    Ok(px) => Value::Length(px, Unit::Px),
                    Err(err) => Value::Keyword(val.to_string()),
                }
            }
            Property::MarginRight => {
                let px = get_px(val);
                match px {
                    Ok(px) => Value::Length(px, Unit::Px),
                    Err(err) => Value::Keyword(val.to_string()),
                }
            }
            Property::MarginBottom => {
                let px = get_px(val);
                match px {
                    Ok(px) => Value::Length(px, Unit::Px),
                    Err(err) => Value::Keyword(val.to_string()),
                }
            }
            Property::Display => Value::Keyword(val.to_string()),
            Property::Width => {
                let px = get_px(val).unwrap();
                Value::Length(px, Unit::Px)
            }
            Property::Height => {
                let px = get_px(val).unwrap();
                Value::Length(px, Unit::Px)
            }
            Property::FontSize => {
                let px = get_px(val).unwrap();
                Value::Length(px, Unit::Px)
            }
            Property::BackgroundColor => Value::Color(Color::new(val).unwrap()),
            Property::Color => Value::Color(Color::new(val).unwrap()),
            _ => Value::Undefined,
        };
        Ok(Declaration { property, value })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Keyword(String),
    Color(Color),
    Length(f32, Unit),
    Undefined,
}

impl Value {
    pub fn to_px(&self) -> f32 {
        match *self {
            Value::Length(f, Unit::Px) => f,
            _ => 0.0,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Unit {
    Px,
}
