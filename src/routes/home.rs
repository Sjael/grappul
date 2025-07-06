use dioxus::prelude::*;
use crate::components::{Header, ClassFilters, RoleFilters, Explain, GodGrid, ClearFilters, Tooltip};

#[component]
pub fn Home() -> Element {
    let selected_god = use_context::<Signal<crate::SelectedGod>>();
    let has_selection = selected_god().0.is_some();

    rsx! {
        div {
            class: "container",
            div {
                class: if has_selection { "grid has-selection" } else { "grid no-selection" },
                // Sidebar with filters
                div {
                    class: "sidebar",
                    Header {}
                    ClassFilters {}
                    div {
                        class: "filters-container",
                        RoleFilters {}
                        ClearFilters {}
                    }
                    GodGrid {}
                }
                
                // Main content area - only render if god is selected
                if has_selection {
                    div {
                        class: "main-content",
                        Explain {}
                    }
                }
            }
        }
        Tooltip {}
    }
} 