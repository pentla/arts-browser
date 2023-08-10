use crate::css::Color;
use crate::font::FontMetrics;
use crate::paint::DisplayCommand;
use std::iter::repeat;

#[derive(Debug)]
pub struct Canvas {
    pub pixels: Vec<Color>,
    pub width: usize,
    pub height: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let white = Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        };
        return Canvas {
            pixels: repeat(white).take(width * height).collect(),
            width,
            height,
        };
    }
    pub fn paint_item(&mut self, item: &DisplayCommand) {
        match item {
            &DisplayCommand::SolidColor(color, rect) => {
                // clamp: 数値の最小値、最大値を指定するとその範囲内に収めてくれる
                let x0 = rect.x.clamp(0.0, self.width as f32) as usize;
                let y0 = rect.y.clamp(0.0, self.height as f32) as usize;
                let x1 = (rect.x + rect.width).clamp(0.0, self.width as f32) as usize;
                let y1 = (rect.y + rect.height).clamp(0.0, self.height as f32) as usize;

                for y in y0..y1 {
                    for x in x0..x1 {
                        self.pixels[y * self.width + x] = color;
                    }
                }
            }

            DisplayCommand::FontSubpixel(color, metrics, bitmap) => {
                for y in 0..metrics.height as usize {
                    // x: 0, 3, 6, 9.....になる
                    for mut x in (0..(metrics.width as usize * 3)).step_by(3) {
                        let char_r = bitmap[x + y * metrics.width as usize * 3];
                        let char_g = bitmap[x + 1 + y * metrics.width as usize * 3];
                        let char_b = bitmap[x + 2 + y * metrics.width as usize * 3];
                        let char_a = char_r.max(char_g).max(char_b);
                        // fontデバッグ用
                        // print!("\x1B[48;2;{};{};{}m   ", char_r, char_g, char_b);

                        // subpixelしているため、実際のx座標は3倍になっている
                        x = x / 3;

                        let pixel_index = get_font_pixel_index(x, y, self.width, metrics);

                        let font_color = Color::from_rgba(char_r, char_g, char_b, char_a);
                        let background_color = self.pixels[pixel_index];

                        // fontの色と背景色を混ぜた値を背景色として設定する
                        let blended_color = font_color.blend(background_color, char_a);
                        self.pixels[pixel_index] = blended_color;
                    }
                    // fontデバッグ用
                    // println!("\x1B[0m");
                }
            }
        }
    }
}

/*
   pixelのindex =
   {y(縦) + metrics.y(縦のbounding box) * width(行数分y方向にずらす)}
   + {x(横) + metrics.x(横のbounding box)}
*/
pub fn get_font_pixel_index(x: usize, y: usize, width: usize, metrics: &FontMetrics) -> usize {
    // 参照: https://github.com/mooman219/fontdue/issues/10#issuecomment-603459057
    let x_index: i32 = (x as f32 + metrics.x) as i32;
    let y_index: i32 = (y as f32 + metrics.y - metrics.height as f32 - metrics.ymin as f32) as i32;
    y_index as usize * width + x_index as usize
}
