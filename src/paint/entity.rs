use crate::css::Color;
use crate::font::{FontBitmap, FontMetrics};
use crate::layout::Rect;

pub type DisplayList = Vec<DisplayCommand>;

#[derive(Debug)]
pub enum DisplayCommand {
    SolidColor(Color, Rect),
    FontSubpixel(Color, FontMetrics, FontBitmap),
}
