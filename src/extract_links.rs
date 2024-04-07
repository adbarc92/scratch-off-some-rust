use html5ever::{parse_document, tendril::TendrilSink, QualName};
use url::Url;

pub fn extract_links(html: &str, base_url: &Url) -> Vec<String> {
    let mut links = Vec::new();
    let parser = parse_document(Sink, Default::default())
        .from_utf8()
        .read_from(&mut html.as_bytes());

    if let Ok(parser) = parser {
        for node in parser.nodes.collect::<Vec<_>>() {
            match node.data {
                html5ever::NodeData::Element { ref name, .. } => {
                    if let Some(local_name) = name.local {
                        if local_name == local_name!("a") {
                            if let Some(attr) = node.get_attribute(&ns!(), &local_name!("href")) {
                                let link = attr.value.to_string();
                                let full_link = base_url.join(&link).unwrap().to_string();
                                links.push(full_link);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    links
}

struct Sink;

impl html5ever::tendril::TendrilSink for Sink {
    type Handle = std::rc::Rc<std::cell::RefCell<Vec<html5ever::NodeData>>>;

    fn process_token(&mut self, token: html5ever::Token, handle: &Self::Handle) {
        match token {
            html5ever::Token::NullCharacter => {}
            html5ever::Token::Tag(tag) => {
                handle.borrow_mut().push(html5ever::NodeData::Element {
                    name: QualName {
                        prefix: tag.name.prefix.map(|foo| foo.to_string()),
                        namespace_id: tag.name.namespace.unwrap_or(ns!(html)),
                        local: tag.name.local.to_string(),
                    },
                    attrs: tag
                        .attrs
                        .iter()
                        .map(|attr| (attr.name.clone(), attr.value.clone()))
                        .collect(),
                    template_contents: None,
                    mathml_annotation_xml_integration_vector: None,
                });
            }
            html5ever::Token::Text(text) => {
                handle.borrow_mut().push(html5ever::NodeData::Text {
                    contents: text.into(),
                });
            }
            _ => {}
        }
    }

    fn get_handle(&mut self) -> std::rc::Rc<std::cell::RefCell<Vec<html5ever::NodeData>>> {
        std::rc::Rc::new(std::cell::RefCell::new(vec![]))
    }
}
