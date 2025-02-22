use crate::generate_config;

use std::rc::Weak;

use dioxus::{
    desktop::{window, DesktopService},
    prelude::*,
};

mod settings_window;
pub mod global_shortcuts;

pub static SETTINGS_WINDOW: GlobalSignal<Weak<DesktopService>> =
    GlobalSignal::new(|| Weak::<DesktopService>::new());

pub fn open_settings() {
    let state = SETTINGS_WINDOW.read().upgrade();

    match state {
        Some(service) => {
            service.set_focus();
        }
        None => {
            let config = generate_config().with_menu(None);
            let dom = VirtualDom::new(settings_window::SettingsWindow);
            let service = window().new_window(dom, config);

            *SETTINGS_WINDOW.write() = service;
        }
    }
}
