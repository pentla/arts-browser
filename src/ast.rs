#[derive(Debug)]
pub struct Element {
    pub name: ElementType,
    pub children: Vec<Element>,
    pub text: String,
}

impl Element {
    pub fn new(name: String) -> Element {
        let elm_name: ElementType = match &*name {
            "html" => ElementType::HTML,
            "div" => ElementType::Div,
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
    Text,
    Error,
    Other,
}
