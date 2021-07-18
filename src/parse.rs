use pest::iterators::Pair;
use pest::Parser;

use crate::ast::{Element, ElementType};

#[derive(Parser)]
#[grammar = "html.pest"]
pub struct HTMLParser;

fn parse_elements(input: &str) -> Vec<Element> {
    let parser = HTMLParser::parse(Rule::element, input).unwrap();
    let mut elements: Vec<Element> = Vec::new();
    for line in parser.into_iter() {
        match line.as_rule() {
            Rule::element => {
                let inner_rule = line.into_inner().next().unwrap();
                let element = parse_element(inner_rule);
                elements.push(element);
            }
            _ => {
                println!("other: {:?}", line);
            }
        }
    }
    elements
}

fn parse_element(rule: Pair<Rule>) -> Element {
    let element_name = rule.as_str();
    Element::new(element_name.to_string())
}

#[test]
fn test_parse() {
    let result1 = parse_elements("<div></div>");
    assert_eq!(result1[0].name, ElementType::Div);

    // let result2 = parse_elements("<div>text</div>");
    // assert_eq!(result2[0].name, ElementType::Div);
    // assert_eq!(result2[0].children[0].name, ElementType::Text);
}
