#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::components::{Item, Ability};
use crate::components::explain::INLINE_ICON_SIZE as SIZE;

#[derive(Props, Clone, PartialEq)]
pub struct ProsProps {
    god: String,
}

#[component]
pub fn Pros(props: ProsProps) -> Element {
    if props.god.is_empty() {
        return rsx! { div {} };
    }

    rsx! {
        h5 { "Pros" }
        ul {
            class: "",
            match props.god.as_str() {
                "agni" => rsx! {
                    li { "Flat Pen early for wave clear and easy kills" }
                    li { 
                        "Spear of the Magus ",
                        Item { item: "spearmagus".to_string(), size: SIZE },
                        " because our main damage is from combos"
                    }
                    li { 
                        "Calamitous ",
                        Item { item: "tahuti_calamitous", size: SIZE },
                        " for our Fumes ",
                        Ability { ab: "fumes", size: SIZE },
                        " combo"
                    }
                    li { 
                        "Myrdin ",
                        Item { item: "myrdin", size: SIZE },
                        " for ult-initiating gods, perfect for Meteor ",
                        Ability { ab: "meteor", size: SIZE }
                    }
                    li { 
                        "If the enemy team has any healing whatsoever, Divine ",
                        Item { item: "divine", size: SIZE },
                        " first is needed, and its a cheap easy spike"
                    }
                },
                "he_bo" => rsx!(
                    li { "Full Lifesteal - your best defense is a good offense on He Bo" }
                    li { 
                        "Spear of the Magus ",
                        Item { item: "spearmagus", size: SIZE },
                        " gives us massive damage after Waterspout ",
                        Ability { ab: "waterspout", size: SIZE }
                    }
                    li { "Full % Penetration for damage on tanks" }
                    li { 
                        "Bumba's Spear ",
                        Item { item: "s_bumbasspear", size: SIZE },
                        " gives great Fire Giant Secure"
                    }
                ),
                _ => rsx! {}
            }
        }
    }
} 