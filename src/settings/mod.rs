mod settings_view;
mod theme_picker;
mod provider;
mod window;

pub mod global_shortcuts;
pub mod models;

pub use models::{Settings, SettingsUpdate, Theme};
pub use provider::SettingsProvider;
pub use provider::use_settings;