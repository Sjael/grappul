#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::components::{Item, Ability, Pros};
use crate::data::gods::GODS;
use crate::data::guides::GUIDES;
use crate::components::timelinepiece::TimelinePiece;
use crate::{SelectedGod, SelectedRole};
use std::collections::HashMap;
use lazy_static::lazy_static;

pub const INLINE_ICON_SIZE: u32 = 32;

lazy_static! {
    static ref SKILL_ORDER: HashMap<String, Vec<u8>> = {
        let json_str = include_str!("../data/json/skill_order.json");
        serde_json::from_str(json_str).unwrap()
    };
}

fn render_item_row(items: &[String]) -> Element {
    rsx! {
        div {
            class: "itemrow",
            for item in items {
                Item { key: "{item}", item: item.clone()}
            }
        }
    }
}


fn skill_point(skill_order: &[u8], skill_idx: usize, i: usize) -> Element {
    let mut level = None;
    for (level_idx, &skill) in skill_order.iter().enumerate() {
        if skill as usize == skill_idx && level_idx == i {
            level = Some(level_idx + 1);
            break;
        }
    }

    let class = match level {
        Some(_) => format!("point level skill{}", skill_idx),
        None => "point".to_string()
    };

    rsx! {  
        div {
            key: "{i}",
            class: "{class}",
            if let Some(lvl) = level {
                "{lvl}"
            }
        }
    }
}

fn skill_row(ability: &str, skill_order: &[u8], skill_idx: usize) -> Element {
    rsx! {
        div {
            class: "skillrow",
            Ability { ab: ability.to_string(), size: 24 }
            for i in 0..20 {
                {skill_point(&skill_order, skill_idx, i)}
            }
        }
    }
}

fn render_tips(god: &str) -> Element {
    let tips = match god {
        "agni" => vec![
            ("meteor", "myrdin", "Late game, dont use Meteor to clear if Fire Giant is being contested, as you will lose Myrdin buff for the fight"),
            ("fdash", "", "In Conquest, start red and ditch speed to Dash first wave safely"),
        ],
        "he_bo" => vec![
            ("waterspout", "crushing wave", "Don't Waterspout instantly into Ult, you will go under the enemy"),
            ("river", "", "Save River to cleanse slows or you're throwing"),
            ("water cannon", "crushing wave", "If you can, use Water Cannon before ulting"),
            ("crushing wave", "bancrofts", "Crushing Wave + Bancrofts gives 1/2 of your health back if low"),
        ],
        _ => vec![],
    };

    rsx! {
        ul {
            class: "dia",
            for (ability1, ability2, text) in tips {
                li {
                    key: "{ability1}",
                    if !ability1.is_empty() {
                        Ability { ab: ability1.to_string(), size: INLINE_ICON_SIZE }
                    }
                    if !ability2.is_empty() {
                        Ability { ab: ability2.to_string(), size: INLINE_ICON_SIZE }
                    }
                    " {text}"
                }
            }
        }
    }
}

#[component]
pub fn Explain() -> Element {
    let god = use_context::<Signal<SelectedGod>>();
    let mut selected_build_role = use_signal(String::new);

    // Early return if no god is selected
    let Some(god_name) = god().0.clone() else {
        return rsx! { div { "Select a god to begin" } }
    };

    let god_info = match GODS.get(&god_name) {
        Some(info) => info,
        None => return rsx! { div { "No god information found" } }
    };

    let builds = match GUIDES.get(&god_name) {
        Some(builds) => builds,
        None => return rsx! { div { "No build information found" } }
    };

    // Get the skill order for this god
    let skill_order = SKILL_ORDER.get(&god_name).unwrap_or(&vec![]).clone();

    // Get unique roles for this god
    let available_roles: Vec<String> = builds.iter()
        .map(|build| build.role.clone())
        .collect();

    // If no role is selected yet, select the first available role
    if selected_build_role().is_empty() && !available_roles.is_empty() {
        selected_build_role.set(available_roles[0].clone());
    }

    // Get the build for the selected role
    let build = builds.iter()
        .find(|build| build.role == *selected_build_role())
        .unwrap_or(&builds[0]);

    let display_name = god_name.replace("_", " ");

    rsx! {
        div {
            class: "explain-top",
            img {
                class: "god-img",
                src: "assets/gods/{god_name}.png"
            }
            div {
                class: "explain-title",
                h1 { "{display_name}" }
            }
            div {
                class: "role-buttons",
                for role_name in available_roles {
                    button {
                        key: "{role_name}",
                        class: {
                            if role_name == build.role { "role-button selected" } else { "role-button" }
                        },
                        onclick: move |_| {
                            selected_build_role.set(role_name.clone());
                        },
                        "{role_name}"
                    }
                }
            }
        }
        div {
            class: "explain-content",
            div {
                style: "display: flex; justify-content: space-between",
                div {
                    h5 { "Full build" }
                    {render_item_row(&build.build)}
                }
                div {
                    h5 { "Relics" }
                    {render_item_row(&build.relics)}
                }
            }

            h5 { "Timeline" }
            div {
                class: "timeline",
                for (i, entry) in build.timeline.iter().enumerate() {
                    TimelinePiece { key: "{i}", entry: entry.clone() }
                }
            }


            h5 { "Skill Order" }
            div {
                class: "grid_hold",
                for (skill_idx, ability) in god_info.abilities.iter().enumerate() {
                    {skill_row(ability, &skill_order, skill_idx + 1)}
                }
            }

            Pros { god: god_name.clone() }

            h5 { "Tips and Tricks" }
            {render_tips(&god_name)}
        }
    }
} 