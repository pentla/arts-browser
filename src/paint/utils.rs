use crate::css::ast::Value;
use crate::css::color::Color;
use crate::layout::{BoxType, LayoutBox};

pub fn get_color(layout_box: &LayoutBox, name: &str) -> Option<Color> {
    match layout_box.box_type {
        BoxType::BlockNode(style) | BoxType::InlineNode(style) => match style.value(name) {
            Some(Value::Color(color)) => Some(color),
            _ => None,
        },
        BoxType::AnonymouseBlock => None,
    }
}

pub fn get_text(layout_box: &LayoutBox) -> String {
    match layout_box.box_type {
        BoxType::BlockNode(style) | BoxType::InlineNode(style) => style.node.element_data.text,
        BoxType::AnonymouseBlock => String::from(""),
    }
}
