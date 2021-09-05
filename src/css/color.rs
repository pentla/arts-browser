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

#[test]
fn color_test() {
    let test1 = Color::new("#dedede").unwrap();
    assert_eq!(test1.r, 222);
    assert_eq!(test1.g, 222);
    assert_eq!(test1.b, 222);
    assert_eq!(test1.a, 1);
}
