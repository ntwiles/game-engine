use wgpu::Color;
use wgpu_glyph::{GlyphBrush, Section, Text};

use crate::graphics::Graphics;

use super::{element_kind::ElementKind, ui_vertex::UiRenderVertex};

const DEFAULT_PADDING: cgmath::Vector2<f32> = cgmath::Vector2 { x: 0.1, y: -0.1 };

#[derive(Debug)]
pub struct Element {
    pub render_id: usize,
    pub script_id: Option<String>,
    pub tag_name: String,
    pub body: Vec<ElementKind>,
}

impl Element {
    pub fn body(&self) -> &Vec<ElementKind> {
        &self.body
    }

    pub fn update(&self, graphics: &mut Graphics, starting_position: cgmath::Vector2<f32>) -> f32 {
        let body_height = self.body.iter().fold(0.0, |acc, child| {
            acc + match child {
                ElementKind::Element(element) => {
                    element.update(graphics, starting_position + DEFAULT_PADDING)
                }
                ElementKind::Content(_text) => 0.1,
            }
        });

        let height = body_height + (DEFAULT_PADDING.y.abs() * 2.0);
        // println!(
        //     "Render ID: {}\nTag: {}\nBody: {:?}\nBody Height: {}\nHeight: {}\nBody: {:?}\n",
        //     self.render_id, self.tag_name, self.body, body_height, height, self.body
        // );

        self.write_verts(graphics, starting_position, height);
        height
    }

    pub fn write_verts(
        &self,
        graphics: &mut Graphics,
        starting_position: cgmath::Vector2<f32>,
        height: f32,
    ) {
        let verts = [
            starting_position,
            cgmath::Vector2 {
                x: starting_position.x,
                y: starting_position.y - height,
            },
            cgmath::Vector2 {
                x: 1.0,
                y: starting_position.y - height,
            },
            cgmath::Vector2 {
                x: 1.0,
                y: starting_position.y,
            },
        ];

        let color = match self.render_id {
            0 => Color::BLACK,
            1 => Color::RED,
            2 => Color::GREEN,
            _ => Color::WHITE,
        };

        let render_verts = UiRenderVertex::new(&verts, color);
        graphics.write_ui_element(self.render_id, render_verts);
    }
}

pub trait DrawElement<'a> {
    fn draw_element(
        &mut self,
        element: &'a Element,
        text_brush: &mut GlyphBrush<()>,
        bounds: cgmath::Vector2<f32>,
        start_position: cgmath::Vector2<f32>,
    );
}

impl<'a, 'b> DrawElement<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_element(
        &mut self,
        element: &'b Element,
        text_brush: &mut GlyphBrush<()>,
        bounds: cgmath::Vector2<f32>,
        start_position: cgmath::Vector2<f32>,
    ) {
        let index_start = element.render_id as u32 * 6;
        let index_end = index_start + 6;

        self.draw_indexed(index_start..index_end, 0, 0..1);

        for child in element.body() {
            match child {
                ElementKind::Content(content) => draw_content(
                    content,
                    text_brush,
                    bounds,
                    start_position + DEFAULT_PADDING,
                ),
                ElementKind::Element(element) => self.draw_element(
                    &element,
                    text_brush,
                    bounds,
                    start_position + DEFAULT_PADDING,
                ),
            }
        }
    }
}

fn draw_content(
    content: &str,
    text_brush: &mut GlyphBrush<()>,
    bounds: cgmath::Vector2<f32>,
    position: cgmath::Vector2<f32>,
) {
    let draw_position = cgmath::Vector2::new(
        (position.x / 2.0) * bounds.x,
        (-position.y / 2.0) * bounds.y,
    );

    text_brush.queue(Section {
        screen_position: (draw_position.x, draw_position.y),
        bounds: (bounds.x, bounds.y),
        text: vec![Text::new(&content)
            .with_color([1.0, 1.0, 1.0, 1.0])
            .with_scale(20.0)],
        ..Section::default()
    })
}
