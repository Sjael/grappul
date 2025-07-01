#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::components::{Icon, Item, Ability, Pros};
use crate::data::gods::GODS;
use crate::data::guides::GUIDES;
use crate::components::timelinepiece::TimelinePiece;
use crate::{SelectedGod, SelectedRole};

fn render_item_row(items: &[String], small: bool) -> Element {
    rsx! {
        div {
            class: "itemrow",
            for item in items {
                Item { key: "{item}", item: item.clone(), small: small }
            }
        }
    }
}


fn skill_point(skill_order: &[u8], skill_idx: usize, i: usize) -> Element {
    let is_leveled = skill_order.iter().enumerate()
        .any(|(level, &skill)| level == i && skill as usize == skill_idx);
    let level = i + 1;
    rsx! {  
        div {
            key: "{i}",
            class: if is_leveled { "point level" } else { "point" },
            if is_leveled {
                "{level}"
            }
        }
    }
}

fn skill_row(ability: &str, skill_order: &[u8], skill_idx: usize) -> Element {
    rsx! {
        div {
            class: "skillrow",
            div {
                class: "labelrow",
                Ability { ab: ability.to_string(), small: true }
                p { "{ability}" }
            }
            div {
                class: "rowpoints",
                for i in 0..20 {
                    {skill_point(&skill_order, skill_idx, i)}
                }
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
                        Ability { ab: ability1.to_string(), small: true }
                    }
                    if !ability2.is_empty() {
                        Ability { ab: ability2.to_string(), small: true }
                    }
                    " {text}"
                }
            }
        }
    }
}

pub fn Explain() -> Element {
    let god = use_context::<Signal<SelectedGod>>();
    let role = use_context::<Signal<SelectedRole>>();

    // Early return if no god or role is selected
    let Some(god_name) = god().0.clone() else {
        return rsx! { div { "No god selected" } }
    };
    let Some(role_name) = role().0.clone() else {
        return rsx! { div { "No role selected" } }
    };

    let god_info = match GODS.get(&god_name) {
        Some(info) => info,
        None => return rsx! { div { "No god information found" } }
    };

    let builds = match GUIDES.get(&god_name) {
        Some(builds) => builds,
        None => return rsx! { div { "No build information found" } }
    };

    // Find the build for the selected role
    let build = builds.iter()
        .find(|build| build.role == *role_name)
        .unwrap_or(&builds[0]); // Default to first build if role not found

    let display_name = god_name.replace("_", " ");

    rsx! {
        div {
            class: "explain-top",
            div {
                img {
                    class: "god-img",
                    src: "/gods/{god_name}.png",
                    alt: "{display_name}"
                }
            }
            div {
                class: "explain-title",
                div {
                    h1 { "{display_name}" }
                }
                div {
                    class: "topdetails",
                }
            }
        }
        div {
            class: "explain-content",
            div {
                class: "left",
                div {
                    class: "frow",
                    div {
                        h5 { "Start" }
                        div {
                            class: "itemrow",
                            if let Some(entry) = build.timeline.get(0) {
                                if let Some(items) = &entry.items {
                                    for item in items {
                                        Item { item: item.clone(), small: true }
                                    }
                                }
                                Icon { name: "arrow".to_string() }
                                if let Some(next_entry) = build.timeline.get(1) {
                                    if let Some(items) = &next_entry.items {
                                        for item in items {
                                            Item { item: item.clone(), small: true }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div {
                        h5 { "Relics" }
                        {render_item_row(&build.relics, true)}
                    }
                }

                h5 { "Full build" }
                {render_item_row(&build.build, true)}

                h5 { "Timeline" }
                div {
                    class: "timeline",
                    for (i, entry) in build.timeline.iter().enumerate() {
                        TimelinePiece { key: "{i}", entry: entry.clone() }
                    }
                }

                // Skill order section
                if !build.skill_order.is_empty() {
                    h5 { "Skill Order" }
                    div {
                        class: "grid_hold",
                        for (skill_idx, ability) in god_info.abilities.iter().enumerate() {
                            {skill_row(ability, &build.skill_order, skill_idx)}
                        }
                    }
                }
            }

            div {
                class: "right",
                Pros { god: god_name.clone() }
                h5 { "Tips and Tricks" }
                {render_tips(&god_name)}
            }
        }
    }
} 