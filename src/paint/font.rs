use crate::layout::LayoutBox;

use super::entity::DisplayList;

extern crate fontdue;

fn generate_font(charactor: char, size: f32) -> (fontdue::Metrics, Vec<u8>) {
    let font_file = include_bytes!("../../resources/Roboto-Regular.ttf") as &[u8];
    let font = fontdue::Font::from_bytes(font_file, fontdue::FontSettings::default()).unwrap();
    let (metrics, bitmap) = font.rasterize_subpixel(charactor, size);
    (metrics, bitmap)
}

pub fn render_fonts(list: &mut DisplayList, layout_box: &LayoutBox) {
    list.push
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