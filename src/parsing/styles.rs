use std::collections::HashMap;

use serde_json::Value;
use wgpu::Color;

use crate::{
    resources::Resource,
    ui::{style::Style, style_rule::StyleRule},
};

pub trait LoadStyles {
    fn load_styles(path: &str) -> Result<HashMap<String, Style>, anyhow::Error>;
}

impl LoadStyles for Resource {
    fn load_styles(file_name: &str) -> Result<HashMap<String, Style>, anyhow::Error> {
        let path = Self::build_path(Some("ui"), file_name);

        let source = std::fs::read_to_string(path)?;

        let v: Value = serde_json::from_str(&source)?;

        create_styles(v)
    }
}

fn create_styles(v: Value) -> Result<HashMap<String, Style>, anyhow::Error> {
    let mut styles = HashMap::new();

    for (key, value) in v.as_object().unwrap() {
        let style = match value {
            Value::Null => todo!(),
            Value::Bool(_) => todo!(),
            Value::Number(_) => todo!(),
            Value::String(_) => todo!(),
            Value::Array(_) => todo!(),
            Value::Object(obj) => create_style(obj.clone()),
        };

        styles.insert(key.to_owned(), style);
    }

    Ok(styles)
}

fn create_style(obj: serde_json::Map<String, Value>) -> Style {
    let mut rules = Vec::new();

    for (key, value) in obj {
        let rule = match value {
            Value::Null => todo!(),
            Value::Bool(_) => todo!(),
            Value::Number(_) => todo!(),
            Value::String(string) => create_rule(&key, string),
            Value::Array(_) => todo!(),
            Value::Object(_) => todo!(),
        };

        rules.push(rule);
    }

    Style { rules }
}

fn create_rule(key: &str, value: String) -> StyleRule {
    match key {
        "background_color" => StyleRule::BackgroundColor(parse_hex_color(&value)),
        "color" => StyleRule::TextColor(parse_hex_color(&value)),
        _ => todo!(),
    }
}

// TODO: Error checking.
fn parse_hex_color(color: &str) -> Color {
    let mut color = color.to_owned();

    if color.starts_with('#') {
        color.remove(0);
    }

    let mut chars = color.chars();

    let r = (hex(chars.next()) * 16.0 + hex(chars.next())) / 255.0;
    let g = (hex(chars.next()) * 16.0 + hex(chars.next())) / 255.0;
    let b = (hex(chars.next()) * 16.0 + hex(chars.next())) / 255.0;
    // let a = (hex(chars.next()) * 16.0 + hex(chars.next())) / 255.0;

    Color { r, g, b, a: 1.0 }
}

fn hex(digit: Option<char>) -> f64 {
    u32::from_str_radix(&digit.unwrap().to_string(), 16).unwrap() as f64
}
