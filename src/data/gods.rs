use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct God {
    pub display_name: String,
    pub class: String,
    pub abilities: Vec<String>,
    #[serde(default)]
    pub pantheon: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub roles: Vec<String>,
    #[serde(default)]
    pub image_path: String,
}

// Load gods from JSON file
pub static GODS: Lazy<BTreeMap<String, God>> = Lazy::new(|| {
    let gods_json = include_str!("json/gods.json");
    serde_json::from_str(gods_json).expect("Failed to parse gods.json")
});