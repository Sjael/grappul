#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::HoveredItem;
use crate::data::aliases::resolve_ability_alias;

#[derive(Props, Clone, PartialEq)]
pub struct AbilityProps {
    ab: String,
    #[props(default = 60)]
    size: u32,
}

#[component]
pub fn Ability(props: AbilityProps) -> Element {
    let mut item = use_context::<Signal<HoveredItem>>();
    
    // Resolve alias immediately
    let resolved_name = resolve_ability_alias(&props.ab);
    let mut tried_original = use_signal(|| false);
    let mut current_src = use_signal(|| format!("assets/abilities/{}.png", resolved_name));
    
    // Clone values for closures
    let ab_for_error = props.ab.clone();
    let ab_for_enter = props.ab.clone();
    let resolved_for_enter = resolved_name.clone();
    
    rsx! {
        img {
            class: "ability",
            style: format!("width: {}px; height: {}px;", props.size, props.size),
            src: "{current_src}",
            "data-tooltip": if tried_original() { props.ab.clone() } else { resolved_name.clone() },
            onerror: move |_| {
                // If the resolved alias fails and we haven't tried the original yet
                if !tried_original() {
                    tried_original.set(true);
                    current_src.set(format!("assets/abilities/{}.png", ab_for_error));
                }
            },
            onmouseenter: move |_| {
                let tooltip_name = if tried_original() { ab_for_enter.clone() } else { resolved_for_enter.clone() };
                item.write().0 = Some(tooltip_name);
            },
            onmouseleave: move |_| {
                item.write().0 = None;
            }
        }
    }
} 