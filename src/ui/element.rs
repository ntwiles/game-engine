use wgpu_glyph::{GlyphBrush, Section, Text};

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

pub const DEFAULT_LINE_HEIGHT: f32 = 0.07;
pub const DEFAULT_PADDING: cgmath::Vector2<f32> = cgmath::Vector2 { x: 0.1, y: 0.1 };

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
        let index_start = element.render_id as u32;
        let index_end = element.render_id as u32 + 6;

        self.draw_indexed(index_start..index_end, 0, 0..1);

        let body = element.body();

        match body {
            ElementBody::Content(content) => draw_content(
                content,
                text_brush,
                bounds,
                start_position + DEFAULT_PADDING,
            ),
            ElementBody::Child(child) => {
                self.draw_element(child, text_brush, bounds, start_position + DEFAULT_PADDING)
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
    text_brush.queue(Section {
        screen_position: (position.x, position.y),
        bounds: (bounds.x, bounds.y),
        text: vec![Text::new(&content)
            .with_color([1.0, 1.0, 1.0, 1.0])
            .with_scale(20.0)],
        ..Section::default()
    })
}
