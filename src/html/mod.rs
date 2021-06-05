pub mod ast;

use crate::html::ast::Element;

lalrpop_mod!(pub html, "/html/grammer.rs");

pub fn parse(text: &str) -> Box<Element> {
    html::HtmlParser::new().parse(text).unwrap()
}

#[test]
fn html() {
    use crate::html::ast::ElementType;

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
  <h1>Text</h1>
</div>
",
    );
    assert!(test3.is_ok());
    let elem3 = test3.unwrap();
    assert_eq!(elem3.name, ElementType::Div);
    // 直下にspanがあるかどうか
    assert_eq!(elem3.children.len(), 1);
    assert_eq!(elem3.children[0].name, ElementType::H1);
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
            <p>hello</p>
        </div>
    </body>
</html>
",
    );
    assert!(test4.is_ok());
    let elem4 = test4.unwrap();
    assert_eq!(elem4.name, ElementType::Html);
    assert_eq!(elem4.children[0].name, ElementType::Body);
    assert_eq!(elem4.children[0].children[0].name, ElementType::H1);
    assert_eq!(elem4.children[0].children[1].name, ElementType::Div);
    assert_eq!(
        elem4.children[0].children[1].children[0].name,
        ElementType::P
    );

    // テキストの後に要素が続く場合
    let test5 = html::HtmlParser::new().parse("<div>Hello<em>world</em>!</div>");
    assert!(test5.is_ok());
    let elem5 = test5.unwrap();
    assert_eq!(elem5.name, ElementType::Div);
    assert_eq!(elem5.children[0].name, ElementType::Text);
    assert_eq!(elem5.children[0].text, "Hello");
    assert_eq!(elem5.children[1].name, ElementType::Em);
    assert_eq!(elem5.children[2].name, ElementType::Text);
    assert_eq!(elem5.children[2].text, "!");

    // id, classのパース
    let text6 = html::HtmlParser::new().parse("<div id=\"text\" class=\"hi\">text</div>");
    assert!(text6.is_ok());
    let elem6 = text6.unwrap();
    assert_eq!(elem6.name, ElementType::Div);
    assert_eq!(elem6.id, "text");
    assert_eq!(elem6.class, "hi");
    assert_eq!(elem6.children[0].name, ElementType::Text);
    assert_eq!(elem6.children[0].text, "text");
}
