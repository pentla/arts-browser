use super::entity::DisplayList;
use crate::css::color::Color;
use crate::layout::{LayoutBox, Rect};
use crate::paint::entity::DisplayCommand;
use crate::paint::utils::{get_color, get_element_data, get_text};

extern crate fontdue;

fn generate_font(charactor: char, size: f32) -> (fontdue::Metrics, Vec<u8>) {
    let font_file = include_bytes!("../../resources/Roboto-Regular.ttf") as &[u8];
    let font = fontdue::Font::from_bytes(font_file, fontdue::FontSettings::default()).unwrap();
    let (metrics, bitmap) = font.rasterize_subpixel(charactor, size);
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
    let char = text.chars().next().unwrap();
    // FIXME: 複数文字を扱えるようにする
    let (metrics, bitmap) = generate_font(char, 12.0);
    // FIXME: SolidColorだと単色の色になってしまうので、フォントを表示できるようなDisplayCommandを開発する
    list.push(DisplayCommand::SolidColor(
        color,
        Rect {
            x: border_box.x,
            y: border_box.y,
            width: metrics.width as f32,
            height: metrics.height as f32,
        },
    ));
}

// テスト用
fn _print_font(metrics: fontdue::Metrics, bitmap: Vec<u8>) {
    for y in 0..metrics.height {
        for x in (0..metrics.width * 3).step_by(3) {
            let char_r = bitmap[x + y * metrics.width * 3];
            let char_g = bitmap[x + y * metrics.width * 3];
            let char_b = bitmap[x + y * metrics.width * 3];
            print!("\x1B[48;2;{};{};{}m   ", char_r, char_g, char_b);
        }
        println!("\x1B[0m");
    }
}

#[test]
fn test_font() {
    let (metrics, bitmap) = generate_font('g', 17.0);
    _print_font(metrics, bitmap);
}
