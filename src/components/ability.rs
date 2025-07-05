#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AbilityProps {
    ab: String,
    #[props(default = 60)]
    size: u32,
}

#[component]
pub fn Ability(props: AbilityProps) -> Element {
    rsx! {
        img {
            class: "ability",
            style: format!("width: {}px; height: {}px;", props.size, props.size),
            src: "assets/abilities/{props.ab}.png"
        }
    }
} 