#[macro_use]
extern crate lalrpop_util;
pub mod ast;

use crate::ast::ElementType;

lalrpop_mod!(pub html);

fn main() {}

#[test]
fn html() {
    // divの下にtextがある場合
    let test1 = html::HtmlParser::new().parse("<div>text</div>");
    assert!(test1.is_ok());
    assert_eq!(test1.unwrap().name, ElementType::Div);

    // div → span → text
    let test2 = html::HtmlParser::new().parse("<div><span>text</span></div>");
    assert!(test2.is_ok());
    let elem2 = test2.unwrap();
    assert_eq!(elem2.name, ElementType::Div);
    // 直下にspanがあるかどうか
    assert_eq!(elem2.children.len(), 1);
    assert_eq!(elem2.children[0].name, ElementType::Span);
    // spanのさらに下にtextがあるか
    assert_eq!(elem2.children[0].children.len(), 1);
    assert_eq!(elem2.children[0].children[0].name, ElementType::Text);

    // 改行を含む場合
    let test3 = html::HtmlParser::new().parse(
        "
<div>
  <span>text</span>
</div>
",
    );
    assert!(test3.is_ok());
    let elem3 = test3.unwrap();
    assert_eq!(elem3.name, ElementType::Div);
    // 直下にspanがあるかどうか
    assert_eq!(elem3.children.len(), 1);
    assert_eq!(elem3.children[0].name, ElementType::Span);
    // spanのさらに下にtextがあるか
    assert_eq!(elem3.children[0].children.len(), 1);
    assert_eq!(elem3.children[0].children[0].name, ElementType::Text);

    // 兄弟要素がある場合
    let test4 = html::HtmlParser::new().parse(
        "
    <html>
        <body>
            <h1>Title</h1>
            <div>
                <p>Hello <em>world</em>!</p>
            </div>
        </body>
    </html>
",
    );
    assert!(test4.is_ok());
    let elem4 = test4.unwrap();
    assert_eq!(elem4.name, ElementType::HTML);
    assert_eq!(elem4.children[0].name, ElementType::Body);
    assert_eq!(elem4.children[0].children[0].name, ElementType::H1);
    assert_eq!(elem4.children[0].children[1].name, ElementType::Div);
    assert_eq!(
        elem4.children[0].children[1].children[0].name,
        ElementType::P
    );
}
