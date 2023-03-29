use std::collections::HashMap;

use cgmath::Vector2;

use crate::graphics::Graphics;

use super::{element::Element, style::Style};

#[derive(Debug)]
pub enum ElementKind {
    Element(Element),
    Content(String),
}

impl ElementKind {
    pub fn update(
        &mut self,
        graphics: &mut Graphics,
        starting_position: Vector2<f32>,
        right_bound: f32,
        styles: &HashMap<String, Style>,
    ) -> () {
        if let ElementKind::Element(element) = self {
            element.update(graphics, starting_position, right_bound, styles);
        }
    }

    pub fn get_height(&self) -> f32 {
        match self {
            ElementKind::Element(element) => *element.get_height(),
            ElementKind::Content(_text) => 0.1,
        }
    }
}
