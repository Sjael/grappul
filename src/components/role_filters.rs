#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::FilteredRole;

#[component]
pub fn RoleFilters() -> Element {
    let mut role = use_context::<Signal<FilteredRole>>();
    let roles = [
        ("Solo", "solo"),
        ("Jungle", "jungle"), 
        ("Mid", "mid"),
        ("Support", "support"),
        ("Carry", "adc"),
    ];

    rsx! {
        div {
            class: "role-filters",
            style: "display: flex; gap: 8px; align-items: center; flex-wrap: wrap; justify-content: flex-start;",
            
            for (display_name, role_id) in roles {
                button {
                    key: "{role_id}",
                    style: format!(
                        "padding: 4px 8px; border: 1px solid {}; background: {}; color: {}; border-radius: 4px; cursor: pointer; transition: all 0.2s ease; font-size: 13px; display: flex; align-items: center; gap: 4px; flex: 0 0 auto; white-space: nowrap;",
                        if role.read().0.as_ref().map_or(false, |r| r == role_id) { "var(--color-accent)" } else { "var(--color-border)" },
                        if role.read().0.as_ref().map_or(false, |r| r == role_id) { "var(--color-accent)" } else { "transparent" },
                        if role.read().0.as_ref().map_or(false, |r| r == role_id) { "white" } else { "var(--color-text-primary)" }
                    ),
                    onclick: {
                        let role_id = role_id.to_string();
                        move |_| {
                            let current = role.read().0.clone();
                            role.write().0 = match current {
                                Some(r) if r == role_id => None,
                                _ => Some(role_id.clone())
                            };
                        }
                    },
                    span { 
                        style: "font-size: 16px;",
                        {
                            match role_id {
                                "solo" => "🛡️",
                                "jungle" => "🌳",
                                "mid" => "⚡",
                                "support" => "💚",
                                "adc" => "🏹",
                                _ => "❓"
                            }
                        }
                    }
                    span { "{display_name}" }
                }
            }
        }
    }
}