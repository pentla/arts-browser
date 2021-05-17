struct Element {
    name: ElementType,
    children: Vec<Element>,
}

impl Element {
    fn new(name: String) -> Element {
        let elm_name: ElementType = match &*name {
            "html" => ElementType::HTML,
            "div" => ElementType::Div,
            _ => ElementType::Other,
        };
        Element {
            name: elm_name,
            children: vec![],
        }
    }
}

pub enum ElementType {
    HTML,
    Div,
    Text,
    Other,
}
