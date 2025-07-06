#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn NoBuildCTA(god_name: String) -> Element {

    rsx! {
        div {
            style: "display: flex; align-items: center; justify-content: space-between; padding: 8px 12px; background: var(--color-bg-tertiary); border-radius: 6px; gap: 8px;",
            
            span {
                style: "font-size: 12px; color: var(--color-text-secondary);",
                "No guides yet"
            }
            
            button {
                style: "padding: 4px 12px; background: var(--color-accent); color: white; border: none; border-radius: 4px; font-size: 11px; font-weight: 500; cursor: pointer; transition: all 0.2s ease; display: flex; align-items: center; gap: 4px;",
                onclick: move |_| {
                    navigator().push(Route::GuideCreator);
                },
                
                // Plus icon
                svg {
                    width: "12",
                    height: "12",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "3",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    
                    line { x1: "12", y1: "5", x2: "12", y2: "19" }
                    line { x1: "5", y1: "12", x2: "19", y2: "12" }
                }
                
                "Create"
            }
        }
    }
}