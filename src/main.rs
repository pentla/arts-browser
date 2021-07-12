extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "html.pest"]
pub struct HTMLParser;

fn main() {}

#[test]
fn html() {
    let parse1 = HTMLParser::parse(Rule::elementName, "a").unwrap();
    for inner1 in parse1.into_iter() {
        println!("{:?}", inner1);
    }

    let parse2 = HTMLParser::parse(Rule::element, "<div>").unwrap();
    for inner2 in parse2.into_iter() {
        println!("{:?}", inner2);
        assert_eq!(inner2.as_str(), "div")
    }
}
