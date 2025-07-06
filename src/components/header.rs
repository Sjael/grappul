#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::{Link, use_route};
use crate::components::ThemeToggle;
use crate::Route;

#[component]
pub fn Header() -> Element {
    let route = use_route::<Route>();
    let is_home = matches!(route, Route::Home);
    let is_cheatsheet = matches!(route, Route::Cheatsheet);
    
    rsx! {
        nav {
            Link { 
                to: "/",
                class: if is_home { "logo active" } else { "logo" },
                img {
                    src: "/assets/logo.svg"
                }
            }
            Link { 
                to: "/cheatsheet",
                class: if is_cheatsheet { "active" } else { "" },
                h5 { "Cheatsheet" }
            }
            ThemeToggle {}
        }
    }
} 