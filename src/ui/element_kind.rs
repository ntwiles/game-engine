use cgmath::Vector2;

use crate::graphics::Graphics;

use super::element::Element;

#[derive(Debug)]
pub enum ElementKind {
    Element(Element),
    Content(String),
}

impl ElementKind {
    pub fn update(&mut self, graphics: &mut Graphics, starting_position: Vector2<f32>) -> f32 {
        match self {
            ElementKind::Element(element) => element.update(graphics, starting_position),
            ElementKind::Content(_text) => 0.1,
        }
    }

    pub fn get_height(&self) -> f32 {
        match self {
            ElementKind::Element(element) => element.height,
            ElementKind::Content(_text) => 0.1,
        }
    }
}
