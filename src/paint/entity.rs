use crate::css::Color;
use crate::layout::Rect;

pub type DisplayList = Vec<DisplayCommand>;

#[derive(Debug)]
pub enum DisplayCommand {
    SolidColor(Color, Rect),
    FontSubpixel(Color, Rect, Vec<u8>),
}
