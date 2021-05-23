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
            "body" => ElementType::Body,
            "div" => ElementType::Div,
            "span" => ElementType::Span,
            "p" => ElementType::P,
            "h1" => ElementType::H1,
            "h2" => ElementType::H2,
            "h3" => ElementType::H3,
            "h4" => ElementType::H4,
            "label" => ElementType::Label,
            "input" => ElementType::Input,
            "em" => ElementType::Em,
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
    Body,
    Div,
    Span,
    P,
    H1,
    H2,
    H3,
    H4,
    Em,
    Label,
    Input,
    Text,
    Error,
    Other,
}
