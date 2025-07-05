use dioxus::prelude::*;
use crate::components::{ClassFilters, ClearFilters, Explain, GodGrid, Header, RoleFilters, Tooltip};

#[component]
pub fn Cheatsheet() -> Element {
    
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