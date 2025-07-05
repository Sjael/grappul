#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::SelectedRole;

struct RolePathData {
    id: String,
    d: Vec<String>,
}

#[component]
pub fn RoleFilters() -> Element {
    let mut role = use_context::<Signal<SelectedRole>>();
    let roles = [
        RolePathData { 
            id: "jungle".to_string(), 
            d: vec![
                "M37 16.9 37 47.1 52.1 32 37 16.9z".to_string(),
                "M27 47.1 27 16.9 11.9 32 27 47.1z".to_string()
            ]
        },
        RolePathData { 
            id: "adc".to_string(), 
            d: vec!["M32 52.1 32 62 62 32 52.1 32 32 52.1z".to_string()]
        },
        RolePathData { 
            id: "support".to_string(), 
            d: vec!["M32 11.9 52.1 32 62 32 32 2 32 11.9z".to_string()]
        },
        RolePathData { 
            id: "solo".to_string(), 
            d: vec!["M11.9 32 32 11.9 32 2 2 32 32 62 32 52.1 11.9 32z".to_string()]
        },
        RolePathData { 
            id: "mid".to_string(), 
            d: vec!["M27 7 27 57 32 62 37 57 37 7 32 2 27 7z".to_string()]
        },
    ];

    rsx! {
        div {
            class: "role-filters",
            svg {
                id: "role-filter",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 64 64",
                for path_data in roles {
                    for d_path in &path_data.d {
                        path {
                            key: "{path_data.id}-{d_path}",
                            id: "{path_data.id}",
                            class: format!("role {}", 
                                if role.read().0.as_ref().map_or(false, |r| &r == &path_data.id.as_str()) { "selected" } else { "" }
                            ),
                            onclick: {
                                let role_id = path_data.id.clone();
                                move |_| {
                                    let current = role.read().0.clone();
                                    role.write().0 = match current {
                                        Some(r) if r == role_id => None,
                                        _ => Some(role_id.clone())
                                    };
                                }
                            },
                            d: "{d_path}"
                        }
                    }
                }
            }
        }
    }
} 