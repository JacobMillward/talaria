use std::rc::Weak;
use dioxus::{desktop::DesktopService, prelude::*};
use dioxus::desktop::window;
use crate::generate_config;
use super::{Settings, settings_view::{settings_view, SettingsUpdateCallback, SettingsViewProps}};

static SETTINGS_WINDOW: GlobalSignal<Weak<DesktopService>> =
    GlobalSignal::new(|| Weak::<DesktopService>::new());

pub fn open_settings(initial_settings: Settings, settings_callback: SettingsUpdateCallback) {
    let state = SETTINGS_WINDOW().upgrade();

    match state {
        Some(service) => {
            service.set_focus();
        }
        None => {
            let config = generate_config().with_menu(None);
            let dom = VirtualDom::new_with_props(
                settings_view,
                SettingsViewProps {
                    initial_settings,
                    settings_callback,
                },
            );
            let service = window().new_window(dom, config);
            *SETTINGS_WINDOW.write() = service;
        }
    }
}