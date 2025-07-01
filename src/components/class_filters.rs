#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::components::Icon;
use crate::SelectedClass;

pub fn ClassFilters() -> Element {
    let mut class = use_context::<Signal<SelectedClass>>();
    let classes = ["assassin", "guardian", "hunter", "mage", "warrior"];
    
    rsx! {
        div {
            class: "flex gap-2",
            for class_name in classes {
                button {
                    key: "{class_name}",
                    class: if class().0.as_ref().map_or(false, |c| c == class_name) { "p-2 rounded bg-gray-700" } else { "p-2 rounded" },
                    onclick: move |_| {
                        let current = class().0.clone();
                        class.set(SelectedClass(match current {
                            Some(c) if c == class_name => None,
                            _ => Some(class_name.to_string())
                        }));
                    },
                    Icon { name: class_name.to_string() }
                }
            }
        }
    }
} 