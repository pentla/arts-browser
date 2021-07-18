use pest::iterators::Pair;
use pest::Parser;

use crate::ast::{element_type, Element, ElementType};

#[derive(Parser)]
#[grammar = "html.pest"]
pub struct HTMLParser;

fn parse_nodes(input: &str) -> Element {
    let parser = HTMLParser::parse(Rule::html, input).unwrap();
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
            Rule::element => {
                let child_element = parse_element(item);
                element.children.push(Box::new(child_element));
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
    let result1 = parse_nodes("<div></div>");
    assert_eq!(result1.name, ElementType::Div);

    let result2 = parse_nodes("<div>hello</div>");
    assert_eq!(result2.name, ElementType::Div);
    assert_eq!(result2.children[0].name, ElementType::Text);

    let result2_1 = parse_nodes("<h1>hello</h1>");
    assert_eq!(result2_1.name, ElementType::H1);
    assert_eq!(result2_1.children[0].name, ElementType::Text);

    // div → span → text
    let result3 = parse_nodes("<div><span>text</span></div>");
    assert_eq!(result3.name, ElementType::Div);
    // 直下にspanがあるかどうか
    assert_eq!(result3.children.len(), 1);
    assert_eq!(result3.children[0].name, ElementType::Span);
    // spanのさらに下にtextがあるか
    assert_eq!(result3.children[0].children.len(), 1);
    assert_eq!(result3.children[0].children[0].name, ElementType::Text);

    // 改行を含む場合
    let result4 = parse_nodes(
        "
<div>
    <h1>Text</h1>
</div>
",
    );
    assert_eq!(result4.name, ElementType::Div);
    // // 直下にspanがあるかどうか
    assert_eq!(result4.children.len(), 1);
    assert_eq!(result4.children[0].name, ElementType::H1);
    // // spanのさらに下にtextがあるか
    assert_eq!(result4.children[0].children.len(), 1);
    assert_eq!(result4.children[0].children[0].name, ElementType::Text);

    // 兄弟要素がある場合
    let result5 = parse_nodes(
        "
    <html>
        <body>
            <h1>Title</h1>
            <div>
                <p>hello</p>
            </div>
        </body>
    </html>
    ",
    );
    assert_eq!(result5.name, ElementType::Html);
    assert_eq!(result5.children[0].name, ElementType::Body);
    assert_eq!(result5.children[0].children[0].name, ElementType::H1);
    assert_eq!(result5.children[0].children[1].name, ElementType::Div);
    assert_eq!(
        result5.children[0].children[1].children[0].name,
        ElementType::P
    );
}
