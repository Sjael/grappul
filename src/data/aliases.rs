use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static ALIASES: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let aliases_json = include_str!("json/aliases.json");
    serde_json::from_str(aliases_json).expect("Failed to parse aliases.json")
}); 