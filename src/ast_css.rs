pub struct StyleSheet {
    pub blocks: Vec<Block>,
}

pub struct Block {
    pub selector: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

impl Block {
    pub fn new() -> Block {
        Block {
            selector: vec![],
            declarations: vec![],
        }
    }
}

pub struct Selector {}

pub struct Declaration {
    pub property: CSSProperty,
    pub value: Value,
}

pub enum CSSProperty {
    Color,
    BackgroundColor,
    Margin,
    Padding,
}

pub enum Value {
    Keyword(String),
    Color,
    Length(f32, Unit),
}

pub enum Unit {
    Px,
}

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
