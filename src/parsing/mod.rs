use roxmltree::{Children, Document, Node, NodeType};

use crate::{
    resources::Resource,
    ui::{element::Element, element_kind::ElementKind},
};

pub trait LoadNml {
    fn load_nml(path: &str) -> Result<Element, ()>;
}

impl LoadNml for Resource {
    fn load_nml(file_name: &str) -> anyhow::Result<Element, ()> {
        let path = Self::build_path(Some("ui"), file_name);

        let source = std::fs::read_to_string(path).unwrap();

        let doc = roxmltree::Document::parse(&source).unwrap();

        create_model(doc)
    }
}

fn create_model(doc: Document) -> Result<Element, ()> {
    // TODO: Ensure that root node exists and that there is only one.
    let mut element_count = 1;

    let children = create_children(doc.root().children(), &mut element_count).unwrap();

    Ok(Element {
        render_id: 0,
        script_id: None,
        body: children,
        tag_name: "root".to_owned(),
    })
}

fn create_children(doc: Children, element_count: &mut usize) -> Result<Vec<ElementKind>, ()> {
    let children = doc.fold(Vec::new(), |mut acc, child| {
        match child.node_type() {
            NodeType::Element => {
                acc.push(ElementKind::Element(
                    create_element(child, element_count).unwrap(),
                ));
            }
            NodeType::Text => {
                let text = child.text().unwrap();

                if text.trim().is_empty() {
                    return acc;
                }

                acc.push(ElementKind::Content(text.to_owned()));
            }
            _ => todo!(),
        }

        acc
    });

    Ok(children)
}

fn create_element(node: Node, element_count: &mut usize) -> Result<Element, ()> {
    let children = create_children(node.children(), element_count).unwrap();
    let render_id = *element_count;
    *element_count += 1;

    Ok(Element {
        render_id,
        tag_name: node.tag_name().name().to_owned(),
        script_id: None,
        body: children,
    })
}
