use std::result;

use pest::iterators::Pair;
use pest::Parser;

use crate::{
    ast_css::{Block, Declaration, Property, Selector, Unit, Value},
    ast_html::{Element, ElementType},
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
                let mut selector = Selector::new();
                for selector_line in line.into_inner().into_iter() {
                    match selector_line.as_rule() {
                        // id, classは先頭の文字(. #)を取り除く
                        Rule::id => selector.set_id(&selector_line.as_str()[1..]),
                        Rule::class => selector.set_class(&selector_line.as_str()[1..]),
                        Rule::element => selector.set_element(selector_line.as_str().trim()),
                        _ => {
                            println!("other {:?}", selector_line);
                        }
                    }
                }
                block.set_selector(selector);
            }
            Rule::declaration => {
                let mut property: &str = "";
                let mut value: &str = "";
                for line_declaration in line.into_inner().into_iter() {
                    println!("{:?}", line_declaration);
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

    // classがparseできる
    let result1_1 = parse_block(".sample {}");
    assert_eq!(
        result1_1.selectors.get(0).unwrap().class,
        Some(String::from("sample"))
    );

    // // selectorとdeclarationがparseできる
    let result2 = parse_block("div { padding: 2px; }");
    assert_eq!(
        result2.selectors.get(0).unwrap().element,
        Some(ElementType::Div)
    );
    let dec2 = result2.declarations.get(0).unwrap();
    assert_eq!(dec2.property, Property::Padding);
    assert_eq!(dec2.value, Value::Length(2.0, Unit::Px));

    // selectorが複数のものに対応している
    let result3 = parse_block("a.sample_class { margin: 2px;}");
    let selector3 = result3.selectors.get(0).unwrap();
    assert_eq!(selector3.element, Some(ElementType::A));
    assert_eq!(selector3.class, Some(String::from("sample_class")));
    let dec3 = result3.declarations.get(0).unwrap();
    assert_eq!(dec3.property, Property::Margin);
    assert_eq!(dec3.value, Value::Length(2.0, Unit::Px));

    // declarationが複数のものに対応している
    let result4 = parse_block("#sample_id { display: block; width: 200px; }");
    let selector4 = result4.selectors.get(0).unwrap();
    assert_eq!(selector4.id, Some(String::from("sample_id")));
    let dec4_0 = result4.declarations.get(0).unwrap();
    assert_eq!(dec4_0.property, Property::Display);
    assert_eq!(dec4_0.value, Value::Keyword(String::from("block")));
    let dec4_1 = result4.declarations.get(1).unwrap();
    assert_eq!(dec4_1.property, Property::Width);
    assert_eq!(dec4_1.value, Value::Length(200.0, Unit::Px));
}

#[test]
fn test_pest_parser() {
    let result1 = CSSParser::parse(Rule::selector, "a").unwrap();
    for rule in result1 {
        assert_eq!(rule.as_str(), "a");
    }
    let result1_1 = CSSParser::parse(Rule::selector, "a ").unwrap();
    for rule in result1_1 {
        assert_eq!(rule.as_str(), "a");
    }

    let result1_2 = CSSParser::parse(Rule::selector, ".sample.sample").unwrap();
    for rule_inner in result1_2.into_iter() {
        for classes in rule_inner.into_inner().into_iter() {
            assert_eq!(classes.as_str(), ".sample");
        }
    }

    // ok
    let result1_4 = CSSParser::parse(Rule::id, "#sample").unwrap();
    for rule_inner in result1_4.into_iter() {
        assert_eq!(rule_inner.as_str(), "#sample");
    }

    let result1_3 = CSSParser::parse(Rule::selector, ".sample#sample").unwrap();
    for rule_inner in result1_3.into_iter() {
        let mut iter = rule_inner.into_inner().into_iter();
        let rule_1 = iter.next().unwrap();
        assert_eq!(rule_1.as_str(), ".sample");
        let rule_2 = iter.next().unwrap();
        assert_eq!(rule_2.as_str(), "#sample");
    }

    let result2 = CSSParser::parse(Rule::selector, "a.sample").unwrap();
    for rule_inner in result2.into_iter() {
        let mut iter = rule_inner.into_inner().into_iter();
        let rule_1 = iter.next().unwrap();
        assert_eq!(rule_1.as_str(), "a");
        let rule_2 = iter.next().unwrap();
        assert_eq!(rule_2.as_str(), ".sample");
    }

    let result2_1 = CSSParser::parse(Rule::block, "a.sample {}").unwrap();
    for rule_inner in result2_1.into_iter() {
        match rule_inner.as_rule() {
            Rule::selector => {
                let mut selector_iter = rule_inner.into_inner().into_iter();
                let rule_1 = selector_iter.next().unwrap();
                assert_eq!(rule_1.as_str(), "a");
                let rule_2 = selector_iter.next().unwrap();
                assert_eq!(rule_2.as_str(), ".sample");
            }
            _ => {
                println!("other rule {:?}", rule_inner);
            }
        }
    }
}

#[test]
fn test_pest_parser_declaration() {
    let result1 = CSSParser::parse(Rule::declaration, "padding: 2px;").unwrap();
    println!("{:?}", result1);
    for rule_inner in result1.into_iter() {
        let mut dec = rule_inner.into_inner().into_iter();
        let rule_1 = dec.next().unwrap();
        assert_eq!(rule_1.as_str(), "padding");
        let rule_2 = dec.next().unwrap();
        assert_eq!(rule_2.as_str(), "2px");
    }
}
