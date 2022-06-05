use crate::layout::entity::{BoxType, Dimensions};
use crate::style::StyledNode;

#[derive(Debug)]
pub struct LayoutBox<'a> {
    pub dimensions: Dimensions,
    pub box_type: BoxType<'a>,
    pub children: Vec<LayoutBox<'a>>,
}

impl<'a> LayoutBox<'a> {
    pub fn new(box_type: BoxType) -> LayoutBox {
        LayoutBox {
            box_type,
            dimensions: Default::default(),
            children: vec![],
        }
    }
    pub fn get_inline_container(&mut self) -> &mut LayoutBox<'a> {
        match self.box_type {
            BoxType::InlineNode(_) | BoxType::AnonymouseBlock => self,
            BoxType::BlockNode(_) => {
                match self.children.last() {
                    Some(&LayoutBox {
                        box_type: BoxType::AnonymouseBlock,
                        ..
                    }) => {}
                    _ => self.children.push(LayoutBox::new(BoxType::AnonymouseBlock)),
                }
                self.children.last_mut().unwrap()
            }
        }
    }
    pub fn get_style_node(&self) -> &'a StyledNode<'a> {
        match self.box_type {
            BoxType::BlockNode(style_node) => style_node,
            BoxType::InlineNode(style_node) => style_node,
            BoxType::AnonymouseBlock => panic!("Anonymous block box has no style node"),
        }
    }
    pub fn layout(&mut self, containing_block: Dimensions) {
        match self.box_type {
            BoxType::BlockNode(_) => self.layout_block(containing_block),
            BoxType::InlineNode(_) => self.layout_inline(containing_block),
            BoxType::AnonymouseBlock => self.layout_anonymous_block(containing_block),
        }
    }

    // FIXME: inline-block独自の位置調整をする
    fn layout_inline(&mut self, containing_block: Dimensions) {
        self.layout_block(containing_block);
    }
}
