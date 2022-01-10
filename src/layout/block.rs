use crate::css::{Unit, Value};
use crate::layout::entity::Dimensions;
use crate::layout::layout_box::LayoutBox;

impl<'a> LayoutBox<'a> {
    // widthは親コンポーネントから計算可能だが、高さは子要素の合計値に左右される
    pub fn layout_block(&mut self, containing_block: Dimensions) {
        // widthは親のコンポーネントから計算できる
        self.set_block_width(containing_block);
        // boxがどの位置にあるのかを計算する
        self.set_block_position(containing_block);
        // boxの子要素を再起的に計算する
        self.layout_block_children();
        // heightは子要素の高さに左右されるため、子要素の描画後でないと計算できない
        self.set_block_height();
    }

    pub fn set_block_width(&mut self, container_block: Dimensions) {
        let style = self.get_style_node();

        let auto = Value::Keyword("auto".to_string());
        let mut width = style.value("width").unwrap_or(auto.clone());

        let zero = Value::Length(0.0, Unit::Px);
        let mut margin_left = style.lookup("margin-left", "margin", &zero);
        let mut margin_right = style.lookup("margin-right", "margin", &zero);

        let border_left = style.lookup("border-left-width", "border-width", &zero);
        let border_right = style.lookup("border-right-width", "border-width", &zero);

        let padding_left = style.lookup("padding-left", "padding", &zero);
        let padding_right = style.lookup("padding-right", "padding", &zero);

        let total_width: f32 = [
            &margin_left,
            &margin_right,
            &border_left,
            &border_right,
            &padding_left,
            &padding_right,
        ]
        .iter()
        .map(|v| v.to_px())
        .sum();

        // widthがcontainerの大きさを超える場合、marginがautoに指定されているなら値を0にする
        if width != auto && total_width > container_block.content.width {
            if margin_left == auto {
                margin_left = Value::Length(0.0, Unit::Px);
            }
            if margin_right == auto {
                margin_right = Value::Length(0.0, Unit::Px);
            }
        }

        // containerの内容がwidthより大きくなってしまった場合の計算
        let underflow = container_block.content.width - total_width;

        match (width == auto, margin_left == auto, margin_right == auto) {
            // overconstrainedの場合
            (false, false, false) => {
                margin_left = Value::Length(margin_right.to_px() + underflow, Unit::Px);
            }

            // margin_right, margin_leftのどちらかが指定されている場合、underflowの値を入れる。
            (false, false, true) => margin_right = Value::Length(underflow, Unit::Px),
            (false, true, false) => margin_left = Value::Length(underflow, Unit::Px),

            // widthがautoの場合は、他の値は0になる
            (true, _, _) => {
                if margin_left == auto {
                    margin_left = Value::Length(0.0, Unit::Px);
                }
                if margin_right == auto {
                    margin_right = Value::Length(0.0, Unit::Px);
                }

                if underflow >= 0.0 {
                    width = Value::Length(underflow, Unit::Px);
                } else {
                    width = Value::Length(0.0, Unit::Px);
                    margin_right = Value::Length(margin_right.to_px() + underflow, Unit::Px);
                }
            }

            // margin-right, margin-leftの両方がautoの場合、underflowの値を半分にする
            (false, true, true) => {
                margin_left = Value::Length(underflow / 2.0, Unit::Px);
                margin_right = Value::Length(underflow / 2.0, Unit::Px);
            }
        }

        let d = &mut self.dimensions;
        d.content.width = width.to_px();
        d.padding.left = padding_left.to_px();
        d.padding.right = padding_right.to_px();

        d.border.left = border_left.to_px();
        d.border.right = border_right.to_px();

        d.margin.left = margin_left.to_px();
        d.margin.right = margin_right.to_px();
    }

    pub fn set_block_position(&mut self, containing_block: Dimensions) {
        let style = self.get_style_node();
        let d = &mut self.dimensions;

        let zero = Value::Length(0.0, Unit::Px);

        d.margin.top = style.lookup("margin-top", "margin", &zero).to_px();
        d.margin.bottom = style.lookup("margin-bottom", "margin", &zero).to_px();

        d.border.top = style.lookup("border-top", "border", &zero).to_px();
        d.border.bottom = style.lookup("border-bottom", "border", &zero).to_px();

        d.padding.top = style.lookup("padding-top", "padding", &zero).to_px();
        d.padding.bottom = style.lookup("padding-bottom", "padding", &zero).to_px();

        d.content.x = containing_block.content.x + d.margin.left + d.border.left + d.padding.left;
        // boxはすべてのblockの下に配置する
        d.content.y = containing_block.content.height
            + containing_block.content.y
            + d.margin.top
            + d.border.top
            + d.padding.top;
    }

    pub fn layout_block_children(&mut self) {
        let d = &mut self.dimensions;
        for child in &mut self.children {
            child.layout(*d);
            d.content.height = d.content.height + child.dimensions.margin_box().height;
        }
    }

    pub fn set_block_height(&mut self) {
        if let Some(Value::Length(h, Unit::Px)) = self.get_style_node().value("height") {
            self.dimensions.content.height = h;
        }
    }
}
