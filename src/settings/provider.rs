use std::rc::Rc;
use dioxus::{desktop::use_global_shortcut, prelude::*};
use crate::settings::{
    Settings, SettingsUpdate,
    global_shortcuts::{compose_global_shortcut, SETTINGS_KEY},
};
use super::window::open_settings;

#[component]
pub fn SettingsProvider(children: Element) -> Element {
    let mut settings_state = use_context_provider(|| Signal::new(Settings::default()));

    // Create a coroutine to handle theme changes
    let handle = use_coroutine(move |mut rx: UnboundedReceiver<SettingsUpdate>| async move {
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
    
    rsx!{ {children} }
}

pub fn use_settings() -> Signal<Settings> {
    use_context::<Signal<Settings>>()
}