use std::collections::HashMap;

use wgpu::Color;
use wgpu_glyph::{GlyphBrush, Section, Text};

use crate::graphics::Graphics;

use super::{
    element_kind::ElementKind, style::Style, style_rule::StyleRule, ui_vertex::UiRenderVertex,
};

const DEFAULT_PADDING: cgmath::Vector2<f32> = cgmath::Vector2 { x: 0.1, y: -0.1 };

#[derive(Debug)]
pub struct Element {
    pub render_id: usize,
    pub script_id: Option<String>,
    pub tag_name: String,
    pub body: Vec<ElementKind>,
    height: f32,
}

impl Element {
    pub fn new(
        render_id: usize,
        script_id: Option<String>,
        tag_name: String,
        body: Vec<ElementKind>,
    ) -> Self {
        Self {
            render_id,
            script_id,
            tag_name,
            body,
            height: 0.0,
        }
    }
    pub fn body(&self) -> &Vec<ElementKind> {
        &self.body
    }

    pub fn update(
        &mut self,
        graphics: &mut Graphics,
        starting_position: cgmath::Vector2<f32>,
        right_bound: f32,
        styles: &HashMap<String, Style>,
    ) -> () {
        let mut child_position = starting_position;

        let body_height = self.body.iter_mut().fold(0.0, |acc, child| {
            child.update(
                graphics,
                child_position + DEFAULT_PADDING,
                right_bound - DEFAULT_PADDING.x,
                styles,
            );

            let child_height = child.get_height();
            child_position.y -= child_height;
            acc + child_height
        });

        let height = body_height + (DEFAULT_PADDING.y.abs() * 2.0);

        self.write_verts(
            graphics,
            starting_position,
            height,
            right_bound,
            styles.get(&self.tag_name),
        );

        self.height = height;
    }

    pub fn write_verts(
        &self,
        graphics: &mut Graphics,
        starting_position: cgmath::Vector2<f32>,
        height: f32,
        right_bound: f32,
        style: Option<&Style>,
    ) {
        let verts = [
            starting_position,
            cgmath::Vector2 {
                x: starting_position.x,
                y: starting_position.y - height,
            },
            cgmath::Vector2 {
                x: right_bound,
                y: starting_position.y - height,
            },
            cgmath::Vector2 {
                x: right_bound,
                y: starting_position.y,
            },
        ];

        let mut background_color = Color::WHITE;

        if let Some(style) = style {
            for rule in style.get_rules() {
                match rule {
                    StyleRule::BackgroundColor(color) => background_color = *color,
                    _ => (),
                }
            }
        }

        let render_verts = UiRenderVertex::new(&verts, background_color);
        graphics.write_ui_element(self.render_id, render_verts);
    }

    pub fn get_height(&self) -> &f32 {
        &self.height
    }
}

pub trait DrawElement<'a> {
    fn draw_element(
        &mut self,
        element: &'a Element,
        text_brush: &mut GlyphBrush<()>,
        bounds: cgmath::Vector2<f32>,
        start_position: cgmath::Vector2<f32>,
        style: &HashMap<String, Style>,
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
        styles: &HashMap<String, Style>,
    ) {
        let index_start = element.render_id as u32 * 6;
        let index_end = index_start + 6;

        self.draw_indexed(index_start..index_end, 0, 0..1);

        let mut draw_position = start_position;

        let style = styles.get(&element.tag_name);

        for child in element.body() {
            match child {
                ElementKind::Content(content) => draw_content(
                    content,
                    text_brush,
                    bounds,
                    draw_position + DEFAULT_PADDING,
                    style,
                ),
                ElementKind::Element(element) => self.draw_element(
                    &element,
                    text_brush,
                    bounds,
                    draw_position + DEFAULT_PADDING,
                    styles,
                ),
            }

            draw_position.y -= child.get_height();
        }
    }
}

fn draw_content(
    content: &str,
    text_brush: &mut GlyphBrush<()>,
    bounds: cgmath::Vector2<f32>,
    position: cgmath::Vector2<f32>,
    style: Option<&Style>,
) {
    let draw_position = cgmath::Vector2::new(
        (position.x / 2.0) * bounds.x,
        (-position.y / 2.0) * bounds.y,
    );

    let mut text_color = [0.0, 0.0, 0.0, 1.0];

    if let Some(style) = style {
        for rule in style.get_rules() {
            match rule {
                StyleRule::TextColor(c) => {
                    text_color = [c.r as f32, c.g as f32, c.b as f32, c.a as f32]
                }
                _ => (),
            }
        }
    }

    text_brush.queue(Section {
        screen_position: (draw_position.x, draw_position.y),
        bounds: (bounds.x, bounds.y),
        text: vec![Text::new(&content).with_color(text_color).with_scale(20.0)],
        ..Section::default()
    })
}
