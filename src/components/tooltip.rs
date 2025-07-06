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
            
            // Render stats
            for stat in &item.stats {
                p { 
                    key: "{stat}", 
                    class: "tooltip-stat",
                    "{stat}"
                }
            }
            
            // Render effects (passives, actives, glyphs)
            for effect in &item.effects {
                if effect.starts_with("Passive:") {
                    p { 
                        key: "{effect}",
                        class: "passive",
                        span { class: "label", "PASSIVE" }
                        " - {&effect[8..].trim()}"
                    }
                } else if effect.starts_with("Active:") {
                    p { 
                        key: "{effect}",
                        class: "active",
                        span { class: "label", "ACTIVE" }
                        " - {&effect[7..].trim()}"
                    }
                } else if effect.starts_with("Glyph:") {
                    p { 
                        key: "{effect}",
                        class: "glyph",
                        span { class: "label", "GLYPH" }
                        " - {&effect[6..].trim()}"
                    }
                } else {
                    p { 
                        key: "{effect}",
                        class: "effect",
                        "{effect}"
                    }
                }
            }
        }
    }
} 