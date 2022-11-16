pub struct Config {
    developer_mode: bool,
}

impl Config {
    pub fn new() -> Self {
        Self {
            developer_mode: get_bool("DEV_MODE", Some(false)),
        }
    }

    pub fn developer_mode(&self) -> bool {
        self.developer_mode
    }
}

fn get_bool(name: &str, default: Option<bool>) -> bool {
    match std::env::var(name) {
        Ok(raw) => parse_bool(&raw),
        Err(_) => default.expect(&format!(
            "Environment value `{name}` is not present and no default was supplied."
        )),
    }
}

fn parse_bool(raw: &str) -> bool {
    let raw = raw.to_lowercase();

    if raw == "true" || raw == "1" {
        return true;
    } else if raw == "false" || raw == "0" {
        return false;
    }

    panic!("Invalid boolean value {raw} for environment variable.");
}
