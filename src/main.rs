use dioxus::desktop::use_global_shortcut;
use dioxus::prelude::*;
use dioxus::{desktop::Config, desktop::WindowCloseBehaviour};

use components::Hero;
use settings::{open_settings, SettingsButton};

mod components;
mod settings;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

const INDEX_HTML: &str = include_str!("../index.html");

pub fn generate_config() -> Config {
    Config::new().with_close_behaviour(WindowCloseBehaviour::LastWindowHides)
        .with_custom_index(INDEX_HTML.into())
}
fn main() {
    dioxus::LaunchBuilder::desktop()
        .with_cfg(generate_config())
        .launch(App)
}

#[component]
fn App() -> Element {

    #[cfg(target_os = "macos")]
    let settings_shortcut = "super+,";
    
    #[cfg(not(target_os = "macos"))]
    let settings_shortcut = "ctrl+,";

    _ = use_global_shortcut(settings_shortcut, move || {
        open_settings();
    });

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        SettingsButton {}

        Hero {}

        
    }
}