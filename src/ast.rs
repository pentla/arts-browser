#[derive(Debug)]
pub struct Element {
    pub name: ElementType,
    pub text: String,
    pub children: Vec<Box<Element>>,
}

impl Element {
    pub fn new(name: String) -> Element {
        let elm_name: ElementType = match &*name {
            "html" => ElementType::HTML,
            "div" => ElementType::Div,
            "span" => ElementType::Span,
            "text" => ElementType::Text,
            "error" => ElementType::Error,
            _ => ElementType::Other,
        };
        Element {
            name: elm_name,
            text: String::from(""),
            children: vec![],
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ElementType {
    HTML,
    Div,
    Span,
    Text,
    Error,
    Other,
}
