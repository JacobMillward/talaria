use dioxus::{
    desktop::{
        tao::event::{Event, WindowEvent}, window, Config
    }, prelude::*
};

use components::{Hero, NavBar, ThemeChanger};
use hooks::use_extracted_state;
use settings::{use_settings, SettingsProvider};

mod components;
mod hooks;
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
        .launch(|| {
            rsx! {
                SettingsProvider {
                    App {}
                }
            }
        });
}

#[component]
fn App() -> Element {
    let settings = use_settings();
    let theme = use_extracted_state(settings, |s| s.theme);

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
        
        ThemeChanger { theme }
        NavBar {}
        Hero {}
    }
}
