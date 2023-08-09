// Font用のBitmap
pub type FontBitmap = Vec<u8>;

#[derive(Debug)]
pub struct FontMetrics {
    pub x: f32,
    pub y: f32,
    pub xmin: i32,
    pub ymin: i32,
    pub width: usize,
    pub height: usize,
}
