use std::collections::{HashMap, BTreeMap};
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ItemStat {
    BasicAttackDamage,
    PhysicalPower,
    MagicalPower,
    Health,
    PhysicalProtection,
    MagicalProtection,
    Mana,
    HP5,
    MP5,
    AttackSpeed,
    PhysicalLifesteal,
    MagicalLifesteal,
    PhysicalPenetration,
    MagicalPenetration,
    PhysicalPenetrationPercent,
    MagicalPenetrationPercent,
    CriticalStrikeChance,
    CooldownReduction,
    MovementSpeed,
    DamageReduction,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Effect {
    Passive,
    Active,
    Glyph,
    Aura,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ItemTag {
    Tier1,
    Tier2,
    Tier3,
    Tier4,
    Consumable,
    Evolved,
    Glyph,
    Starter,
    Relic,
    Shard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub display_name: String,
    pub price: u32,
    #[serde(default)]
    pub stats: HashMap<ItemStat, i32>,
    #[serde(default)]
    pub effects: BTreeMap<Effect, String>,
    #[serde(default)]
    pub tags: Vec<ItemTag>,
}

// Load items from JSON file
pub static ITEMS: Lazy<HashMap<String, Item>> = Lazy::new(|| {
    let items_json = include_str!("json/items.json");
    serde_json::from_str(items_json).expect("Failed to parse items.json")
});