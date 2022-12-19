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

pub trait DrawElement<'a> {
    fn draw_element(
        &mut self,
        element: &'a Element,
        text_brush: &mut GlyphBrush<()>,
        bounds: (f32, f32),
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
        bounds: (f32, f32),
    ) {
        let index_start = element.render_id as u32;
        let index_end = element.render_id as u32 + 6;

        self.draw_indexed(index_start..index_end, 0, 0..1);

        let body = element.body();

        if let ElementBody::Content(content) = body {
            text_brush.queue(Section {
                screen_position: (0.0, 0.0),
                bounds,
                text: vec![Text::new(&content)
                    .with_color([1.0, 1.0, 1.0, 1.0])
                    .with_scale(20.0)],
                ..Section::default()
            });
        }
    }
}
