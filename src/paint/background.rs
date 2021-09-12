use crate::layout::{LayoutBox, Rect};
use crate::paint::entity::{DisplayCommand, DisplayList};
use crate::paint::utils::get_color;

pub fn render_background(list: &mut DisplayList, layout_box: &LayoutBox) {
    // FIXME: background-colorにしか対応していないので、background両方に対応させたい
    get_color(layout_box, "background-color").map(|color| {
        list.push(DisplayCommand::SolidColor(
            color,
            layout_box.dimensions.border_box(),
        ));
    });
}
