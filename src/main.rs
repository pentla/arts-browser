#[macro_use]
extern crate lalrpop_util;
pub mod ast;

lalrpop_mod!(pub html);

fn main() {}

#[test]
fn html() {
    let div_in_text = html::HtmlParser::new().parse("<div>text</div>");
    // let div = html::HtmlParser::new().parse("<div></div>").unwrap();
    assert!(div_in_text.is_ok());
    assert_eq!(div_in_text.unwrap(), "div");
}
