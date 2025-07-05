use dioxus::prelude::*;
use crate::components::{Header, ClassFilters, RoleFilters, Explain, GodGrid, ClearFilters, Tooltip};

#[component]
pub fn Home() -> Element {

    rsx! {
        div {
            class: "container",
            div {
                class: "grid",
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
                
                // Main content area
                div {
                    class: "main-content",

                    Explain {}
                }
            }
        }
        Tooltip {}
    }
} 