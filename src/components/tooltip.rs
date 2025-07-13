#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::{data::items::{ITEMS, Effect}, TooltipPos};
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
            if !item.stats.is_empty() {
                div {
                    class: "tooltip-stats-container",
                    {
                        // Collect and sort stats by ItemStat order
                        let mut sorted_stats: Vec<_> = item.stats.iter().collect();
                        sorted_stats.sort_by_key(|(stat_type, _)| *stat_type);
                        
                        rsx! {
                            for (stat_type, value) in sorted_stats {
                                div { 
                                    key: "{stat_type:?}", 
                                    class: "tooltip-stat-row",
                                    span {
                                        class: "stat-value",
                                        "{value}"
                                    }
                                    span {
                                        class: "stat-name",
                                        "{crate::utils::format::format_stat_name(stat_type)}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            // Render effects (passives, actives, glyphs)
            for (effect_type, description) in &item.effects {
                match effect_type {
                    Effect::Passive => rsx! {
                        div { 
                            key: "{effect_type:?}",
                            class: "passive-effect effect-row",
                            p {
                                span { 
                                    class: "label passive-label",
                                    "PASSIVE - "
                                }
                                span {
                                    class: "effect-text",
                                    "{description}"
                                }
                            }
                        }
                    },
                    Effect::Active => rsx! {
                        div { 
                            key: "{effect_type:?}",
                            class: "active-effect effect-row",
                            p {
                                span { 
                                    class: "label active-label",
                                    "ACTIVE - "
                                }
                                span {
                                    class: "effect-text",
                                    "{description}"
                                }
                            }
                        }
                    },
                    Effect::Glyph => rsx! {
                        div { 
                            key: "{effect_type:?}",
                            class: "glyph-effect effect-row",
                            p {
                                span { 
                                    class: "label glyph-label",
                                    "GLYPH - "
                                }
                                span {
                                    class: "effect-text",
                                    "{description}"
                                }
                            }
                        }
                    },
                    Effect::Aura => rsx! {
                        div { 
                            key: "{effect_type:?}",
                            class: "aura-effect effect-row",
                            p {
                                span { 
                                    class: "label aura-label",
                                    "AURA - "
                                }
                                span {
                                    class: "effect-text",
                                    "{description}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
} 