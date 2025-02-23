use dioxus::prelude::*;

use super::super::settings::Theme;

#[derive(Props, PartialEq, Clone)]
pub struct ThemePickerProps {
    current_theme: Option<Theme>,
    change_theme: Callback<Option<Theme>>,
}

#[component]
pub fn ThemePicker(props: ThemePickerProps) -> Element {
    rsx! {
        fieldset { class: "p-4 fieldset bg-base-300 rounded w-fit",
            legend { class: "fieldset-legend", "Theme" }
        
            label { 
                class: "flex gap-2 items-center cursor-pointer",
                input {
                    class: "radio radio-sm checked:radio-primary", 
                    type: "radio", 
                    id: "auto", 
                    name: "theme", 
                    value: "auto", 
                    checked: props.current_theme.is_none(),
                    oninput: move |_| props.change_theme.call(None)
                }
                span { "Follow system theme" }
            }

            label { 
                class: "flex gap-2 items-center cursor-pointer",
                input {
                    class: "radio radio-sm checked:radio-primary", 
                    type: "radio", 
                    id: "light", 
                    name: "theme", 
                    value: "light", 
                    checked: props.current_theme == Some(Theme::Light),
                    oninput: move |_| props.change_theme.call(Some(Theme::Light))
                }
                span { "Light" }
            }

            label { 
                class: "flex gap-2 items-center cursor-pointer",
                input {
                    class: "radio radio-sm checked:radio-primary", 
                    type: "radio", 
                    id: "dark", 
                    name: "theme", 
                    value: "dark", 
                    checked: props.current_theme == Some(Theme::Dark),
                    oninput: move |_| props.change_theme.call(Some(Theme::Dark))
                }
                span { "Dark" }
            }
        }
    }
}
