use super::entity::DisplayList;
use crate::css::Color;
use crate::layout::{LayoutBox, Rect};
use crate::paint::entity::DisplayCommand;
use crate::paint::utils::{get_color, get_text};

extern crate fontdue;

fn generate_font(charactor: char, size: f32) -> (fontdue::Metrics, Vec<u8>) {
    let font_file = include_bytes!("../../resources/Roboto-Regular.ttf") as &[u8];
    let font = fontdue::Font::from_bytes(font_file, fontdue::FontSettings::default()).unwrap();
    let (metrics, bitmap) = font.rasterize(charactor, size);
    (metrics, bitmap)
}

pub fn render_fonts(list: &mut DisplayList, layout_box: &LayoutBox) {
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

    // 文字のX座標はfontを描画するたび、その長さずつ右にずれていく
    let mut text_position_x = border_box.x;
    let text_position_y = border_box.y;
    for char in text.chars() {
        let (metrics, bitmap) = generate_font(char, 12.0);
        list.push(DisplayCommand::Font(
            color,
            Rect {
                x: text_position_x,
                y: text_position_y,
                width: metrics.width as f32,
                height: metrics.height as f32,
            },
            bitmap,
        ));
        text_position_x += (metrics.width) as f32;
    }
}

// テスト用
fn _print_font(metrics: fontdue::Metrics, bitmap: Vec<u8>) {
    // Metrics { xmin: 0, ymin: -4, width: 9, height: 14, advance_width: 9.537598, advance_height: 0.0, bounds: OutlineBounds { xmin: 0.796875, ymin: -3.5361328, width: 7.586914, height: 12.683594 } }
    println!("{:?}", metrics);
    for y in 0..metrics.height {
        for x in 0..metrics.width {
            let char_s = bitmap[x + y * metrics.width];
            print!("\x1B[48;2;{};{};{}m   ", char_s, char_s, char_s);
        }
        println!("\x1B[0m");
    }
}

#[test]
fn test_font() {
    let (metrics, bitmap) = generate_font('g', 17.0);
    _print_font(metrics, bitmap);
}
