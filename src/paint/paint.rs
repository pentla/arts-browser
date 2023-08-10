use crate::layout::{LayoutBox, Rect};
use crate::paint::background::render_background;
use crate::paint::border::render_borders;
use crate::paint::canvas::Canvas;
use crate::paint::entity::DisplayList;
use crate::paint::font::render_font_subpixel;

pub fn build_display_list(layout_root: &LayoutBox) -> DisplayList {
    let mut list = Vec::new();
    render_layout_box(&mut list, layout_root);
    list
}

fn render_layout_box(list: &mut DisplayList, layout_box: &LayoutBox) {
    render_background(list, layout_box);
    render_borders(list, layout_box);
    render_font_subpixel(list, layout_box);

    for child in &layout_box.children {
        render_layout_box(list, child);
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
