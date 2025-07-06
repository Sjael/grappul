use std::collections::HashMap;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aliases {
    pub abilities: HashMap<String, Vec<String>>,
    pub items: HashMap<String, Vec<String>>,
}

pub static ALIASES: Lazy<Aliases> = Lazy::new(|| {
    let aliases_json = include_str!("json/aliases.json");
    serde_json::from_str(aliases_json).expect("Failed to parse aliases.json")
});

// Create reverse lookup maps for quick alias resolution
pub static ITEM_ALIASES: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for (canonical, aliases) in &ALIASES.items {
        for alias in aliases {
            map.insert(alias.to_lowercase(), canonical.clone());
        }
    }
    map
});

pub static ABILITY_ALIASES: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for (canonical, aliases) in &ALIASES.abilities {
        for alias in aliases {
            map.insert(alias.to_lowercase(), canonical.clone());
        }
    }
    map
});

// Utility functions to resolve aliases
pub fn resolve_item_alias(name: &str) -> String {
    ITEM_ALIASES.get(&name.to_lowercase())
        .cloned()
        .unwrap_or_else(|| name.to_string())
}

pub fn resolve_ability_alias(name: &str) -> String {
    ABILITY_ALIASES.get(&name.to_lowercase())
        .cloned()
        .unwrap_or_else(|| name.to_string())
}

// Role aliases map
pub static ROLE_ALIASES: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    // Jungle aliases
    map.insert("jg".to_string(), "jungle".to_string());
    map.insert("jung".to_string(), "jungle".to_string());
    map.insert("jungler".to_string(), "jungle".to_string());
    
    // Carry/ADC aliases
    map.insert("adc".to_string(), "carry".to_string());
    
    // Support aliases
    map.insert("supp".to_string(), "support".to_string());
    map.insert("sup".to_string(), "support".to_string());
    
    // Mid aliases
    map.insert("middle".to_string(), "mid".to_string());
    
    map
});

// Utility function to resolve role aliases
pub fn resolve_role_alias(role: &str) -> String {
    ROLE_ALIASES.get(&role.to_lowercase())
        .cloned()
        .unwrap_or_else(|| role.to_string())
} 