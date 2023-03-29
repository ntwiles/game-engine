use super::style_rule::StyleRule;

#[derive(Debug)]
pub struct Style {
    pub rules: Vec<StyleRule>,
}

impl Style {
    pub fn get_rules(&self) -> &Vec<StyleRule> {
        &self.rules
    }
}
