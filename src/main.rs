use dioxus::{
    desktop::{
        tao::event::{Event, WindowEvent}, use_global_shortcut, window, Config
    },
    prelude::*,
};

use components::{Hero, NavBar};
use settings::global_shortcuts::compose_global_shortcut;
use settings::{
    global_shortcuts::SETTINGS_KEY,
    open_settings,
};

mod components;
mod settings;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

const INDEX_HTML: &str = include_str!("../index.html");

pub fn generate_config() -> Config {
    Config::new().with_custom_index(INDEX_HTML.into())
}
fn main() {
    dioxus::LaunchBuilder::desktop()
        .with_cfg(generate_config())
        .launch(App)
}

#[component]
fn App() -> Element {
    // Setup global shortcut for opening settings
    _ = use_global_shortcut(compose_global_shortcut(SETTINGS_KEY).as_str(), open_settings);

    // On main window close, exit the app
    window().create_wry_event_handler(|event, 
                                     _target| {
        if let Event::WindowEvent { 
            event: WindowEvent::CloseRequested,
            ..
        } = event {
            std::process::exit(0);
        }
    });

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        NavBar {}

        Hero {}


    }
}
