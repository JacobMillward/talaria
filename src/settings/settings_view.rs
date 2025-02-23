use std::rc::Rc;

use dioxus::prelude::*;
use super::settings::Theme;
use crate::{components::ThemeChanger, FAVICON, TAILWIND_CSS};

use super::theme_picker::ThemePicker;


pub fn settings_view(theme_callback: Rc<dyn Fn(Option<Theme>)>) -> Element {
    let mut current_theme = use_signal(|| None::<Theme>);

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        ThemeChanger { theme: current_theme }
        
        main { class: "p-4",
            h1 { class: "text-2xl font-bold", "Settings" }

            div { class: "mt-2 p-2",
                ThemePicker { 
                    current_theme: current_theme.read().clone(),
                    change_theme: move |new_theme| {
                        current_theme.set(new_theme);
                        theme_callback(new_theme);
                    }
                }
            }
        }
    }
}