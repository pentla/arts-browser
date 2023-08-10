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

#[derive(Debug)]
pub struct FontBounds {
    pub xmin: f32,
    pub ymin: f32,
    pub width: f32,
    pub height: f32,
}

impl FontMetrics {
    pub fn from_fontdue_metrics(x: f32, y: f32, metrics: fontdue::Metrics) -> Self {
        Self {
            x,
            y,
            xmin: metrics.xmin,
            ymin: metrics.ymin,
            width: metrics.width,
            height: metrics.height,
        }
    }
}
