use std::rc::Rc;

use dioxus::prelude::*;
use crate::{components::ThemeChanger, hooks::use_extracted_state, settings::theme_picker::ThemePicker, FAVICON, TAILWIND_CSS};

use super::models::{Settings, SettingsUpdate};

pub type SettingsUpdateCallback = Rc<dyn Fn(SettingsUpdate)>;

#[derive(Clone)]
pub struct SettingsViewProps {
    pub initial_settings: Settings,
    pub settings_callback: SettingsUpdateCallback,
}

pub fn settings_view(props: SettingsViewProps) -> Element {
    let mut settings = use_signal(|| props.initial_settings.clone());
    let current_theme = use_extracted_state(settings, |s| s.theme);

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        
        ThemeChanger { theme: current_theme }
        
        main { class: "p-4",
            h1 { class: "text-2xl font-bold", "Settings" }

            div { class: "mt-2 p-2",
                ThemePicker { 
                    current_theme: current_theme(),
                    change_theme: move |new_theme| {
                        settings.write().theme = new_theme;
                        (props.settings_callback)(SettingsUpdate::Theme(new_theme));
                    }
                }
            }
        }
    }
}