use crate::css::Color;
use crate::layout::{LayoutBox, Rect};
use crate::paint::{build_display_list, DisplayCommand};
use std::iter::repeat;

#[derive(Debug)]
pub struct Canvas {
    pub pixels: Vec<Color>,
    pub width: usize,
    pub height: usize,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Canvas {
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
    fn paint_item(&mut self, item: &DisplayCommand) {
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
            DisplayCommand::Font(color, metrics, bitmap) => {
                for y in 0..metrics.height as usize {
                    for x in 0..metrics.width as usize * 3 {
                        let char_r = bitmap[x + y * metrics.width as usize * 3];
                        let char_g = bitmap[x + y * metrics.width as usize * 3];
                        let char_b = bitmap[x + y * metrics.width as usize * 3];
                        // fontデバッグ用
                        // print!("\x1B[48;2;{};{};{}m   ", char_r, char_g, char_b);

                        /*
                           pixelのindex =
                           {y(縦) + metrics.y(縦のbounding box) * width(行数分y方向にずらす)}
                           + {x(横) + metrics.x(横のbounding box)}
                        */
                        let pixel_index =
                            (y + metrics.y as usize) * self.width + (x + metrics.x as usize);
                        self.pixels[pixel_index] = Color::from_rgba(char_r, char_g, char_b, 255);
                    }
                    // fontデバッグ用
                    // println!("\x1B[0m");
                }
            }
        }
    }
}

pub fn paint(layout_root: &LayoutBox, bounds: Rect) -> Canvas {
    let display_list = build_display_list(layout_root);
    // println!("{:?}", display_list);
    let mut canvas = Canvas::new(bounds.width as usize, bounds.height as usize);
    for item in display_list {
        canvas.paint_item(&item);
    }
    canvas
}
