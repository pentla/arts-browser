use crate::css::ast::Property;

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
