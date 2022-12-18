use super::element::Element;

pub struct Canvas {
    document: Element,
    num_elements: usize,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            document: Element {
                id: 0,
                body: String::new(),
                children: Box::new(Vec::new()),
            },
            num_elements: 1,
        }
    }

    pub fn root(&mut self) -> &mut Element {
        &mut self.document
    }
}
