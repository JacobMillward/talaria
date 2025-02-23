use std::rc::Rc;

use dioxus::{
    desktop::{
        tao::event::{Event, WindowEvent}, use_global_shortcut, window, Config
    },
    prelude::*,
};

use components::{Hero, NavBar, ThemeChanger};
use settings::{global_shortcuts::compose_global_shortcut, Theme};
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
    let mut theme = use_signal(|| None::<Theme>);

    // Create a coroutine to handle theme changes
    let handle = use_coroutine(move |mut rx: UnboundedReceiver<Option<Theme>>|
        async move {
            use futures_util::StreamExt;
            while let Some(new_theme) = rx.next().await {
                theme.set(new_theme);
            }
    });

    // Setup global shortcut for opening settings
    let open_settings_handler = move || {
        let tx = handle.tx();
        open_settings(Rc::new(move |new_theme| {
            let _ = tx.unbounded_send(new_theme);
        }));
    };

    _ = use_global_shortcut(compose_global_shortcut(SETTINGS_KEY).as_str(), open_settings_handler);

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
