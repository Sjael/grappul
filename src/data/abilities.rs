use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Ability {
    pub display_name: String,
    pub description: String,
    pub details: HashMap<String, serde_json::Value>,
}

// Load abilities from JSON file
pub static ABILITIES: Lazy<HashMap<String, Ability>> = Lazy::new(|| {
    let abilities_json = include_str!("json/abilities.json");
    serde_json::from_str(abilities_json).expect("Failed to parse abilities.json")
});