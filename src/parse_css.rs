use pest::iterators::Pair;
use pest::Parser;

use crate::ast_css::Block;

#[derive(Parser)]
#[grammar = "css.pest"]
pub struct CSSParser;

fn parse_css(input: &str) -> Block {
    let parser = CSSParser::parse(Rule::block, input).unwrap();
    let mut block = Block::new();
    for line in parser.into_iter() {
        match line.as_rule() {
            Rule::block => {
                block = parse_style_block(line);
            }
            _ => {}
        }
    }
    block
}

fn parse_style_block(rule: Pair<Rule>) -> Block {
    let mut block = Block::new();
    block.set_selector(rule.as_str());
    block
}

#[test]
fn test_css_parse() {
    let result = parse_css("a {}");
    println!("{:?}", result);
    assert_eq!(false, true);
}
