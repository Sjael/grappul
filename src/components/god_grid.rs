#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::{SelectedGod, FilteredRole, FilteredClass};
use crate::data::gods::GODS;
use crate::data::guides::GUIDES;
use crate::data::aliases::resolve_role_alias;
use crate::utils::format_god_image_name;
use wasm_bindgen::{JsValue, JsCast, closure::Closure};
use web_sys::window;

#[component]
pub fn GodGrid() -> Element {
    let mut god = use_context::<Signal<SelectedGod>>();
    let role = use_context::<Signal<FilteredRole>>();
    let class = use_context::<Signal<FilteredClass>>();
    
    // Set up keyboard event listener to jump to god on letter press
    use_effect(move || {
        let handle_keydown = move |event: web_sys::KeyboardEvent| {
            // Check if we're already in an input element
            if let Some(win) = window() {
                if let Some(doc) = win.document() {
                    if let Some(active) = doc.active_element() {
                        let tag_name = active.tag_name();
                        if tag_name.to_lowercase() == "input" || tag_name.to_lowercase() == "textarea" {
                            return;
                        }
                    }
                }
            }
            
            // Check if it's a single letter key and no modifiers
            if event.key().len() == 1 && 
               event.key().chars().next().map_or(false, |c| c.is_alphabetic()) &&
               !event.ctrl_key() && !event.alt_key() && !event.meta_key() {
                
                if let Some(win) = window() {
                    if let Some(doc) = win.document() {
                        // Find first god starting with this letter
                        let target_letter = event.key().to_lowercase();
                        let first_god = GODS.keys()
                            .find(|name| name.to_lowercase().starts_with(&target_letter));
                        
                        if let Some(god_name) = first_god {
                            // Find the god element and scroll to it
                            if let Ok(Some(element)) = doc.query_selector(&format!("[key=\"{}\"]", god_name)) {
                                element.scroll_into_view();
                            }
                        }
                    }
                }
            }
        };
        
        let listener = {
            let closure = Closure::<dyn FnMut(_)>::new(handle_keydown);
            
            if let Some(win) = window() {
                if let Some(doc) = win.document() {
                    let _ = doc.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
                }
            }
            
            closure
        };
        
        // Store the listener to prevent it from being dropped
        Box::leak(Box::new(listener));
    });
    
    // Load and filter gods data
    let filtered_gods: Vec<_> = {
        let separator = "-".repeat(30);
        web_sys::console::log_1(&JsValue::from_str(&separator));
        GODS.iter()
            .map(|(name, god_info)| {
                let class_matches = class.read().0.as_ref().map_or(true, |c| c.to_lowercase() == god_info.class.to_lowercase());
                let role_matches = role.read().0.as_ref().map_or(true, |r| {
                    let resolved_role = resolve_role_alias(r);
                    let r_lower = resolved_role.to_lowercase();
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
                
                // Check if god has any builds for the selected filters
                let has_builds = GUIDES.get(name)
                    .map(|guides| {
                        guides.iter().any(|g| {
                            let build_role_matches = role.read().0.as_ref().map_or(true, |r| {
                                let resolved_role = resolve_role_alias(r);
                                g.role.contains(&resolved_role)
                            });
                            build_role_matches
                        })
                    })
                    .unwrap_or(false);
                
                (name.clone(), god_info.clone(), class_matches, role_matches, has_builds)
            })
            .collect::<Vec<_>>()
    };
    
    // Sort gods alphabetically for keyboard navigation
    let mut filtered_gods = filtered_gods;
    filtered_gods.sort_by(|a, b| a.0.cmp(&b.0));

    rsx! {
        div {
            class: "god-grid",
            for (god_name, _, class_matches, role_matches, has_builds) in filtered_gods {
                div {
                    key: "{god_name}",
                    class: format!("god{}{}", 
                        if god().0.as_ref().map_or(false, |g| g == &god_name) { 
                            " selected" 
                        } else { 
                            "" 
                        },
                        if !class_matches || !role_matches || !has_builds {
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
                        src: format!("/assets/gods/{}.png", format_god_image_name(&god_name))
                    }
                }
            }
        }
    }
} 