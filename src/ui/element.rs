pub enum ElementBody {
    Child(Element),
    Content(String),
}

pub struct Element {
    pub render_id: usize,
    pub script_id: Option<String>,
    pub body: Box<ElementBody>,
}

impl Element {
    pub fn body(&self) -> &ElementBody {
        &*self.body
    }

    pub fn set_body_text(&mut self, content: &str) {
        self.body = Box::new(ElementBody::Content(content.to_owned()));
    }
}
