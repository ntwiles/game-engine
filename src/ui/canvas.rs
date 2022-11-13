use super::element::Element;

pub struct Canvas {
    document: Element,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            document: Element {
                id: String::new(),
                body: String::new(),
                children: Box::new(Vec::new()),
            },
        }
    }

    pub fn root(&mut self) -> &mut Element {
        &mut self.document
    }
}
