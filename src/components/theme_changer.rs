use dioxus::prelude::*;

use crate::settings::models::Theme;


#[component]
pub fn ThemeChanger(theme: Signal<Option<Theme>>) -> Element {
    use_effect(move || {
        let current_theme = theme();
        
        if current_theme.is_none() {
            document::eval("document.documentElement.removeAttribute('data-theme')");
        } else {
            document::eval(&format!("document.documentElement.setAttribute('data-theme', '{}')", current_theme.unwrap().as_str()));
        }
    });

    rsx!()
}