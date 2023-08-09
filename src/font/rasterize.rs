// fontdue::Fontの初期化をする
pub fn init_fontdue() -> fontdue::Font {
    let font_file = include_bytes!("../../resources/Roboto-Regular.ttf") as &[u8];
    fontdue::Font::from_bytes(font_file, fontdue::FontSettings::default()).unwrap()
}

pub fn generate_font(
    font_cli: &fontdue::Font,
    charactor: char,
    size: f32,
) -> (fontdue::Metrics, Vec<u8>) {
    let (metrics, bitmap) = font_cli.rasterize_subpixel(charactor, size * 2.0);
    (metrics, bitmap)
}
