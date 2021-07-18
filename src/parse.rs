use pest::iterators::Pair;
use pest::Parser;

use crate::ast::{element_type, Element, ElementType};

#[derive(Parser)]
#[grammar = "html.pest"]
pub struct HTMLParser;

fn parse_elements(input: &str) -> Element {
    let parser = HTMLParser::parse(Rule::element, input).unwrap();
    let mut element: Element = Element::new(String::from(""));
    for line in parser.into_iter() {
        match line.as_rule() {
            Rule::element => {
                element = parse_element(line);
            }
            Rule::text => {
                // println!("{:?}", line);
                let inner_rule = line.into_inner().next().unwrap();
                let mut text_element = Element::new(String::from("text"));
                text_element.text = inner_rule.to_string();
                element.children.push(Box::new(text_element));
            }
            _ => {
                println!("other: {:?}", line);
            }
        }
    }
    element
}

fn parse_element(rule: Pair<Rule>) -> Element {
    let mut element = Element::new(String::from(""));
    for item in rule.into_inner().into_iter() {
        match item.as_rule() {
            Rule::elementName => {
                if element.name == ElementType::Undefined {
                    element.name = element_type(item.as_str());
                }
            }
            Rule::text => {
                let mut text_element = Element::new(String::from("text"));
                text_element.text = item.to_string();
                element.children.push(Box::new(text_element));
            }
            _ => {}
        }
    }
    element
}

#[test]
fn test_parse() {
    let result1 = parse_elements("<div></div>");
    assert_eq!(result1.name, ElementType::Div);

    let result2 = parse_elements("<div>hello</div>");
    assert_eq!(result2.name, ElementType::Div);
    assert_eq!(result2.children[0].name, ElementType::Text);
}
