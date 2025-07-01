#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ItemProps {
    item: String,
    #[props(default = false)]
    small: bool,
}

pub fn Item(props: ItemProps) -> Element {
    let class = if props.small { "item small" } else { "item" };
    rsx! {
        img {
            class: "{class}",
            src: format!("/items/{}.png", props.item),
            alt: props.item
        }
    }
} 