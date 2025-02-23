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

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Settings {
    pub theme: Option<Theme>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: None,
        }
    }
}

impl Settings {
    pub fn update(&mut self, update: SettingsUpdate) {
        match update {
            SettingsUpdate::Theme(theme) => self.theme = theme,
        }
    }
}

pub enum SettingsUpdate {
    Theme(Option<Theme>),
}