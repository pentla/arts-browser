use pest::Parser;

use crate::ast_css::Block;

#[derive(Parser)]
#[grammar = "css.pest"]
pub struct CSSParser;

fn parse_css(input: &str) {
    let parser = CSSParser::parse(Rule::block, input).unwrap();
    let mut block = Block::new();
}

#[test]
fn test_css_parse() {
    let result = CSSParser::parse(Rule::block, "a {}").unwrap();
    println!("{:?}", result);
    assert_eq!(false, true);
}
