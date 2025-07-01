#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::components::{Item, Ability};

#[derive(Props, Clone, PartialEq)]
pub struct ProsProps {
    god: String,
}

pub fn Pros(props: ProsProps) -> Element {
    if props.god.is_empty() {
        return rsx! { div {} };
    }

    rsx! {
        h5 { "Pros" }
        ol {
            match props.god.as_str() {
                "agni" => rsx! {
                    li { "Flat Pen early for wave clear and easy kills" }
                    li { 
                        "Spear of the Magus ",
                        Item { item: "spearmagus".to_string(), small: true },
                        " because our main damage is from combos"
                    }
                    li { 
                        "Calamitous ",
                        Item { item: "tahuti_calamitous", small: true },
                        " for our Fumes ",
                        Ability { ab: "fumes", small: true },
                        " combo"
                    }
                    li { 
                        "Myrdin ",
                        Item { item: "myrdin", small: true },
                        " for ult-initiating gods, perfect for Meteor ",
                        Ability { ab: "meteor", small: true }
                    }
                    li { 
                        "If the enemy team has any healing whatsoever, Divine ",
                        Item { item: "divine", small: true },
                        " first is needed, and its a cheap easy spike"
                    }
                },
                "he_bo" => rsx!(
                    li { "Full Lifesteal - your best defense is a good offense on He Bo" }
                    li { 
                        "Spear of the Magus ",
                        Item { item: "spearmagus", small: true },
                        " gives us massive damage after Waterspout ",
                        Ability { ab: "waterspout", small: true }
                    }
                    li { "Full % Penetration for damage on tanks" }
                    li { 
                        "Bumba's Spear ",
                        Item { item: "s_bumbasspear", small: true },
                        " gives great Fire Giant Secure"
                    }
                ),
                _ => rsx! {}
            }
        }
    }
} 