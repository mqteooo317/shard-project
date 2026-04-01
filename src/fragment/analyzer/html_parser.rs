use lol_html::{html_content::ContentType, HtmlRewriter, Settings};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct ElementInfo {
    pub tag: String,
    pub id: Option<String>,
    pub class: Option<String>,
    pub inner_html: String,
    pub outer_html: String,
    pub depth: usize,
}

pub struct HtmlParser;

impl HtmlParser {
    pub fn parse(html: &str) -> Vec<ElementInfo> {
        let elements = Rc::new(RefCell::new(Vec::new()));
        let elements_clone = elements.clone();

        let mut rewriter = HtmlRewriter::new(
            Settings {
                element_content_handlers: vec![
                    lol_html::element!("*", |el| {
                        let tag = el.tag_name().to_string();
                        let id = el.get_attribute("id").map(|s| s.to_string());
                        let class = el.get_attribute("class").map(|s| s.to_string());
                        let depth = el.depth();

                        let inner_html = el.inner_html();
                        let outer_html = el.outer_html();

                        elements_clone.borrow_mut().push(ElementInfo {
                            tag,
                            id,
                            class,
                            inner_html,
                            outer_html,
                            depth,
                        });

                        Ok(())
                    }),
                ],
                ..Default::default()
            },
            |_: &[u8]| {},
        );

        rewriter
            .write(html.as_bytes())
            .expect("HTML parsing failed");
        rewriter.end().expect("HTML parsing failed");

        Rc::try_unwrap(elements).unwrap().into_inner()
    }
}