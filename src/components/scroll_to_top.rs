#![allow(non_snake_case)]
use dioxus::prelude::*;
use web_sys::window;
use wasm_bindgen::{JsCast, closure::Closure};

#[component]
pub fn ScrollToTop() -> Element {
    let mut show_button = use_signal(|| false);
    
    // Check scroll position
    use_effect(move || {
        // Initial check
        if let Some(win) = window() {
            let scroll_y = win.scroll_y().unwrap_or(0.0);
            show_button.set(scroll_y > 200.0);
        }
        
        // Set up scroll listener
        let listener = {
            let mut show_button = show_button.clone();
            let closure = Closure::<dyn FnMut()>::new(move || {
                if let Some(win) = window() {
                    let scroll_y = win.scroll_y().unwrap_or(0.0);
                    show_button.set(scroll_y > 200.0);
                }
            });
            
            if let Some(win) = window() {
                let _ = win.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
            }
            
            closure
        };
        
        // Store the listener to prevent it from being dropped
        Box::leak(Box::new(listener));
    });
    
    // Keyboard shortcuts and arrow key handling
    use_effect(move || {
        let handle_keydown = move |event: web_sys::KeyboardEvent| {
            let key = event.key();
            
            // Check if we're in an input element
            if let Some(win) = window() {
                if let Some(doc) = win.document() {
                    if let Some(active) = doc.active_element() {
                        let tag_name = active.tag_name();
                        if tag_name.to_lowercase() == "input" || tag_name.to_lowercase() == "textarea" {
                            return;
                        }
                    }
                }
                
                // Only handle up/down arrows, not left/right
                match key.as_str() {
                    // Ctrl+Up for scroll to top
                    "ArrowUp" if event.ctrl_key() => {
                        event.prevent_default();
                        event.stop_propagation();
                        win.scroll_to_with_x_and_y(0.0, 0.0);
                    },
                    // Override default arrow key scrolling - only for up/down
                    "ArrowUp" if !event.shift_key() && !event.alt_key() => {
                        event.prevent_default();
                        let current_y = win.scroll_y().unwrap_or(0.0);
                        // Scroll up by a larger amount (300px instead of default ~40px)
                        win.scroll_to_with_x_and_y(0.0, (current_y - 300.0).max(0.0));
                    },
                    "ArrowDown" if !event.shift_key() && !event.alt_key() && !event.ctrl_key() => {
                        event.prevent_default();
                        let current_y = win.scroll_y().unwrap_or(0.0);
                        // Scroll down by a larger amount
                        win.scroll_to_with_x_and_y(0.0, current_y + 300.0);
                    },
                    // Don't handle left/right arrows - let other components handle them
                    _ => {}
                }
            }
        };
        
        let listener = {
            let closure = Closure::<dyn Fn(_)>::new(handle_keydown);
            
            if let Some(win) = window() {
                if let Some(doc) = win.document() {
                    let _ = doc.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
                }
            }
            
            closure
        };
        
        // Store the listener to prevent it from being dropped
        Box::leak(Box::new(listener));
    });
    
    rsx! {
        button {
            class: if show_button() { "scroll-to-top show" } else { "scroll-to-top" },
            style: format!("position: fixed; bottom: 24px; right: 24px; width: 48px; height: 48px; background: var(--color-accent); color: white; border: none; border-radius: 50%; cursor: pointer; display: flex; align-items: center; justify-content: center; z-index: 1000; box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15); pointer-events: {};", if show_button() { "all" } else { "none" }),
            onclick: move |_| {
                if let Some(win) = window() {
                    win.scroll_to_with_x_and_y(0.0, 0.0);
                }
            },
            
            // Up arrow icon
            svg {
                width: "24",
                height: "24",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "3",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                
                path { d: "M12 19V5M12 5l-7 7M12 5l7 7" }
            }
        }
    }
}