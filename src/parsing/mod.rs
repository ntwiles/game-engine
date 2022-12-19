use std::{iter::Peekable, str::Chars};

use crate::{
    resources::Resource,
    ui::element::{Element, ElementBody},
};

pub trait LoadNml {
    fn load_nml(path: &str) -> Result<Element, ()>;
}

impl LoadNml for Resource {
    fn load_nml(file_name: &str) -> anyhow::Result<Element, ()> {
        let path = Self::build_path(Some("ui"), file_name);

        // TODO: Can/should this be done in an io stream instead of reading to a string?
        let source = std::fs::read_to_string(path).unwrap();

        parse_nml(source)
    }
}

fn parse_nml(source: String) -> Result<Element, ()> {
    println!("{source}");

    let mut stream = source.chars().peekable();

    if let Some(_) = stream.find(|char| *char == '<') {
        let _ = capture_tag(&mut stream);
        let body = Box::new(capture_body(&mut stream));

        Ok(Element {
            render_id: 0,
            script_id: None,
            body,
        })
    } else {
        Err(())
    }
}

fn capture_tag(stream: &mut Peekable<Chars>) -> String {
    let mut tag = String::new();

    while let Some(c) = stream.next() {
        if c == '>' {
            break;
        }

        if !c.is_whitespace() {
            tag += &c.to_string();
        }
    }

    tag
}

fn capture_body(stream: &mut Peekable<Chars>) -> ElementBody {
    match stream.next() {
        Some('<') => ElementBody::Child(capture_child(stream)),
        Some(first_char) => ElementBody::Content(capture_content(stream, first_char)),
        None => todo!(),
    }
}

fn capture_content(stream: &mut Peekable<Chars>, first_char: char) -> String {
    let mut content = first_char.to_string();

    while let Some(c) = stream.next() {
        if c == '<' {
            break;
        }

        content += &c.to_string();
    }

    content.trim().to_owned()
}

fn capture_child(stream: &mut Peekable<Chars>) -> Element {
    todo!()
}
