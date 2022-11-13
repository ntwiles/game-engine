pub struct Element {
    pub id: String, // TODO: This should be hashed an uniqueness enforced.
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
