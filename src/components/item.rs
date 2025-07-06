#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::HoveredItem;
use crate::data::aliases::resolve_item_alias;

#[derive(Props, Clone, PartialEq)]
pub struct ItemProps {
    item: String,
    #[props(default = 60)]
    size: u32,
}

#[component]
pub fn Item(props: ItemProps) -> Element {
    let mut item = use_context::<Signal<HoveredItem>>();
    
    // Resolve alias immediately
    let resolved_name = resolve_item_alias(&props.item);
    let mut tried_original = use_signal(|| false);
    let mut current_src = use_signal(|| format!("assets/items/{}.png", resolved_name));
    
    // Clone values for closures
    let item_for_error = props.item.clone();
    let item_for_enter = props.item.clone();
    let resolved_for_enter = resolved_name.clone();
    
    rsx! {
        img {
            class: "item",
            style: format!("width: {}px; height: {}px;", props.size, props.size),
            src: "{current_src}",
            "data-tooltip": if tried_original() { props.item.clone() } else { resolved_name.clone() },
            onerror: move |_| {
                // If the resolved alias fails and we haven't tried the original yet
                if !tried_original() {
                    tried_original.set(true);
                    current_src.set(format!("assets/items/{}.png", item_for_error));
                }
            },
            onmouseenter: move |_| {
                let tooltip_name = if tried_original() { item_for_enter.clone() } else { resolved_for_enter.clone() };
                item.write().0 = Some(tooltip_name);
            },
            onmouseleave: move |_| {
                item.write().0 = None;
            }
        }
    }
} 