use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEntry {
    pub percent: u8,
    #[serde(default)]
    pub items: Option<Vec<String>>,
    #[serde(default)]
    pub tip: Option<String>,
}

impl PartialEq for TimelineEntry {
    fn eq(&self, other: &Self) -> bool {
        self.percent == other.percent
            && self.items == other.items
            && self.tip == other.tip
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guide {
    pub god_id: String,
    pub role: String,
    pub build: Vec<String>, // final build item IDs
    pub relics: Vec<String>, // relic item IDs
    pub timeline: Vec<TimelineEntry>,
    #[serde(default)]
    pub skill_order: Vec<u8>, // indices of abilities to level up (0-3)
    #[serde(default)]
    pub strategy: Option<String>, // Markdown strategy guide
}

impl Guide {
    pub fn new<T: Into<String>>(god_id: T, role: T) -> Self {
        Guide {
            god_id: god_id.into(),
            role: role.into(),
            build: Vec::new(),
            relics: Vec::new(),
            timeline: Vec::new(),
            skill_order: Vec::new(),
            strategy: None,
        }
    }

    pub fn with_build<T: Into<String>>(mut self, items: Vec<T>) -> Self {
        self.build = items.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_relics<T: Into<String>>(mut self, relics: Vec<T>) -> Self {
        self.relics = relics.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_timeline(mut self, timeline: Vec<TimelineEntry>) -> Self {
        self.timeline = timeline;
        self
    }

    pub fn with_skill_order(mut self, skill_order: Vec<u8>) -> Self {
        self.skill_order = skill_order;
        self
    }
    
    pub fn with_strategy<T: Into<String>>(mut self, strategy: T) -> Self {
        self.strategy = Some(strategy.into());
        self
    }
}

// Load guides from JSON file
pub static GUIDES: Lazy<HashMap<String, Vec<Guide>>> = Lazy::new(|| {
    let guides_json = include_str!("json/guides.json");
    serde_json::from_str(guides_json).expect("Failed to parse guides.json")
});