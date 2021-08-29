use std::result;

use pest::iterators::Pair;
use pest::Parser;

use crate::{
    css::ast::{Block, Declaration, Property, Selector, StyleSheet, Unit, Value},
    html::ast::{element_type, Element, ElementType},
};

#[derive(Parser)]
#[grammar = "css/css.pest"]
pub struct CSSParser;

pub fn parse_css(input: &str) -> StyleSheet {
    let parser = CSSParser::parse(Rule::css, input).unwrap();
    let mut css = StyleSheet::new();
    for line in parser.into_iter() {
        match line.as_rule() {
            Rule::block => {
                let block = parse_block(line.as_str());
                css.append_block(block);
            }
            _ => {}
        }
    }
    css
}

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
        *result1_1.selectors.get(0).unwrap().class.get(0).unwrap(),
        String::from("sample")
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
    assert_eq!(
        *selector3.class.get(0).unwrap(),
        String::from("sample_class")
    );
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
fn test_css_parse_2() {
    // 改行がある
    let result1 = parse_css(
        "
h1 {
    font-size: 50px;
}
",
    );
    let block1 = result1.blocks.get(0).unwrap();
    let selector1 = block1.selectors.get(0).unwrap();
    assert_eq!(selector1.element, Some(ElementType::H1));
    let dec1 = block1.declarations.get(0).unwrap();
    assert_eq!(dec1.property, Property::FontSize);
    assert_eq!(dec1.value, Value::Length(50.0, Unit::Px));

    // 複数行の宣言がある
    let result2 = parse_css(
        "
span {
    display: inline-block;
    background-color: black;
}
",
    );
    let block2 = result2.blocks.get(0).unwrap();
    let selector2 = block2.selectors.get(0).unwrap();
    assert_eq!(selector2.element, Some(ElementType::Span));
    let dec2_1 = block2.declarations.get(0).unwrap();
    assert_eq!(dec2_1.property, Property::Display);
    assert_eq!(dec2_1.value, Value::Keyword("inline-block".to_string()));
    let dec2_2 = block2.declarations.get(1).unwrap();
    assert_eq!(dec2_2.property, Property::BackgroundColor);
    assert_eq!(dec2_2.value, Value::Keyword("black".to_string()));
}

#[test]
fn test_css_parse_3() {
    // https://limpet.net/mbrubeck/2014/08/13/toy-layout-engine-3-css.html
    // のパターンに対応する
    let result1 = parse_css(
        "
h1, h2, h3 { margin: auto; color: #cc0000; }
div.note { margin-bottom: 20px; padding: 10px; }
#answer { display: none; }
",
    );
    let block1 = result1.blocks.get(0).unwrap();
    let sel1_1 = block1.selectors.get(0).unwrap();
    assert_eq!(sel1_1.element, Some(ElementType::H1));
    let sel1_2 = block1.selectors.get(1).unwrap();
    assert_eq!(sel1_2.element, Some(ElementType::H2));
    let sel1_3 = block1.selectors.get(2).unwrap();
    assert_eq!(sel1_3.element, Some(ElementType::H3));
    let dec1_1 = block1.declarations.get(0).unwrap();
    assert_eq!(dec1_1.property, Property::Margin);
    assert_eq!(dec1_1.value, Value::Keyword("auto".to_string()));
    let dec1_2 = block1.declarations.get(1).unwrap();
    assert_eq!(dec1_2.property, Property::Color);
    assert_eq!(dec1_2.value, Value::Keyword("#cc0000".to_string()));

    let block2 = result1.blocks.get(1).unwrap();
    let sel2 = block2.selectors.get(0).unwrap();
    assert_eq!(sel2.element, Some(ElementType::Div));
    assert_eq!(*sel2.class.get(0).unwrap(), "note".to_string());
    let dec2_1 = block2.declarations.get(0).unwrap();
    assert_eq!(dec2_1.property, Property::MarginBottom);
    assert_eq!(dec2_1.value, Value::Length(20.0, Unit::Px));
    let dec2_2 = block2.declarations.get(1).unwrap();
    assert_eq!(dec2_2.property, Property::Padding);
    assert_eq!(dec2_2.value, Value::Length(10.0, Unit::Px));

    let block3 = result1.blocks.get(2).unwrap();
    let sel3 = block3.selectors.get(0).unwrap();
    assert_eq!(sel3.id, Some("answer".to_string()));
    let dec3 = block3.declarations.get(0).unwrap();
    assert_eq!(dec3.property, Property::Display);
    assert_eq!(dec3.value, Value::Keyword("none".to_string()));
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
    for rule_inner in result1.into_iter() {
        let mut dec = rule_inner.into_inner().into_iter();
        let rule_1 = dec.next().unwrap();
        assert_eq!(rule_1.as_str(), "padding");
        let rule_2 = dec.next().unwrap();
        assert_eq!(rule_2.as_str(), "2px");
    }
}

#[test]
fn test_pest_parser_selector() {
    let result1 = CSSParser::parse(Rule::block, "a, div {}").unwrap();
    for line_inner in result1.into_iter() {
        let mut inner = line_inner.into_inner().into_iter();
        let elem1 = inner.next().unwrap();
        assert_eq!(elem1.as_str(), "a");
        let elem2 = inner.next().unwrap();
        assert_eq!(elem2.as_str(), "div");
    }
}
