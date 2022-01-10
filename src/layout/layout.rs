use crate::layout::{BoxType, Dimensions, LayoutBox};
use crate::style::{Display, StyledNode};

pub fn layout_tree<'a>(node: &'a StyledNode, mut containing_block: Dimensions) -> LayoutBox<'a> {
    containing_block.content.height = 0.0;
    let mut root_box = build_layout_tree(node);
    root_box.layout(containing_block);
    root_box
}

fn build_layout_tree<'a>(style_node: &'a StyledNode) -> LayoutBox<'a> {
    let mut root = LayoutBox::new(match style_node.display() {
        Display::Block => BoxType::BlockNode(style_node),
        Display::Inline => BoxType::InlineNode(style_node),
        Display::None => panic!("Root node has display: none;"),
    });

    for child in &style_node.children {
        match child.display() {
            Display::Block => root.children.push(build_layout_tree(child)),
            Display::Inline => root
                .get_inline_container()
                .children
                .push(build_layout_tree(child)),
            Display::None => {}
        }
    }
    return root;
}
