#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::components::Item;
use crate::data::guides::TimelineEntry;

#[derive(Props, Clone, PartialEq)]
pub struct TimelinePieceProps {
    entry: TimelineEntry,
}

#[component]
pub fn TimelinePiece(props: TimelinePieceProps) -> Element {
    let left_style = format!("left: {}%", props.entry.percent);

    rsx! {
        div {
            class: {
                let mut classes = String::from("entry");
                if props.entry.percent == 0 {
                    classes.push_str(" entry-start");
                }
                classes
            },
            style: left_style,
            if let Some(items) = &props.entry.items {
                div {
                    class: "items",
                    for item in items {
                        Item {
                            item: item.clone(),
                            size: 30
                        }
                    }
                }
            }
            div {
                class: "tick"
            }
        }
    }
} 