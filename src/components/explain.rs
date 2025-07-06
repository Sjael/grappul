#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::components::{Item, Ability, NoBuildCTA, MarkdownRenderer};
use crate::data::gods::GODS;
use crate::data::guides::GUIDES;
use crate::components::timelinepiece::TimelinePiece;
use crate::{SelectedGod, FilteredRole, SelectedRole};
use std::collections::HashMap;
use lazy_static::lazy_static;
use web_sys::window;
use wasm_bindgen::{JsCast, closure::Closure};

pub const INLINE_ICON_SIZE: u32 = 32;

lazy_static! {
    static ref SKILL_ORDER: HashMap<String, Vec<u8>> = {
        let json_str = include_str!("../data/json/skill_order.json");
        serde_json::from_str(json_str).unwrap()
    };
}

fn render_item_row(items: &[String], size: Option<u32>) -> Element {
    rsx! {
        div {
            class: "itemrow",
            style: "display: flex; flex-wrap: nowrap; gap: 0.75rem;",
            for item in items {
                Item { 
                    key: "{item}", 
                    item: item.clone(),
                    size: size.unwrap_or(48)
                }
            }
        }
    }
}


fn skill_point(skill_order: &[u8], skill_idx: usize, i: usize) -> Element {
    let mut level = None;
    for (level_idx, &skill) in skill_order.iter().enumerate() {
        if skill as usize == skill_idx && level_idx == i {
            level = Some(level_idx + 1);
            break;
        }
    }

    let class = match level {
        Some(_) => format!("point level skill{}", skill_idx),
        None => "point".to_string()
    };

    rsx! {  
        div {
            key: "{i}",
            class: "{class}",
            if let Some(lvl) = level {
                "{lvl}"
            }
        }
    }
}

fn skill_row(ability: &str, skill_order: &[u8], skill_idx: usize) -> Element {
    rsx! {
        div {
            class: "skillrow",
            Ability { ab: ability.to_string(), size: 24 }
            for i in 0..20 {
                {skill_point(&skill_order, skill_idx, i)}
            }
        }
    }
}


#[component]
pub fn Explain() -> Element {
    let god = use_context::<Signal<SelectedGod>>();
    let filtered_role = use_context::<Signal<FilteredRole>>();
    let mut selected_role = use_context::<Signal<SelectedRole>>();
    let mut selected_build_role = use_signal(String::new);

    // Early return if no god is selected
    let Some(god_name) = god().0.clone() else {
        return rsx! { div {} }
    };

    let god_info = match GODS.get(&god_name) {
        Some(info) => info,
        None => return rsx! { div { "No god information found" } }
    };

    let builds = GUIDES.get(&god_name);

    // Get the skill order for this god
    let skill_order = SKILL_ORDER.get(&god_name).unwrap_or(&vec![]).clone();

    let display_name = god_name.replace("_", " ");

    // Check if there are any builds
    let Some(builds) = builds else {
        return rsx! {
            div {
                class: "explain-container",
                div {
                    class: "explain-top",
                    img {
                        class: "god-img",
                        src: "assets/gods/{god_name}.png"
                    }
                    div {
                        class: "explain-title",
                        h1 { "{display_name}" }
                    }
                }
                div {
                    class: "explain-content",
                    NoBuildCTA { god_name: god_name.clone() }
                }
            }
        }
    };

    // Get unique roles for this god
    let available_roles: Vec<String> = {
        let mut roles = std::collections::HashSet::new();
        for build in builds.iter() {
            roles.insert(build.role.clone());
        }
        let mut roles: Vec<String> = roles.into_iter().collect();
        roles.sort(); // Sort for consistent ordering
        roles
    };

    // Determine which role to show based on priority:
    // 1. SelectedRole (from clicking role badges/buttons)
    // 2. FilteredRole (from role filter)
    // 3. First available role
    let current_role = selected_role.read().0.clone()
        .or_else(|| filtered_role.read().0.clone())
        .unwrap_or_else(|| {
            if !available_roles.is_empty() {
                available_roles[0].clone()
            } else {
                String::new()
            }
        });

    // Update selected_build_role if it's different
    if *selected_build_role() != current_role {
        selected_build_role.set(current_role.clone());
    }

    // Get the build for the selected role
    let build = builds.iter()
        .find(|build| build.role == current_role)
        .unwrap_or(&builds[0]);
    
    // Add keyboard navigation for roles
    use_effect({
        let available_roles = available_roles.clone();
        let selected_role = selected_role.clone();
        let build_role = build.role.clone();
        
        move || {
            if available_roles.len() <= 1 {
                return;
            }
            
            let handle_keydown = {
                let available_roles = available_roles.clone();
                let mut selected_role = selected_role.clone();
                let build_role = build_role.clone();
                
                move |event: web_sys::KeyboardEvent| {
                    // Check if we're in an input element
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
                    
                    let key = event.key();
                    
                    // Only handle left/right arrows without modifiers
                    if !event.ctrl_key() && !event.alt_key() && !event.meta_key() {
                        match key.as_str() {
                            "ArrowLeft" | "ArrowRight" => {
                                // Find current role index based on the actual displayed build
                                let current_index = available_roles.iter()
                                    .position(|r| r == &build_role)
                                    .unwrap_or(0);
                                
                                let new_index = match key.as_str() {
                                    "ArrowLeft" => {
                                        event.prevent_default();
                                        event.stop_propagation();
                                        if current_index == 0 {
                                            available_roles.len() - 1
                                        } else {
                                            current_index - 1
                                        }
                                    },
                                    "ArrowRight" => {
                                        event.prevent_default();
                                        event.stop_propagation();
                                        if current_index == available_roles.len() - 1 {
                                            0
                                        } else {
                                            current_index + 1
                                        }
                                    },
                                    _ => return,
                                };
                                
                                if let Some(new_role) = available_roles.get(new_index) {
                                    selected_role.set(SelectedRole(Some(new_role.clone())));
                                }
                            },
                            _ => {}
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
        }
    });

    rsx! {
        div {
            class: "explain-top",
            img {
                class: "god-img",
                src: "assets/gods/{god_name}.png"
            }
            div {
                class: "explain-title",
                h1 { "{display_name}" }
            }
            div {
                class: "role-buttons",
                for role_name in available_roles {
                    button {
                        key: "{role_name}",
                        class: {
                            if role_name == build.role { "role-button selected" } else { "role-button" }
                        },
                        onclick: move |_| {
                            selected_role.set(SelectedRole(Some(role_name.clone())));
                            selected_build_role.set(role_name.clone());
                        },
                        "{role_name}"
                    }
                }
            }
        }
        div {
            class: "explain-content",
            
            // Build and relics in same row
            div {
                class: "build-relics-row",
                style: "display: flex; gap: 2rem; flex-wrap: wrap; align-items: flex-start;",
                
                // Build section
                div {
                    class: "build-section",
                    style: "flex: 0 0 auto; min-width: 0;",
                    h5 { "Full build" }
                    div {
                        class: "build-items-container",
                        style: "overflow-x: auto; overflow-y: hidden;",
                        {render_item_row(&build.build, None)}
                    }
                }
                
                // Relics section
                div {
                    style: "flex: 0 0 auto;",
                    h5 { "Relics" }
                    {render_item_row(&build.relics, None)}
                }
            }

            h5 { "Timeline" }
            div {
                class: "timeline",
                for (i, entry) in build.timeline.iter().enumerate() {
                    TimelinePiece { key: "{i}", entry: entry.clone() }
                }
            }


            h5 { "Skill Order" }
            div {
                class: "grid_hold",
                for (skill_idx, ability) in god_info.abilities.iter().enumerate() {
                    {skill_row(ability, &skill_order, skill_idx + 1)}
                }
            }

            
            // Strategy Guide section if available
            if let Some(strategy) = build.strategy.as_ref() {
                if !strategy.is_empty() {
                    h5 { "Strategy Guide" }
                    div {
                        style: "padding: 16px; background: var(--color-bg-secondary); border-radius: 8px; margin-top: 16px;",
                        MarkdownRenderer { content: strategy.clone() }
                    }
                }
            }
        }
    }
} 