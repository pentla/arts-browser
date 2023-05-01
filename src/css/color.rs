use regex::Regex;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(color_name: &str) -> Option<Self> {
        if color_name.chars().next().unwrap() == '#' {
            return Color::hex_to_rgba(color_name);
        }
        Color::default_color_name(color_name)
    }
    fn hex_to_rgba(mut hex_code: &str) -> Option<Self> {
        hex_code = remove_first_char(hex_code);
        let re = Regex::new(r"^([A-Fa-f0-9]{3}){1,2}$").unwrap();
        let caps = re.captures(hex_code);
        let mut hex = match caps {
            Some(cap) => cap.get(0).unwrap().as_str().to_string(),
            _ => return None,
        };
        if hex.chars().count() == 3 {
            let mut new_hex = String::from("");
            for char in hex.chars().into_iter() {
                new_hex.push(char);
                new_hex.push(char);
            }
            hex = new_hex;
        }
        Some(Color {
            r: hex_pair(&hex[0..2]),
            g: hex_pair(&hex[2..4]),
            b: hex_pair(&hex[4..6]),
            a: 1,
        })
    }
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    /// 他のColorを指定することで、その色とのブレンドを行う.
    /// alpha: どの程度文字の色と背景色が混ざるかを決定。 0~255の値を指定
    pub fn blend(&self, blended_color: Color, alpha: u8) -> Color {
        let alpha = alpha as f32 / 255.0;
        // 文字のアルファ値の補数を表し、背景色がどの程度透明であるか
        let inv_alpha = 1.0 - alpha;
        Color {
            r: (self.r as f32 * alpha + blended_color.r as f32 * inv_alpha).ceil() as u8,
            g: (self.g as f32 * alpha + blended_color.g as f32 * inv_alpha).ceil() as u8,
            b: (self.b as f32 * alpha + blended_color.b as f32 * inv_alpha).ceil() as u8,
            a: 255,
        }
    }

    fn default_color_name(name: &str) -> Option<Color> {
        match name {
            "black" => Some(Color {
                r: 0,
                g: 0,
                b: 0,
                a: 1,
            }),
            "gray" => Some(Color {
                r: 128,
                g: 128,
                b: 128,
                a: 1,
            }),
            "white" => Some(Color {
                r: 255,
                g: 255,
                b: 255,
                a: 1,
            }),
            "blue" => Some(Color {
                r: 0,
                g: 0,
                b: 255,
                a: 0,
            }),

            "green" => Some(Color {
                r: 0,
                g: 128,
                b: 0,
                a: 1,
            }),
            "yellow" => Some(Color {
                r: 255,
                g: 255,
                b: 0,
                a: 1,
            }),
            "red" => Some(Color {
                r: 255,
                g: 0,
                b: 0,
                a: 1,
            }),
            _ => None,
        }
    }
}

fn hex_pair(input: &str) -> u8 {
    u8::from_str_radix(input, 16).unwrap()
}

fn remove_first_char(input: &str) -> &str {
    let mut chars = input.chars();
    chars.next();
    chars.as_str()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_test() {
        let test1 = Color::new("#dedede").unwrap();
        assert_eq!(test1.r, 222);
        assert_eq!(test1.g, 222);
        assert_eq!(test1.b, 222);
        assert_eq!(test1.a, 1);
    }
    #[test]
    fn test_blend() {
        // 完全に透明な文字の場合、背景色がそのまま返される
        let char_color1 = Color::from_rgba(255, 0, 0, 255); // Red
        let bg_color1 = Color::from_rgba(0, 255, 0, 255); // Green
        let blended_color1 = char_color1.blend(bg_color1, 0);
        assert_eq!(bg_color1, blended_color1);

        // 完全に不透明な文字の場合、文字色がそのまま返される
        let char_color2 = Color::from_rgba(255, 0, 0, 255); // Red
        let bg_color2 = Color::from_rgba(0, 255, 0, 255); // Green
        let blended_color2 = char_color2.blend(bg_color2, 255);
        assert_eq!(char_color2, blended_color2);

        // 他の色と透明度の組み合わせでもテスト
        let char_color4 = Color::from_rgba(0, 0, 255, 255); // Blue
        let bg_color4 = Color::from_rgba(255, 255, 0, 255); // Yellow
        let blended_color4 = char_color4.blend(bg_color4, 64);
        let expected_color4 = Color::from_rgba(191, 191, 64, 255); // Blended: Light Gray
        assert_eq!(expected_color4, blended_color4);
    }
}
