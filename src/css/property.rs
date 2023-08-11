#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Property {
    Color,
    BackgroundColor,
    Margin,
    MarginTop,
    MarginLeft,
    MarginRight,
    MarginBottom,
    Padding,
    Width,
    Height,
    Display,
    FontSize,
    Undefined,
}

impl Property {
    pub fn to_string(self: &Self) -> String {
        property_to_string(*self)
    }
}

pub fn property_type(input: &str) -> Property {
    match input {
        "padding" => Property::Padding,
        "margin" => Property::Margin,
        "margin-top" => Property::MarginTop,
        "margin-left" => Property::MarginLeft,
        "margin-right" => Property::MarginRight,
        "margin-bottom" => Property::MarginBottom,
        "color" => Property::Color,
        "background-color" => Property::BackgroundColor,
        "width" => Property::Width,
        "height" => Property::Height,
        "font-size" => Property::FontSize,
        "display" => Property::Display,
        _ => Property::Undefined,
    }
}

pub fn property_to_string(input: Property) -> String {
    let result = match input {
        Property::Padding => "padding",
        Property::Margin => "margin",
        Property::MarginTop => "margin-top",
        Property::MarginLeft => "margin-left",
        Property::MarginRight => "margin-right",
        Property::MarginBottom => "margin-bottom",
        Property::Color => "color",
        Property::BackgroundColor => "background-color",
        Property::Width => "width",
        Property::Height => "height",
        Property::FontSize => "font-size",
        Property::Display => "display",
        Property::Undefined => "undefined",
    };
    result.to_string()
}
