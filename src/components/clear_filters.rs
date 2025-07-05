#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::{SelectedRole, SelectedClass, components::Icon};

#[component]
pub fn ClearFilters() -> Element {
    let mut role = use_context::<Signal<SelectedRole>>();
    let mut class = use_context::<Signal<SelectedClass>>();

    // Only show if either filter is active
    let show_reset = role.read().0.is_some() || class.read().0.is_some();

    rsx! {
        button {
            class: format!("clear-filters {}", if show_reset { "visible" } else { "hidden" }),
            onclick: move |_| {
                role.set(SelectedRole(None));
                class.set(SelectedClass(None));
            },
            Icon {
                name: "reset".to_string(),
                size: 16
            }
        }
    }
} 