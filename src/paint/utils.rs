use crate::css::ast::Value;
use crate::css::color::Color;
use crate::html::ast::ElementData;
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

pub fn get_text<'a>(layout_box: &'a LayoutBox) -> &'a str {
    match layout_box.box_type {
        BoxType::BlockNode(style) | BoxType::InlineNode(style) => {
            style.node.element_data.text.as_str()
        }
        BoxType::AnonymouseBlock => "",
    }
}

pub fn get_element_data(layout_box: &LayoutBox) -> Option<ElementData> {
    match layout_box.box_type {
        BoxType::BlockNode(style) | BoxType::InlineNode(style) => {
            Some(style.node.element_data.clone())
        }
        BoxType::AnonymouseBlock => None,
    }
}
