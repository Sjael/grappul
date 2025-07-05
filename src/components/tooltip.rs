#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::{data::items::ITEMS, TooltipPos};
use crate::HoveredItem;

#[component]
pub fn Tooltip() -> Element {
    let hovered_item = use_context::<Signal<HoveredItem>>();
    let mouse_pos = use_context::<Signal<TooltipPos>>();

    // Only render if we have an item
    let Some(item_id) = &hovered_item().0 else {
        return rsx! { div {} };
    };

    let Some(item) = ITEMS.get(item_id) else {
        return rsx! { div {} };
    };

    rsx! {
        div {
            class: "tooltip",
            style: "left: {mouse_pos.read().x + 15}px; top: {mouse_pos.read().y + 25}px; pointer-events: none;",
            div {
                class: "tooltip-header",
                h3 { "{item.display_name}" }
                if item.price > 0 {
                    span { class: "price", "{item.price}" }
                }
            }
            
            // Render main item stats and passives
            for (stat, value) in &item.stats.0 {
                p { 
                    key: "{stat:?}", 
                    class: "tooltip-stat",
                    span { 
                        class: "stat-value",
                        "{value}"
                    }
                    span { 
                        class: "stat-name",
                        "{stat.to_string()}"
                    }
                }
            }
            
            for passive in item.get_passives() {
                p { 
                    key: "{passive}",
                    class: "passive",
                    span { class: "label", "PASSIVE" }
                    " - {passive}"
                }
            }
            
            // Render active ability if present
            if let Some((desc, cd)) = item.get_active() {
                p { 
                    class: "active",
                    span { class: "label", "ACTIVE" }
                    " - {desc} "
                    span { class: "cd", "(Cooldown: {cd}s)" }
                }
            }
            
            // Render glyph information
            if let Some((desc, _)) = item.get_glyph() {
                p { 
                    class: "glyph",
                    span { class: "label", "GLYPH" }
                    " - {desc}"
                }
            }
        }
    }
} 