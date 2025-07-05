#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::HoveredItem;

#[derive(Props, Clone, PartialEq)]
pub struct ItemProps {
    item: String,
    #[props(default = 60)]
    size: u32,
}

#[component]
pub fn Item(props: ItemProps) -> Element {
    let mut item = use_context::<Signal<HoveredItem>>();

    rsx! {
        img {
            class: "item",
            style: format!("width: {}px; height: {}px;", props.size, props.size),
            src: format!("assets/items/{}.png", props.item),
            "data-tooltip": props.item.clone(),
            onmouseenter: move |_| {
                item.write().0 = Some(props.item.clone());
            },
            onmouseleave: move |_| {
                item.write().0 = None;
            }
        }
    }
} 