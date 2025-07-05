#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use crate::components::ThemeToggle;

#[component]
pub fn Header() -> Element {
    rsx! {
        nav {
            Link { 
                to: "/",
                class: "logo",
                img {
                    src: "assets/logo.svg"
                }
            }
            Link { 
                to: "/cheatsheet",
                h5 { "Cheatsheet" }
            }
            ThemeToggle {}
        }
    }
} 