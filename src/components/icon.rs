#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::data::icon_paths::get_icon_paths;

#[derive(Props, Clone, PartialEq)]
pub struct IconProps {
    name: String,
    #[props(default = 24)]
    size: u32,
    #[props(default = "currentColor".to_string())]
    fill: String,
}

pub fn Icon(props: IconProps) -> Element {
    let IconProps { name, size, fill } = props;
    
    // Try to get SVG paths first, fallback to direct image if not found
    let paths = get_icon_paths(&name);
    
    match paths {
        Some(paths) => rsx! {
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "{size}",
                height: "{size}",
                fill: "{fill}",
                view_box: "0 0 24 24",
                for path in paths {
                    path {
                        d: "{path}",
                        fill: "{fill}"
                    }
                }
            }
        },
        None => {
            let img_src = format!("/icons/{}.svg", name);
            let img_style = format!("fill: {}", fill);
            rsx! {
                img {
                    class: "icon",
                    src: "{img_src}",
                    alt: "{name}",
                    width: "{size}",
                    height: "{size}",
                    style: "{img_style}"
                }
            }
        }
    }
} 