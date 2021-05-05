#[macro_use]
extern crate lalrpop_util;
pub mod ast;

lalrpop_mod!(pub html);

fn main() {}

#[test]
fn html() {
    let expr = html::HtmlParser::new().parse("<div></div>").unwrap();
    assert_eq!(&format!("{}", expr), "div");
}
