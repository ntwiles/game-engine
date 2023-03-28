use super::element::Element;

#[derive(Debug)]
pub enum ElementKind {
    Element(Element),
    Content(String),
}
