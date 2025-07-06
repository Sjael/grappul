#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::components::{Item, Ability};
use crate::data::aliases::{ITEM_ALIASES, ABILITY_ALIASES};
use std::collections::HashSet;
use once_cell::sync::Lazy;

pub const INLINE_ICON_SIZE: u32 = 24;

#[derive(Clone, Copy, PartialEq)]
pub struct IconDisplayOptions {
    pub show_text: bool,
}

impl Default for IconDisplayOptions {
    fn default() -> Self {
        Self { show_text: true }
    }
}

// Build sets of all known items and abilities (including aliases)
static KNOWN_ITEMS: Lazy<HashSet<String>> = Lazy::new(|| {
    ITEM_ALIASES.keys().cloned().collect()
});

static KNOWN_ABILITIES: Lazy<HashSet<String>> = Lazy::new(|| {
    ABILITY_ALIASES.keys().cloned().collect()
});

// Format names for display
fn format_name_for_display(name: &str, is_item: bool) -> String {
    // Get the appropriate alias map and resolve the canonical name
    let canonical = if is_item {
        ITEM_ALIASES.get(&name.to_lowercase())
    } else {
        ABILITY_ALIASES.get(&name.to_lowercase())
    }
    .cloned()
    .unwrap_or_else(|| name.to_string());
    
    // Format the canonical name nicely
    canonical.replace('_', " ").replace('-', " ")
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

/// Parses text and adds inline icons next to item/ability names
/// Returns a Fragment that can be rendered
pub fn parse_text_with_icons(text: &str, options: IconDisplayOptions) -> Element {
    rsx! {
        {
            let words: Vec<&str> = text.split_whitespace().collect();
            let mut index = 0;
            
            words.into_iter().map(move |word| {
                index += 1;
                let i = index;
                
                // Clean the word of punctuation for matching
                let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '_' && c != '-').to_lowercase();
                let prefix_punct = word.chars().take_while(|c| !c.is_alphanumeric() && *c != '_' && *c != '-').collect::<String>();
                let suffix_punct = word.chars().rev().take_while(|c| !c.is_alphanumeric() && *c != '_' && *c != '-').collect::<String>().chars().rev().collect::<String>();
                
                // Check if it's an item
                if KNOWN_ITEMS.contains(&clean_word) {
                    rsx! {
                        span { key: "{i}",
                            style: "display: inline-flex; align-items: center; gap: 2px;",
                            "{prefix_punct}"
                            if options.show_text {
                                span { 
                                    {format_name_for_display(&clean_word, true)}
                                }
                            }
                            Item { 
                                item: ITEM_ALIASES.get(&clean_word).cloned().unwrap_or(clean_word.clone()), 
                                size: INLINE_ICON_SIZE 
                            }
                            "{suffix_punct} "
                        }
                    }
                }
                // Check if it's an ability
                else if KNOWN_ABILITIES.contains(&clean_word) {
                    rsx! {
                        span { key: "{i}",
                            style: "display: inline-flex; align-items: center; gap: 2px;",
                            "{prefix_punct}"
                            if options.show_text {
                                span { 
                                    {format_name_for_display(&clean_word, false)}
                                }
                            }
                            Ability { 
                                ab: ABILITY_ALIASES.get(&clean_word).cloned().unwrap_or(clean_word.clone()), 
                                size: INLINE_ICON_SIZE 
                            }
                            "{suffix_punct} "
                        }
                    }
                }
                // Otherwise just text
                else {
                    rsx! {
                        span { key: "{i}", "{word} " }
                    }
                }
            })
        }
    }
}

#[component]
pub fn TextWithIcons(text: String, #[props(default = true)] show_text: bool) -> Element {
    let options = IconDisplayOptions { show_text };
    
    rsx! {
        span {
            style: "display: inline-flex; align-items: center; gap: 4px; flex-wrap: wrap; line-height: 1.8;",
            {parse_text_with_icons(&text, options)}
        }
    }
}

#[component]
pub fn FormattedList(items: Vec<String>, list_type: String, #[props(default = true)] show_text: bool) -> Element {
    let options = IconDisplayOptions { show_text };
    rsx! {
        div {
            h5 { "{list_type}" }
            ul {
                class: if list_type == "Tips" { "dia" } else { "" },
                style: "list-style-type: disc; padding-left: 1.5rem;",
                
                for (i, item) in items.iter().enumerate() {
                    li {
                        key: "{i}",
                        style: "margin-bottom: 0.75rem; display: flex; align-items: flex-start;",
                        
                        // Bullet point
                        span {
                            style: "margin-right: 0.5rem; flex-shrink: 0; user-select: none;",
                            if list_type == "Tips" { "⬩" } else { "•" }
                        }
                        
                        // Content with icons
                        div {
                            style: "flex: 1;",
                            TextWithIcons { text: item.clone(), show_text: show_text }
                        }
                    }
                }
            }
        }
    }
}