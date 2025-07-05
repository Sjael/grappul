#![allow(non_snake_case)]
use dioxus::prelude::*;
use web_sys::{window, MediaQueryListEvent};
use wasm_bindgen::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    Smite,
}

impl Theme {
    fn next(&self) -> Self {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Smite,
            Theme::Smite => Theme::Light,
        }
    }

    fn to_string(&self) -> String {
        match self {
            Theme::Light => "light".to_string(),
            Theme::Dark => "dark".to_string(),
            Theme::Smite => "smite".to_string(),
        }
    }

    fn from_string(s: &str) -> Self {
        match s {
            "dark" => Theme::Dark,
            "smite" => Theme::Smite,
            _ => Theme::Light,
        }
    }

    fn emoji(&self) -> &'static str {
        match self {
            Theme::Light => "🌞",
            Theme::Dark => "🌙",
            Theme::Smite => "⚔️",
        }
    }
}

fn get_system_theme() -> Theme {
    if window()
        .and_then(|window| window.match_media("(prefers-color-scheme: dark)").ok().flatten())
        .map_or(false, |query| query.matches())
    {
        Theme::Dark
    } else {
        Theme::Light
    }
}

#[component]
pub fn ThemeToggle() -> Element {
    let mut current_theme = use_signal(|| {
        // Try to get stored preference, or fall back to system theme
        window()
            .and_then(|window| window.local_storage().ok().flatten())
            .and_then(|storage| storage.get_item("theme").ok().flatten())
            .map(|theme| Theme::from_string(&theme))
            .unwrap_or_else(get_system_theme)
    });

    // Set up system theme change listener
    use_effect(move || {
        if let Some(window) = window() {
            if let Ok(Some(query)) = window.match_media("(prefers-color-scheme: dark)") {
                let mut theme_clone = current_theme.clone();
                let closure = Closure::wrap(Box::new(move |e: MediaQueryListEvent| {
                    // Only update if there's no stored preference
                    let stored_preference = web_sys::window()
                        .and_then(|w| w.local_storage().ok().flatten())
                        .and_then(|s| s.get_item("theme").ok().flatten());
                    
                    if stored_preference.is_none() {
                        theme_clone.set(if e.matches() { Theme::Dark } else { Theme::Light });
                    }
                }) as Box<dyn FnMut(MediaQueryListEvent)>);

                let callback = closure.as_ref().unchecked_ref();
                query.add_event_listener_with_callback("change", callback).ok();
                closure.forget();
            }
        }
    });

    // Effect to update body class and localStorage when theme changes
    use_effect(move || {
        let theme = current_theme();
        if let Some(window) = window() {
            // Update body class
            if let Some(document) = window.document() {
                if let Some(body) = document.body() {
                    // First remove all theme attributes
                    body.set_attribute("data-theme", &theme.to_string()).ok();
                }
            }
            
            // Update localStorage
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("theme", &theme.to_string());
            }
        }
    });

    rsx! {
        button {
            class: "theme-toggle",
            onclick: move |_| current_theme.set(current_theme().next()),
            "{current_theme().emoji()}"
        }
    }
} 