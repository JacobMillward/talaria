use std::rc::{Rc, Weak};
use dioxus::{desktop::{window, DesktopService}, prelude::*};
use crate::generate_config;

mod settings_view;
mod theme_picker;

pub mod global_shortcuts;
pub mod settings;

pub use settings::Theme;

// Type alias for our theme change callback
type ThemeCallback = Rc<dyn Fn(Option<Theme>)>;

static SETTINGS_WINDOW: GlobalSignal<Weak<DesktopService>> =
    GlobalSignal::new(|| Weak::<DesktopService>::new());

pub fn open_settings(theme_callback: ThemeCallback) {
    let state = SETTINGS_WINDOW().upgrade();

    match state {
        Some(service) => {
            service.set_focus();
        }
        None => {
            let config = generate_config().with_menu(None);
            let dom = VirtualDom::new_with_props(
                settings_view::settings_view,
                theme_callback
            );
            let service = window().new_window(dom, config);
            *SETTINGS_WINDOW.write() = service;
        }
    }
}
