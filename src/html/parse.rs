use pest::iterators::Pair;
use pest::Parser;

use crate::html::ast::{element_type, Element, ElementType};

#[derive(Parser)]
#[grammar = "html/html.pest"]
pub struct HTMLParser;

pub fn parse_nodes(input: &str) -> Element {
    let parser = HTMLParser::parse(Rule::html, input).unwrap();
    let mut element: Element = Element::new(String::from(""));
    for line in parser.into_iter() {
        match line.as_rule() {
            Rule::element => {
                element = parse_element(line);
            }
            _ => {}
        }
    }
    element
}

fn parse_element(rule: Pair<Rule>) -> Element {
    let mut element = Element::new(String::from(""));
    for item in rule.into_inner().into_iter() {
        match item.as_rule() {
            Rule::elementName => {
                if element.element_data.name == ElementType::Undefined {
                    element.element_data.name = element_type(item.as_str());
                }
            }
            Rule::element => {
                let child_element = parse_element(item);
                element.children.push(child_element);
            }
            Rule::text => {
                let mut text_element = Element::new(String::from("text"));
                text_element.element_data.text = item.as_str().to_string();
                element.children.push(text_element);
            }
            Rule::elementAttr => {
                let mut attr_name = "";
                let mut attr_value = "";
                for attribute in item.into_inner() {
                    match attribute.as_rule() {
                        Rule::attrName => {
                            attr_name = attribute.as_str();
                        }
                        Rule::attrValue => {
                            attr_value = attribute.as_str();
                        }
                        _ => {}
                    }
                }
                element.set_attr(attr_name, attr_value);
            }
            _ => {}
        }
    }
    element
}

#[test]
fn test_parse() {
    let result1 = parse_nodes("<div></div>");
    assert_eq!(result1.element_data.name, ElementType::Div);

    let result2 = parse_nodes("<div>hello</div>");
    assert_eq!(result2.element_data.name, ElementType::Div);
    assert_eq!(result2.children[0].element_data.name, ElementType::Text);

    let result2_1 = parse_nodes("<h1>hello</h1>");
    assert_eq!(result2_1.element_data.name, ElementType::H1);
    assert_eq!(result2_1.children[0].element_data.name, ElementType::Text);

    // div → span → text
    let result3 = parse_nodes("<div><span>text</span></div>");
    assert_eq!(result3.element_data.name, ElementType::Div);
    // 直下にspanがあるかどうか
    assert_eq!(result3.children.len(), 1);
    assert_eq!(result3.children[0].element_data.name, ElementType::Span);
    // spanのさらに下にtextがあるか
    assert_eq!(result3.children[0].children.len(), 1);
    assert_eq!(
        result3.children[0].children[0].element_data.name,
        ElementType::Text
    );

    // 改行を含む場合
    let result4 = parse_nodes(
        "
<div>
    <h1>Text</h1>
</div>
",
    );
    assert_eq!(result4.element_data.name, ElementType::Div);
    // // 直下にspanがあるかどうか
    assert_eq!(result4.children.len(), 1);
    assert_eq!(result4.children[0].element_data.name, ElementType::H1);
    // // spanのさらに下にtextがあるか
    assert_eq!(result4.children[0].children.len(), 1);
    assert_eq!(
        result4.children[0].children[0].element_data.name,
        ElementType::Text
    );

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
    assert_eq!(result5.element_data.name, ElementType::Html);
    assert_eq!(result5.children[0].element_data.name, ElementType::Body);
    assert_eq!(
        result5.children[0].children[0].element_data.name,
        ElementType::H1
    );
    assert_eq!(
        result5.children[0].children[1].element_data.name,
        ElementType::Div
    );
    assert_eq!(
        result5.children[0].children[1].children[0]
            .element_data
            .name,
        ElementType::P
    );

    // テキストの後に要素が続く場合
    let result6 = parse_nodes("<div>Hello<em>world</em>!</div>");
    assert_eq!(result6.element_data.name, ElementType::Div);
    assert_eq!(result6.children[0].element_data.name, ElementType::Text);
    assert_eq!(result6.children[0].element_data.text, "Hello");
    assert_eq!(result6.children[1].element_data.name, ElementType::Em);
    assert_eq!(result6.children[2].element_data.name, ElementType::Text);
    assert_eq!(result6.children[2].element_data.text, "!");

    // id, classのパース
    let result7 = parse_nodes(r#"<div id="text" class="hi">text</div>"#);
    assert_eq!(result7.element_data.name, ElementType::Div);
    assert_eq!(result7.element_data.id, "text");
    assert_eq!(result7.element_data.class, "hi");
    assert_eq!(result7.children[0].element_data.name, ElementType::Text);
    assert_eq!(result7.children[0].element_data.text, "text");
}

#[test]
fn html() {
    let parse1 = HTMLParser::parse(Rule::elementName, "div").unwrap();
    assert_eq!(parse1.as_str(), "div");

    let parse2 = HTMLParser::parse(Rule::elementName, "h1").unwrap();
    assert_eq!(parse2.as_str(), "h1");

    let parse3 = HTMLParser::parse(Rule::element, "<div></div>").unwrap();
    for line in parse3.into_iter() {
        match line.as_rule() {
            Rule::element => {
                let mut inner_rule = line.into_inner();
                let element_name = inner_rule.next().unwrap().as_str();
                assert_eq!(element_name, "div");
            }
            _ => {
                println!("{:?}", line);
            }
        }
    }

    let parse4 = HTMLParser::parse(Rule::element, "<div>a</div>").unwrap();
    for line4 in parse4.into_iter() {
        match line4.as_rule() {
            Rule::element => {
                let mut inner_rule = line4.into_inner();
                let element_name = inner_rule.next().unwrap().as_str();
                assert_eq!(element_name, "div");
            }
            _ => {
                println!("{:?}", line4);
            }
        }
    }
}
