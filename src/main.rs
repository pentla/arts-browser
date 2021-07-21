extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod ast_css;
pub mod ast_html;
pub mod parse_css;
pub mod parse_html;

use pest::Parser;

#[derive(Parser)]
#[grammar = "html.pest"]
pub struct HTMLParser;

fn main() {}

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
