#[derive(Debug)]
pub struct Element {
    pub name: ElementType,
    pub children: Vec<Box<Element>>,
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
