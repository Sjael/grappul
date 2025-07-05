#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::components::Icon;
use crate::SelectedClass;

#[component]
pub fn ClassFilters() -> Element {
    let mut class = use_context::<Signal<SelectedClass>>();
    let classes = [ "warrior", "assassin", "mage", "guardian", "hunter",];
    
    rsx! {
        div {
            class: "class-filters",
            for class_name in classes {
                button {    
                    key: "{class_name}",
                    class: if class.read().0.as_ref().map_or(false, |c| c == class_name) { "selected class" } else { "class" },
                    "data-class": "{class_name}",
                    onclick: move |_| {
                        let current = class.read().0.clone();
                        class.write().0 = match current {
                            Some(c) if c == class_name => None,
                            _ => Some(class_name.to_string())
                        };
                    },
                    Icon { name: class_name.to_string() }
                }
            }
        }
    }
} 