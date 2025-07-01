#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::Link;

pub fn Header() -> Element {
    rsx! {
        header {
            class: "bg-gray-800 text-white p-4",
            nav {
                class: "flex gap-4",
                Link { to: "/", class: "hover:text-gray-300", "Home" }
                Link { to: "/cheatsheet", class: "hover:text-gray-300", "Cheatsheet" }
            }
            div {
                class: "corner",
                Link {
                    to: "/",
                    div {
                        id: "logo",
                        img { src: "static/logo.svg" }
                    }
                }
            }
        }
    }
} 