use wgpu::Color;
use wgpu_glyph::{GlyphBrush, Section, Text};

use crate::{graphics::Graphics, state::State};

use super::ui_vertex::UiRenderVertex;

const DEFAULT_LINE_HEIGHT: f32 = 0.07;
const DEFAULT_PADDING: cgmath::Vector2<f32> = cgmath::Vector2 { x: 0.1, y: -0.1 };

pub enum ElementBody {
    Child(Box<Element>),
    Content(String),
}

pub struct Element {
    pub render_id: usize,
    pub script_id: Option<String>,
    pub body: Option<ElementBody>,
}

impl Element {
    pub fn body(&self) -> &Option<ElementBody> {
        &self.body
    }

    pub fn set_body_text(&mut self, content: &str) {
        self.body = Some(ElementBody::Content(content.to_owned()));
    }

    pub fn update(&self, graphics: &mut Graphics, starting_position: cgmath::Vector2<f32>) -> f32 {
        let body_height = match &self.body {
            None => 0.0,
            Some(ElementBody::Content(_)) => DEFAULT_LINE_HEIGHT,
            Some(ElementBody::Child(child)) => {
                child.update(graphics, starting_position + DEFAULT_PADDING)
            }
        };

        let height = body_height + (DEFAULT_PADDING.y.abs() * 2.0);
        self.write_verts(graphics, starting_position, height);
        height
    }

    pub fn write_verts(
        &self,
        graphics: &mut Graphics,
        starting_position: cgmath::Vector2<f32>,
        height: f32,
    ) {
        println!("Staring position: {starting_position:?}, height: {height}");

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

        let color = if self.render_id == 0 {
            Color::BLACK
        } else {
            Color::RED
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

        let body = element.body();

        match body {
            Some(ElementBody::Content(content)) => draw_content(
                content,
                text_brush,
                bounds,
                start_position + DEFAULT_PADDING,
            ),
            Some(ElementBody::Child(child)) => {
                self.draw_element(child, text_brush, bounds, start_position + DEFAULT_PADDING)
            }
            None => todo!(),
        }
    }
}

fn draw_content(
    content: &str,
    text_brush: &mut GlyphBrush<()>,
    bounds: cgmath::Vector2<f32>,
    position: cgmath::Vector2<f32>,
) {
    text_brush.queue(Section {
        screen_position: (position.x, position.y),
        bounds: (bounds.x, bounds.y),
        text: vec![Text::new(&content)
            .with_color([1.0, 1.0, 1.0, 1.0])
            .with_scale(20.0)],
        ..Section::default()
    })
}
