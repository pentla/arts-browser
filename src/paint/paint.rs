use crate::layout::LayoutBox;
use crate::paint::border::render_borders;
use crate::paint::entity::{DisplayCommand, DisplayList};
use crate::paint::utils::get_color;

pub fn build_display_list(layout_root: &LayoutBox) -> DisplayList {
    let mut list = Vec::new();
    render_layout_box(&mut list, layout_root);
    list
}

// 何を描画するかを決定する。
fn render_layout_box(list: &mut DisplayList, layout_box: &LayoutBox) {
    render_background(list, layout_box);
    render_borders(list, layout_box);

    for child in &layout_box.children {
        render_layout_box(list, child);
    }
}

fn render_background(list: &mut DisplayList, layout_box: &LayoutBox) {
    // FIXME: background-colorにしか対応していないので、background両方に対応させたい
    get_color(layout_box, "background-color").map(|color| {
        list.push(DisplayCommand::SolidColor(
            color,
            layout_box.dimensions.border_box(),
        ));
    });
}
