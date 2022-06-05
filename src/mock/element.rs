use crate::html::Element;

pub struct ElementMockOption {
    pub name: String,
    pub id: String,
    pub class: String,
}

impl ElementMockOption {
    pub fn new() -> ElementMockOption {
        ElementMockOption {
            name: String::from(""),
            id: String::from(""),
            class: String::from(""),
        }
    }
}

pub fn gen_mock_element(option: ElementMockOption) -> Element {
    let name: &str = if !option.name.is_empty() {
        option.name.as_str()
    } else {
        "div"
    };
    let id = if !option.id.is_empty() {
        option.id.as_str()
    } else {
        "mock"
    };
    let class = if !option.class.is_empty() {
        option.class.as_str()
    } else {
        "mock"
    };
    let mut elem = Element::new(String::from(name));
    elem.set_attr("class", class);
    elem.set_attr("id", id);
    elem
}
