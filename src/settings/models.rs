#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn as_str(&self) -> &str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }
}

pub struct Settings {
    pub theme: Option<Theme>,
}
