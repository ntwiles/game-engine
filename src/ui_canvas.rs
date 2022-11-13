pub struct UiCanvas {
    pub text: String,
}

impl UiCanvas {
    pub fn set_text(&mut self, content: &str) {
        self.text = content.to_owned();
    }
}
