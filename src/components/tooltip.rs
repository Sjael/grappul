#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::data::items::ITEMS;
use std::collections::HashMap;

#[derive(Props, Clone, PartialEq)]
pub struct TooltipProps {
    item_id: String,
    #[props(default = false)]
    show_base: bool,
}

fn render_stats(stats: &HashMap<String, String>) -> Option<Element> {
    (!stats.is_empty()).then(|| rsx! {
        div {
            class: "tooltip-stats",
            for (stat, value) in stats {
                p { key: "{stat}", "{stat}: {value}" }
            }
        }
    })
}

fn render_passives(passives: Vec<&str>) -> Option<Element> {
    (!passives.is_empty()).then(|| rsx! {
        for passive in passives {
            div {
                key: "{passive}",
                class: "tooltip-passive",
                p { "PASSIVE - {passive}" }
            }
        }
    })
}

pub fn Tooltip(props: TooltipProps) -> Element {
    let item = match ITEMS.get(&props.item_id) {
        Some(item) => item,
        None => return rsx! { div { "Unknown item" } }
    };

    rsx! {
        div {
            class: "tooltip",
            div {
                class: "tooltip-header",
                h3 { "{item.display_name}" }
                if item.price > 0 {
                    p { class: "price", "{item.price} gold" }
                }
            }
            
            // Render main item stats and passives
            {render_stats(&item.stats.0)}
            {render_passives(item.get_passives())}
            
            // Render active ability if present
            if let Some((desc, cd)) = item.get_active() {
                div {
                    class: "tooltip-active",
                    p { "ACTIVE - {desc} (Cooldown: {cd}s)" }
                }
            }
            
            // Render glyph information
            if let Some((desc, base)) = item.get_glyph() {
                if props.show_base {
                    if let Some(base_item) = ITEMS.get(base) {
                        {render_stats(&base_item.stats.0)}
                        {render_passives(base_item.get_passives())}
                    }
                }
                div {
                    class: "tooltip-glyph",
                    p { "GLYPH - {desc}" }
                }
            }
        }
    }
} 