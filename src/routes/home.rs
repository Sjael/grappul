use dioxus::prelude::*;
use crate::components::{Header, ClassFilters, RoleFilters, Explain};

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "container mx-auto px-4",
            Header {}
            // Filters
            ClassFilters {}
            RoleFilters {}
            
            Explain {}
        }
    }
} 