#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::{FilteredRole, FilteredClass, components::Icon};

#[component]
pub fn ClearFilters() -> Element {
    let mut role = use_context::<Signal<FilteredRole>>();
    let mut class = use_context::<Signal<FilteredClass>>();

    // Only show if either filter is active
    let show_reset = role.read().0.is_some() || class.read().0.is_some();

    rsx! {
        button {
            class: format!("clear-filters {}", if show_reset { "visible" } else { "hidden" }),
            onclick: move |_| {
                role.set(FilteredRole(None));
                class.set(FilteredClass(None));
            },
            Icon {
                name: "reset".to_string(),
                size: 16
            }
        }
    }
} 