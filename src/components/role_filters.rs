#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::SelectedRole;

struct RolePathData {
    id: &'static str,
    d: &'static str,
}

pub fn RoleFilters() -> Element {
    let mut role = use_context::<Signal<SelectedRole>>();
    let roles = [
        ("jungle", RolePathData { 
            id: "jungle", 
            d: "M37 16.9 37 47.1 52.1 32 37 16.9z M27 47.1 27 16.9 11.9 32 27 47.1z" 
        }),
        ("adc", RolePathData { 
            id: "adc", 
            d: "M32 52.1 32 62 62 32 52.1 32 32 52.1z" 
        }),
        ("support", RolePathData { 
            id: "support", 
            d: "M32 11.9 52.1 32 62 32 32 2 32 11.9z" 
        }),
        ("solo", RolePathData { 
            id: "solo", 
            d: "M11.9 32 32 11.9 32 2 2 32 32 62 32 52.1 11.9 32z" 
        }),
        ("mid", RolePathData { 
            id: "mid", 
            d: "M27 7 27 57 32 62 37 57 37 7 32 2 27 7z" 
        }),
    ];

    rsx! {
        svg {
            id: "role-filter",
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 64 64",
            g {
                id: "base",
                rect {
                    x: "10.79",
                    y: "10.79",
                    width: "42.43",
                    height: "42.43",
                    transform: "translate(-13.25 32) rotate(-45)"
                }
            }
            for (role_name, path_data) in roles {
                path {
                    key: "{role_name}",
                    id: "{path_data.id}",
                    class: format!("role {}", 
                        if role().0.as_ref().map_or(false, |r| r == role_name) { "selected" } else { "" }
                    ),
                    onclick: move |_| {
                        let current = role().0.clone();
                        role.set(SelectedRole(match current {
                            Some(r) if r == role_name => None,
                            _ => Some(role_name.to_string())
                        }));
                    },
                    d: "{path_data.d}"
                }
            }
        }
    }
} 