use super::element::Element;

pub struct Canvas {
    root: Element,
}

impl Canvas {
    pub fn new(root: Element) -> Self {
        Self { root }
    }

    pub fn root(&mut self) -> &mut Element {
        &mut self.root
    }
}
