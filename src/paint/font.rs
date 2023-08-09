use super::entity::DisplayList;
use crate::css::Color;
use crate::font::rasterize::{generate_font, init_fontdue};
use crate::font::FontMetrics;
use crate::layout::{LayoutBox, Rect};
use crate::paint::entity::DisplayCommand;
use crate::paint::utils::{get_color, get_text};

extern crate fontdue;

pub fn render_font_subpixel(list: &mut DisplayList, layout_box: &LayoutBox) {
    let text = get_text(layout_box);
    if text == "" {
        return;
    }
    let color = match get_color(layout_box, "color") {
        Some(color) => color,
        _ => Color::new("black").unwrap(),
    };
    let d = &layout_box.dimensions;
    let border_box = d.border_box();

    let font_cli = init_fontdue();

    // 文字のX座標はfontを描画するたび、その長さずつ右にずれていく
    let mut text_position_x = border_box.x;
    let text_position_y = border_box.y;
    for char in text.chars() {
        let (metrics, bitmap) = generate_font(&font_cli, char, 12.0);
        list.push(DisplayCommand::FontSubpixel(
            color,
            FontMetrics {
                x: text_position_x,
                y: text_position_y,
                xmin: metrics.xmin,
                ymin: metrics.ymin,
                width: metrics.width,
                height: metrics.height,
            },
            bitmap,
        ));
        text_position_x += metrics.width as f32;
    }
}
