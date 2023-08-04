use crate::css::Color;
use crate::layout::Rect;

pub type DisplayList = Vec<DisplayCommand>;

#[derive(Debug)]
pub enum DisplayCommand {
    SolidColor(Color, Rect),
    Font(Color, Rect, Vec<u8>),
    FontSubpixel(Color, Rect, Vec<u8>),
}
