#[derive(Debug)]
pub struct Element {
    pub name: ElementType,
    pub text: String,
    pub children: Vec<Box<Element>>,
    pub id: String,
    pub class: String,
}

impl Element {
    pub fn new(name: String) -> Element {
        let elm_name = element_type(&*name);
        Element {
            name: elm_name,
            children: vec![],
            text: String::from(""),
            id: String::from(""),
            class: String::from(""),
        }
    }
    pub fn set_attr(self: &mut Self, attrs: Vec<(String, String)>) {
        for attr in attrs {
            let key = &*attr.0;
            let value = attr.1;
            match key {
                "id" => self.id = value,
                "class" => self.class = value,
                _ => {}
            };
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ElementType {
    Html,
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
    Undefined,
}

pub fn element_type(name: &str) -> ElementType {
    match name {
        "html" => ElementType::Html,
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
        "" => ElementType::Undefined,
        _ => ElementType::Other,
    }
}
