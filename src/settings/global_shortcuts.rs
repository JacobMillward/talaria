#[cfg(target_os = "macos")]
pub const GLOBAL_MODIFIER: &str = "super";
#[cfg(not(target_os = "macos"))]
pub const GLOBAL_MODIFIER: &str = "ctrl";

pub const SETTINGS_KEY: &str = ",";

pub fn compose_shortcut(key: &str, modifier: &str) -> String {
    format!("{}+{}", modifier, key)
}

pub fn compose_global_shortcut(key: &str) -> String {
    compose_shortcut(key, GLOBAL_MODIFIER)
}