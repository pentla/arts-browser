use crate::layout::entity::Dimensions;
use crate::layout::layout_box::LayoutBox;

/*
AnonymousBlockとは、Block要素の直下にinline要素が来た場合に、
複数のInline要素をまとめて1つのBlockにするための概念的なBlockのこと
*/
impl<'a> LayoutBox<'a> {
    pub fn layout_anonymous_block(&mut self, containing_block: Dimensions) {
        self.set_anonymouse_block_position(containing_block);
        self.layout_block_children();
    }

    fn set_anonymouse_block_position(&mut self, containing_block: Dimensions) {
        let d = &mut self.dimensions;
        d.content.x = containing_block.content.x;
        d.content.y = containing_block.content.y;
    }
}
