use crate::css::color::Color;
use crate::layout::layout::Rect;

pub type DisplayList = Vec<DisplayCommand>;

#[derive(Debug)]
pub enum DisplayCommand {
    SolidColor(Color, Rect),
}
