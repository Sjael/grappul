use dioxus::prelude::*;
use crate::components::{Tooltip, ClassFilters, RoleFilters, Header, Item};
use crate::data::gods::GODS;
use crate::data::guides::GUIDES;
use crate::{FilteredClass, FilteredRole, SelectedRole, SelectedGod};
use crate::data::aliases::resolve_role_alias;
use web_sys::window;
use wasm_bindgen::{JsCast, closure::Closure};

#[component]
pub fn Cheatsheet() -> Element {
    // Use shared filter state
    let filtered_role = use_context::<Signal<FilteredRole>>();
    let selected_role = use_context::<Signal<SelectedRole>>();
    let class = use_context::<Signal<FilteredClass>>();
    let mut search = use_signal(|| String::new());
    let search_input_id = "cheatsheet-search-input";
    
    // Set up keyboard event listener to focus search on any letter key
    use_effect(move || {
        let handle_keydown = {
            let search_input_id = search_input_id;
            move |event: web_sys::KeyboardEvent| {
                // Check if it's a single letter key and no modifiers
                if event.key().len() == 1 && 
                   event.key().chars().next().map_or(false, |c| c.is_alphabetic()) &&
                   !event.ctrl_key() && !event.alt_key() && !event.meta_key() {
                    
                    // Check if we're already in an input element
                    if let Some(win) = window() {
                        if let Some(doc) = win.document() {
                            if let Some(active) = doc.active_element() {
                                let tag_name = active.tag_name();
                                if tag_name.to_lowercase() == "input" || tag_name.to_lowercase() == "textarea" {
                                    return;
                                }
                            }
                            
                            // Focus the search input
                            if let Some(search_input) = doc.get_element_by_id(&search_input_id) {
                                if let Ok(input) = search_input.dyn_into::<web_sys::HtmlInputElement>() {
                                    let _ = input.focus();
                                }
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
    
    // Get filtered gods
    let search_term = search.read().to_lowercase();
    let filter_role = filtered_role.read();
    let sel_role = selected_role.read();
    let selected_class = class.read();
    
    let mut filtered_gods: Vec<_> = GODS.iter()
        .filter(|(name, god)| {
            let name_matches = name.contains(&search_term);
            
            // Check if god has builds for the filtered role
            let role_matches = filter_role.0.as_ref().map_or(true, |r| {
                let resolved_role = resolve_role_alias(r);
                
                // Check if god has any guides for this role
                if let Some(guides) = GUIDES.get(*name) {
                    guides.iter().any(|guide| guide.role == resolved_role)
                } else {
                    false
                }
            });
            
            let class_matches = selected_class.0.as_ref().map_or(true, |c| c.to_lowercase() == god.class.to_lowercase());
            
            name_matches && role_matches && class_matches
        })
        .collect();
    
    // Sort gods alphabetically for scrolling
    filtered_gods.sort_by(|a, b| a.0.cmp(b.0));

    rsx! {
        div {
            class: "cheatsheet-container",
            style: "display: flex; flex-direction: column; height: 100vh; background-color: var(--color-bg-primary); color: var(--color-text-primary); position: relative;",
            
            // Header with navigation
            div {
                class: "cheatsheet-header",
                style: "background: var(--color-bg-secondary);",
                
                // Navigation row
                div {
                    style: "padding: 12px 24px; border-bottom: 1px solid var(--color-border);",
                    Header {}
                }
                
                // Filters row
                div {
                    style: "padding: 16px 24px; display: flex; align-items: center; gap: 24px; background: var(--color-bg-tertiary); border-bottom: 1px solid var(--color-border);",
                    
                    // Class filters section
                    div {
                        style: "display: flex; align-items: center; gap: 12px;",
                        
                        span {
                            style: "font-size: 12px; text-transform: uppercase; letter-spacing: 0.5px; color: var(--color-text-secondary); font-weight: 600;",
                            "Class:"
                        }
                        
                        ClassFilters {}
                    }
                    
                    // Divider
                    div {
                        style: "width: 1px; height: 24px; background: var(--color-border-light);",
                    }
                    
                    // Role filters section
                    div {
                        style: "display: flex; align-items: center; gap: 12px;",
                        
                        span {
                            style: "font-size: 12px; text-transform: uppercase; letter-spacing: 0.5px; color: var(--color-text-secondary); font-weight: 600;",
                            "Role:"
                        }
                        
                        RoleFilters {}
                    }

                    // Spacer
                    div { style: "flex: 1;" }

                    // Search bar with icon
                    div {
                        style: "position: relative; width: 280px; max-width: 100%;",
                        
                        // Search icon
                        svg {
                            style: "position: absolute; left: 12px; top: 50%; transform: translateY(-50%); width: 16px; height: 16px; opacity: 0.5;",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            
                            circle { cx: "11", cy: "11", r: "8" }
                            path { d: "m21 21-4.35-4.35" }
                        }
                        
                        input {
                            id: "{search_input_id}",
                            r#type: "text",
                            placeholder: "Search gods by name...",
                            value: "{search}",
                            oninput: move |evt| {
                                let value = evt.value();
                                search.set(value.clone());
                                
                                // Auto-scroll to first matching god if single letter
                                if value.len() == 1 {
                                    if let Some(first_char) = value.chars().next() {
                                        if first_char.is_alphabetic() {
                                            let target_letter = first_char.to_uppercase().to_string();
                                            // Find first god starting with this letter
                                            if let Some((name, _)) = filtered_gods.iter()
                                                .find(|(name, _)| name.to_uppercase().starts_with(&target_letter)) {
                                                // Scroll to the god element
                                                if let Some(win) = window() {
                                                    if let Some(doc) = win.document() {
                                                        // Use the god's name as ID (you'll need to add IDs to god elements)
                                                        if let Some(element) = doc.query_selector(&format!("[data-god-name=\"{}\"]", name)).ok().flatten() {
                                                            element.scroll_into_view();
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            style: "width: 100%; padding: 8px 12px 8px 36px; border: 1px solid var(--color-border); border-radius: 8px; background: var(--color-bg-primary); color: var(--color-text-primary); font-size: 14px; transition: all 0.2s ease;",
                        }
                    }
                    
                    span {
                        style: "padding: 4px 12px; background: var(--color-accent); color: white; border-radius: 12px; font-size: 12px; font-weight: 600;",
                        "{filtered_gods.len()}"
                    }
                }
            }
            
            // Grid content
            div {
                class: "cheatsheet-grid-container",
                style: "flex: 1; overflow-y: auto; padding: 12px; background: var(--color-bg-primary);",
                
                if filtered_gods.is_empty() {
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100%; text-align: center;",
                        
                        // Empty state illustration
                        svg {
                            width: "120",
                            height: "120",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "var(--color-text-secondary)",
                            stroke_width: "1.5",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            style: "opacity: 0.3; margin-bottom: 16px;",
                            
                            path { d: "M21 21l-6-6m6 6v-4.8m0 4.8h-4.8" }
                            circle { cx: "11", cy: "11", r: "8" }
                            path { d: "M11 8v3m0 4h.01" }
                        }
                        
                        h3 {
                            style: "margin: 0 0 8px 0; color: var(--color-text-primary); font-size: 18px;",
                            "No gods found"
                        }
                        
                        p {
                            style: "color: var(--color-text-secondary); margin: 0;",
                            "Try adjusting your filters or search terms"
                        }
                    }
                } else {
                    // Single unified grid without letter sections
                    div {
                        class: "cheatsheet-grid",
                        style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 4px;",
                        
                        for (name, god) in &filtered_gods {
                            div {
                                "data-god-name": "{name}",
                                CheatsheetCard {
                                    god_name: (*name).clone(),
                                    god_info: (*god).clone(),
                                    filtered_role: filter_role.0.clone(),
                                    selected_role: sel_role.0.clone(),
                                    filtered_class: selected_class.0.clone(),
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Tooltip {}
    }
}

#[component]
fn CheatsheetCard(
    god_name: String, 
    god_info: crate::data::gods::God,
    filtered_role: Option<String>,
    selected_role: Option<String>,
    filtered_class: Option<String>,
) -> Element {
    let mut selected_god = use_context::<Signal<SelectedGod>>();
    let mut selected_role_signal = use_context::<Signal<SelectedRole>>();
    let mut is_hovered = use_signal(|| false);
    
    // Get all guides for the god (no filtering)
    let all_guides = GUIDES.get(&god_name)
        .map(|guides| guides.iter().collect::<Vec<_>>())
        .unwrap_or_default();
    
    let has_builds_for_filters = !all_guides.is_empty();
    
    // Get unique roles from all guides for this god
    let mut unique_roles: Vec<String> = if has_builds_for_filters {
        let mut roles = std::collections::HashSet::new();
        for guide in &all_guides {
            roles.insert(guide.role.clone());
        }
        roles.into_iter().collect()
    } else {
        god_info.roles.clone()
    };
    
    // Sort roles alphabetically for consistent ordering
    unique_roles.sort();
    
    let display_name = god_info.display_name.clone();
    
    rsx! {
        div {
            class: "cheatsheet-card",
            style: format!("
                border: none;
                border-bottom: 1px solid var(--color-border-light);
                padding: 8px 0;
                background: transparent;
                opacity: {};
                position: relative;
                overflow: hidden;
                width: 100%;
            ", 
                if has_builds_for_filters { "1" } else { "0.5" }
            ),
            onmouseenter: move |_| is_hovered.set(true),
            onmouseleave: move |_| is_hovered.set(false),
            
            // Header with god icon and name
            div {
                style: "display: flex; align-items: center; gap: 8px; margin-bottom: 6px;",
                
                // God icon
                img {
                    src: "assets/gods/{god_name}.png",
                    alt: "{display_name}",
                    style: "width: 24px; height: 24px; border-radius: 2px;",
                }
                
                // God name and roles
                div {
                    style: "flex: 1; display: flex; align-items: center; gap: 8px;",
                    
                    h3 {
                        style: "margin: 0; font-size: 14px; font-weight: 600; color: var(--color-text-primary);",
                        "{display_name}"
                    }
                    
                    // Role badges - inline with name
                    if has_builds_for_filters {
                        div {
                            style: "display: flex; gap: 4px; flex-wrap: wrap;",
                            
                            for role in &unique_roles {
                                RoleBadgeButton {
                                    role: role.clone()
                                }
                            }
                        }
                    } else {
                        span {
                            style: "font-size: 12px; color: var(--color-text-secondary);",
                            "No guides yet"
                        }
                    }
                }
                
                // Create button - only show on hover when no builds
                if !has_builds_for_filters && is_hovered() {
                    button {
                        style: "padding: 4px 12px; background: var(--color-accent); color: white; border: none; border-radius: 4px; font-size: 11px; font-weight: 500; cursor: pointer; transition: all 0.2s ease; display: flex; align-items: center; gap: 4px; opacity: 0; animation: fadeIn 0.2s ease forwards;",
                        onclick: move |e| {
                            e.stop_propagation();
                            navigator().push(Route::GuideCreator);
                        },
                        
                        // Plus icon
                        svg {
                            width: "12",
                            height: "12",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "3",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            
                            line { x1: "12", y1: "5", x2: "12", y2: "19" }
                            line { x1: "5", y1: "12", x2: "19", y2: "12" }
                        }
                        
                        "Create"
                    }
                }
            }
            
            // Show all builds for this god
            if !all_guides.is_empty() {
                div {
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    
                    for (_idx, guide_data) in all_guides.iter().enumerate() {
                        // Clickable build container
                        div {
                            class: "build-container",
                            style: "padding: 4px; border-radius: 4px; cursor: pointer; transition: all 0.2s ease;",
                            onclick: {
                                let god_name = god_name.clone();
                                let role = guide_data.role.clone();
                                move |_| {
                                    selected_god.set(SelectedGod(Some(god_name.clone())));
                                    selected_role_signal.set(SelectedRole(Some(role.clone())));
                                    navigator().push(Route::Home);
                                }
                            },
                            
                            div {
                                style: "display: flex; flex-direction: column; gap: 4px;",
                                
                                // Build items row with role indicator
                                div {
                                    style: "display: flex; align-items: center; gap: 8px;",
                                    
                                    // Build items
                                    if !guide_data.build.is_empty() {
                                        div {
                                            class: "item-grid",
                                            style: "display: grid; grid-template-columns: repeat(6, 1fr); gap: 2px;",
                                            
                                            for item in guide_data.build.iter().take(6) {
                                                Item { 
                                                    item: item.clone(),
                                                    size: 24
                                                }
                                            }
                                        }
                                    }
                                    
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RoleBadgeButton(role: String) -> Element {
    let mut selected_role_signal = use_context::<Signal<SelectedRole>>();
    let selected_role = use_context::<Signal<SelectedRole>>();
    
    let is_selected = selected_role.read().0.as_ref() == Some(&role);
    
    rsx! {
        button {
            style: if is_selected {
                "font-size: 11px; padding: 4px 10px; background: var(--color-accent); border-radius: 12px; color: white; font-weight: 600; border: none; cursor: pointer; transition: all 0.2s ease; transform: scale(1.05);"
            } else {
                "font-size: 11px; padding: 4px 10px; background: var(--color-accent-alpha); border-radius: 12px; color: var(--color-accent); font-weight: 600; border: none; cursor: pointer; transition: all 0.2s ease;"
            },
            class: "role-badge-btn",
            onclick: move |e| {
                e.stop_propagation();
                selected_role_signal.set(SelectedRole(Some(role.clone())));
            },
            "{role}"
        }
    }
}

use crate::Route;