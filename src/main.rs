extern crate pest;
#[macro_use]
extern crate pest_derive;

use clap::{load_yaml, App};
use image::{DynamicImage, ImageBuffer};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod css;
mod font;
mod html;
mod layout;
mod mock;
mod paint;
mod style;

use pest::Parser;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    let read_source = |arg_filename: Option<&str>, default_filename: &str| {
        let path = match arg_filename {
            Some(ref filename) => filename,
            None => default_filename,
        };
        let mut file = File::open(&Path::new(path)).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        content
    };
    let html = read_source(matches.value_of("html"), "examples/test.html");
    let css = read_source(matches.value_of("css"), "examples/test.css");

    let initial_containing_block = layout::Dimensions {
        content: layout::Rect {
            x: 0.0,
            y: 0.0,
            width: 800.0,
            height: 600.0,
        },
        padding: Default::default(),
        border: Default::default(),
        margin: Default::default(),
    };

    let root_node = html::parse_nodes(html.as_str());
    let stylesheet = css::parse_css(css.as_str());
    // println!("{:?}", stylesheet);
    let style_root = style::style_tree(&root_node, &stylesheet);
    // println!("{:?}", style_root);
    let layout_root = layout::layout_tree(&style_root, initial_containing_block);
    // println!("{:?}", layout_root);
    let canvas = paint::paint(&layout_root, initial_containing_block.content);
    let filename = matches.value_of("output").unwrap_or("output.png");
    let (w, h) = (canvas.width as u32, canvas.height as u32);
    let img = ImageBuffer::from_fn(w, h, move |x, y| {
        let color = canvas.pixels[(y * w + x) as usize];
        image::Rgb([color.r, color.g, color.b])
    });
    DynamicImage::ImageRgb8(img).save(filename).unwrap();
}
