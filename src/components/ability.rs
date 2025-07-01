#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AbilityProps {
    ab: String,
    #[props(default = false)]
    small: bool,
}

pub fn Ability(props: AbilityProps) -> Element {
    let class = if props.small { "ability small" } else { "ability" };
    let alt_text = format!("{} ability icon", props.ab);
    
    rsx! {
        img {
            class: "{class}",
            src: "/abilities/{props.ab}.png",
            alt: "{alt_text}"
        }
    }
} 