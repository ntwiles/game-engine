pub struct Element {
    pub id: usize,
    pub body: String,
    pub children: Box<Vec<Element>>,
}

impl Element {
    pub fn body(&self) -> String {
        self.body.clone()
    }

    pub fn set_body(&mut self, body: &str) {
        self.body = body.to_owned();
    }
}
