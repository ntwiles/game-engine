use wgpu::Color;

#[derive(Debug)]
pub enum StyleRule {
    BackgroundColor(Color),
    TextColor(Color),
}
