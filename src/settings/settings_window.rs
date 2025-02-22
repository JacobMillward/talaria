use dioxus::prelude::*;

use crate::{FAVICON, TAILWIND_CSS};

#[component]
pub fn SettingsWindow() -> Element {
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
