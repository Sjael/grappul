#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::components::Item;
use crate::data::guides::TimelineEntry;

#[derive(Props, Clone, PartialEq)]
pub struct TimelinePieceProps {
    entry: TimelineEntry,
}

pub fn TimelinePiece(props: TimelinePieceProps) -> Element {
    let left_style = format!("left: {}%", props.entry.percent);

    rsx! {
        div {
            class: "entry",
            style: left_style,
            div {
                class: "tick-h",
                div {
                    class: "tick"
                }
                if let Some(items) = &props.entry.items {
                    for item in items {
                        Item {
                            item: item.clone(),
                            small: true
                        }
                    }
                }
            }
        }
    }
} 