#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::{SelectedGod, SelectedRole, SelectedClass};
use crate::data::gods::GODS;
use wasm_bindgen::JsValue;

#[component]
pub fn GodGrid() -> Element {
    let mut god = use_context::<Signal<SelectedGod>>();
    let role = use_context::<Signal<SelectedRole>>();
    let class = use_context::<Signal<SelectedClass>>();
    
    // Load and filter gods data
    let filtered_gods: Vec<_> = {
        let separator = "-".repeat(30);
        web_sys::console::log_1(&JsValue::from_str(&separator));
        GODS.iter()
            .map(|(name, god_info)| {
                let class_matches = class.read().0.as_ref().map_or(true, |c| c.to_lowercase() == god_info.class.to_lowercase());
                let role_matches = role.read().0.as_ref().map_or(true, |r| {
                    let r_lower = r.to_lowercase();
                    god_info.roles.iter().any(|role| role.to_lowercase().contains(&r_lower))
                });
                
                // Print information about filtered gods
                if !class_matches || !role_matches {
                    let message = format!(
                        "{} | {} {} | {:?} {}",
                        name,
                        god_info.class,
                        if class_matches { "✅" } else { "❌" },
                        god_info.roles,
                        if role_matches { "✅" } else { "❌" }
                    );
                    web_sys::console::log_1(&JsValue::from_str(&message));
                }
                
                (name.clone(), god_info.clone(), class_matches, role_matches)
            })
            .collect()
    };

    rsx! {
        div {
            class: "god-grid",
            for (god_name, _, class_matches, role_matches) in filtered_gods {
                div {
                    key: "{god_name}",
                    class: format!("god{}{}", 
                        if god().0.as_ref().map_or(false, |g| g == &god_name) { 
                            " selected" 
                        } else { 
                            "" 
                        },
                        if !class_matches || !role_matches {
                            " filtered"
                        } else {
                            ""
                        }
                    ),
                    onclick: {
                        let god_name = god_name.clone();
                        move |_| {
                            god.set(SelectedGod(Some(god_name.clone())));
                        }
                    },
                    ondoubleclick: {
                        move |_| {
                            god.set(SelectedGod(None));
                        }
                    },
                    img {
                        src: "assets/gods/{god_name}.png"
                    }
                }
            }
        }
    }
} 