use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct God {
    pub name: String,
    pub class: String,
    pub abilities: Vec<String>,
    pub roles: Vec<String>,
}

impl God {
    pub fn new<T: Into<String>>(name: T, class: T) -> Self {
        God {
            name: name.into(),
            class: class.into(),
            abilities: Vec::new(),
            roles: Vec::new(),
        }
    }

    pub fn with_abilities<T: Into<String>>(mut self, abilities: Vec<T>) -> Self {
        self.abilities = abilities.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_roles<T: Into<String>>(mut self, roles: Vec<T>) -> Self {
        self.roles = roles.into_iter().map(Into::into).collect();
        self
    }
}

// Helper trait for god insertion
trait GodInserter {
    fn add(&mut self, god: God) -> &mut Self;
}

impl GodInserter for BTreeMap<String, God> {
    fn add(&mut self, god: God) -> &mut Self {
        self.insert(god.name.clone(), god);
        self
    }
}

// Helper function to convert anything that can become a String
fn s<T: Into<String>>(value: T) -> String {
    value.into()
}

// Raw JSON data from the files
pub static GODS_JSON: Lazy<Value> = Lazy::new(|| {
    let gods_json = include_str!("json/gods.json");
    serde_json::from_str(gods_json).expect("Failed to parse gods.json")
});

pub static SKILLS_JSON: Lazy<Value> = Lazy::new(|| {
    let skills_json = include_str!("json/skills.json");
    serde_json::from_str(skills_json).expect("Failed to parse skills.json")
});

// Processed data with combined information
pub static GODS: Lazy<BTreeMap<String, God>> = Lazy::new(|| {
    let mut gods = BTreeMap::new();

    gods
        .add(God::new(
            "agni",
            "Mage",
        ).with_abilities(vec![
            "Noxious Fumes",
            "Flame Wave",
            "Path of Flames",
            "Rain Fire",
        ]).with_roles(vec!["Mid"]))
        .add(God::new(
            "chaac",
            "Warrior",
        ).with_abilities(vec![
            "Thunder Strike",
            "Torrent",
            "Rain Dance",
            "Storm Call",
        ]).with_roles(vec!["Mid, Solo"]))
        .add(God::new(
            "cliodhna",
            "Assassin",
        ).with_abilities(vec![
            "Banshee's Wail",
            "Flickering Visions",
            "Lurching Claw",
            "Tearing the Veil",
        ]).with_roles(vec!["Mid, Jungle, Solo"]))
        .add(God::new(
            "eset",
            "Mage",
        ).with_abilities(vec![
            "Wing Gust",
            "Spirit Ball",
            "Dispel Magic",
            "Circle of Protection",
        ]).with_roles(vec!["Mid, Support"]))
        .add(God::new(
            "freya",
            "Mage",
        ).with_abilities(vec![
            "Irradiate",
            "Pulse",
            "Banish",
            "Valkyrie's Discretion",
        ]).with_roles(vec!["ADC"]))
        .add(God::new(
            "he_bo",
            "Mage",
        ).with_abilities(vec![
            "Water Cannon",
            "Atlas of the Yellow River",
            "Waterspout",
            "Crushing Wave",
        ]).with_roles(vec!["Mid, Jungle, Support"]))
        .add(God::new(
            "mercury",
            "Assassin",
        ).with_abilities(vec![
            "Made You Look",
            "Maximum Velocity",
            "Special Delivery",
            "Sonic Boom",
        ]).with_roles(vec!["Jungle"]))
        .add(God::new(
            "poseidon",
            "Mage",
        ).with_abilities(vec![
            "Tidal Surge",
            "Trident",
            "Whirlpool",
            "Release the Kraken",
        ]).with_roles(vec!["Mid, Jungle, ADC"]))
        .add(God::new(
            "thanatos",
            "Assassin",
        ).with_abilities(vec![
            "Death Scythe",
            "Scent of Death",
            "Soul Reap",
            "Hovering Death",
        ]).with_roles(vec!["Jungle"]));

    gods
}); 