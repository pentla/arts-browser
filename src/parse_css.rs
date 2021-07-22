use pest::iterators::Pair;
use pest::Parser;

use crate::{
    ast_css::{Block, CSSProperty, Declaration, Selector},
    ast_html::ElementType,
};

#[derive(Parser)]
#[grammar = "css.pest"]
pub struct CSSParser;

fn parse_block(input: &str) -> Block {
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
    for line in rule.into_inner().into_iter() {
        match line.as_rule() {
            Rule::selector => {
                let selector = Selector::new(line.as_str());
                block.set_selector(selector);
            }
            Rule::declaration => {
                let mut property: &str = "";
                let mut value: &str = "";
                for line_declaration in line.into_inner().into_iter() {
                    match line_declaration.as_rule() {
                        Rule::property => {
                            property = line_declaration.as_str();
                        }
                        Rule::value => {
                            value = line_declaration.as_str();
                        }
                        _ => {}
                    }
                }
                let declaration = Declaration::new(property, value).unwrap();
                block.set_declaration(declaration);
            }
            _ => {}
        }
    }
    block
}

#[test]
fn test_css_parse() {
    // selectorがparseできる
    let result1 = parse_block("a {}");
    assert_eq!(
        result1.selectors.get(0).unwrap().element,
        Some(ElementType::A)
    );

    // selectorとdeclarationがparseできる
    let result2 = parse_block("div { padding: 2px; }");
    assert_eq!(
        result2.selectors.get(0).unwrap().element,
        Some(ElementType::Div)
    );
    assert_eq!(
        result2.declarations.get(0).unwrap().property,
        CSSProperty::Padding
    );
}
