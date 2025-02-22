use std::rc::Weak;

use dioxus::{
    desktop::{
        window, DesktopService,
    },
    prelude::*
};

use crate::{generate_config, FAVICON, TAILWIND_CSS};

pub static SETTINGS_WINDOW: GlobalSignal<Weak<DesktopService>> = GlobalSignal::new(|| Weak::<DesktopService>::new());

#[component]
fn Settings_Window() -> Element {
    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        main { class: "bg-base-100 p-4",
            h1 { class: "text-2xl font-bold", "Settings" }
            p { "This is a new window" }
        }
    }
}

#[component]
pub fn SettingsButton() -> Element {
    let onclick = move |_| {
        open_settings();
    };

    rsx! {
        button {
            onclick,
            class: "btn btn-neutral",
            "Settings"
        }
    }
}

pub fn open_settings() {
    let state = SETTINGS_WINDOW.read().upgrade();

    match state {
        Some(service) => {
            service.set_focus();
        },
        None => {
            let config = generate_config()
                .with_menu(None);

            let dom = VirtualDom::new(Settings_Window);
            let service = window().new_window(dom, config);
            
            *SETTINGS_WINDOW.write() = service;
        }
    }
}