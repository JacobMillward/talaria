use std::rc::{Rc, Weak};
use dioxus::{desktop::{use_global_shortcut, window, DesktopService}, prelude::*};
use crate::generate_config;

// Private modules
mod settings_view;
mod theme_picker;

// Public modules
pub mod global_shortcuts;
pub mod models;

// Re-exports
pub use models::{Settings, SettingsUpdate, Theme};
pub use settings_view::SettingsUpdateCallback;

use global_shortcuts::{compose_global_shortcut, SETTINGS_KEY};


static SETTINGS_WINDOW: GlobalSignal<Weak<DesktopService>> =
    GlobalSignal::new(|| Weak::<DesktopService>::new());

pub fn use_settings() -> Signal<Settings> {
    use_context::<Signal<Settings>>()
}

#[component]
pub fn SettingsProvider(children: Element) -> Element {
    let mut settings_state =  use_context_provider(|| Signal::new(Settings::default()));

     // Create a coroutine to handle theme changes
     let handle = use_coroutine(move |mut rx: UnboundedReceiver<SettingsUpdate>|
        async move {
            use futures_util::StreamExt;
            while let Some(update) = rx.next().await {
                settings_state.write().update(update);
            }
    });

    let open_settings_handler = move || {
        let tx = handle.tx();
        open_settings(settings_state.read().clone(), Rc::new(move |update| {
            tx.unbounded_send(update).unwrap();
        }));
    };

    _ = use_global_shortcut(compose_global_shortcut(SETTINGS_KEY).as_str(), move || open_settings_handler());
    
    rsx!{
        {children}
    }
}

fn open_settings(initial_settings: Settings, settings_callback: SettingsUpdateCallback) {
    let state = SETTINGS_WINDOW().upgrade();

    match state {
        Some(service) => {
            service.set_focus();
        }
        None => {
            let config = generate_config().with_menu(None);
            let dom = VirtualDom::new_with_props(
                settings_view::settings_view,
                settings_view::SettingsViewProps {
                    initial_settings,
                    settings_callback,
                },
            );
            let service = window().new_window(dom, config);
            *SETTINGS_WINDOW.write() = service;
        }
    }
}