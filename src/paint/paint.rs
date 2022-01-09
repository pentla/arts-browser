use crate::layout::layout::LayoutBox;
use crate::paint::background::render_background;
use crate::paint::border::render_borders;
use crate::paint::entity::DisplayList;
use crate::paint::font::render_fonts;

pub fn build_display_list(layout_root: &LayoutBox) -> DisplayList {
    let mut list = Vec::new();
    render_layout_box(&mut list, layout_root);
    list
}

fn render_layout_box(list: &mut DisplayList, layout_box: &LayoutBox) {
    render_background(list, layout_box);
    render_borders(list, layout_box);
    render_fonts(list, layout_box);

    for child in &layout_box.children {
        render_layout_box(list, child);
    }
}
