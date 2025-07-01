use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemStats(pub HashMap<String, String>);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ItemEffect {
    Passive { description: String },
    Active { description: String, cooldown: u32 },
    Glyph { description: String, base_item: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub display_name: String,
    pub price: u32,
    #[serde(default)]
    pub stats: ItemStats,
    #[serde(default)]
    pub effects: Vec<ItemEffect>,
}

// Helper function to convert anything that can become a String
fn s<T: Into<String>>(value: T) -> String {
    value.into()
}

// Helper macro for creating stats
macro_rules! stats {
    () => {
        ItemStats::default()
    };
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut map = HashMap::new();
        $(map.insert(s($key), s($value));)*
        ItemStats(map)
    }};
}

// Helper macro for creating effects
macro_rules! effects {
    () => { vec![] };
    (passive: $desc:expr) => {
        vec![ItemEffect::Passive { description: s($desc) }]
    };
    (active: $desc:expr, $cd:expr) => {
        vec![ItemEffect::Active { description: s($desc), cooldown: $cd }]
    };
    (glyph: $desc:expr, $base:expr) => {
        vec![ItemEffect::Glyph { description: s($desc), base_item: s($base) }]
    };
    (passive: $pdesc:expr, glyph: $gdesc:expr, $base:expr) => {
        vec![
            ItemEffect::Passive { description: s($pdesc) },
            ItemEffect::Glyph { description: s($gdesc), base_item: s($base) }
        ]
    };
}

impl ItemStats {
    pub fn get_stat(&self, name: &str) -> Option<&str> {
        self.0.get(name).map(String::as_str)
    }
    
    pub fn get_stat_as_i32(&self, name: &str) -> Option<i32> {
        self.get_stat(name).and_then(|s| s.parse().ok())
    }
    
    pub fn get_stat_as_f32(&self, name: &str) -> Option<f32> {
        self.get_stat(name).and_then(|s| 
            s.trim_end_matches('%')
             .parse()
             .map(|n: f32| if s.ends_with('%') { n / 100.0 } else { n })
             .ok()
        )
    }
}

impl Item {
    pub fn new<T, U>(id: T, display_name: U, price: u32) -> Self 
    where 
        T: Into<String>,
        U: Into<String>,
    {
        let id = id.into();
        Item {
            name: id,
            display_name: display_name.into(),
            price,
            stats: stats!(),
            effects: vec![],
        }
    }

    pub fn with_stats(mut self, stats: ItemStats) -> Self {
        self.stats = stats;
        self
    }

    pub fn with_effects(mut self, effects: Vec<ItemEffect>) -> Self {
        self.effects = effects;
        self
    }

    pub fn get_passives(&self) -> Vec<&str> {
        self.effects.iter()
            .filter_map(|effect| match effect {
                ItemEffect::Passive { description } => Some(description.as_str()),
                _ => None
            })
            .collect()
    }
    
    pub fn get_active(&self) -> Option<(&str, u32)> {
        self.effects.iter()
            .find_map(|effect| match effect {
                ItemEffect::Active { description, cooldown } => 
                    Some((description.as_str(), *cooldown)),
                _ => None
            })
    }

    pub fn get_glyph(&self) -> Option<(&str, &str)> {
        self.effects.iter()
            .find_map(|effect| match effect {
                ItemEffect::Glyph { description, base_item } => 
                    Some((description.as_str(), base_item.as_str())),
                _ => None
            })
    }
}

// Helper trait for item insertion
trait ItemInserter {
    fn add(&mut self, item: Item) -> &mut Self;
}

impl ItemInserter for HashMap<String, Item> {
    fn add(&mut self, item: Item) -> &mut Self {
        self.insert(item.name.clone(), item);
        self
    }
}

pub static ITEMS: Lazy<HashMap<String, Item>> = Lazy::new(|| {
    let mut items = HashMap::new();

    // Relics
    items
        .add(Item::new("beads", "Purification Beads", 0)
            .with_effects(effects!(
                active: "Using this item removes Crowd Control Effects and makes you immune to new ones for 2s.", 
                170
            ))
        )
        .add(Item::new("blink", "Blink Rune", 0)
            .with_effects(effects!(
                active: "Using this item will allow you to teleport up to 45 units away instantly. This item can not be used if you have taken or dealt damage in the last 3s.",
                140
            ))
        )
        .add(Item::new("shell", "Shell", 0))
        .add(Item::new("ankh", "Ankh", 0))
        .add(Item::new("aegis", "Aegis", 0));

    // Basic Items
    items
        .add(Item::new("mace", "Mace", 650)
            .with_stats(stats!(
                "Physical Power" => "10"
            ))
        )
        .add(Item::new("magic-focus", "Magic Focus", 650)
            .with_stats(stats!(
                "Magical Power" => "25"
            ))
        );

    // Starter Items
    items
        .add(Item::new("s_bumbas", "Bumba's Dagger", 600)
            .with_stats(stats!(
                "Physical Power" => "7",
                "Magical Power" => "12",
                "Health" => "50",
                "Mana" => "50"
            ))
            .with_effects(effects!(
                passive: "Your Basic Attacks deal +25 True Damage and your Abilities deal +35% Damage versus Jungle Monsters. When a Jungle Monster is killed you are restored for 10% of the Monster's Health and 25 Mana."
            ))
        )
        .add(Item::new("s_bumbasspear", "Bumba's Spear", 2100)
            .with_stats(stats!(
                "Physical Power" => "70",
                "Magical Power" => "105",
                "Penetration" => "10%",
                "Cooldown Reduction" => "10%"
            ))
            .with_effects(effects!(
                passive: "Your Basic Attacks deal +50 True Damage and your Abilities deal +35% damage against Jungle Camps, Structures, and Jungle Bosses. When any of these die you gain a 10% power buff for 30s and are healed for 10% of their Health, 10% of your Mana."
            ))
        )
        .add(Item::new("s_manikin", "Manikin's Scepter", 750)
            .with_stats(stats!(
                "Basic Attack Damage" => "10",
                "Physical Protection" => "30"
            ))
            .with_effects(effects!(
                passive: "Enemies hit by your Basic Attacks are burned, taking 16 Physical Damage (+5% of your Physical and Magical Power) over 2s and have their Attack Speed reduced by 4.5%. Jungle Monsters restore 3% Health and 5% Mana when they die if they are burned. This effect can stack up to 3 times."
            ))
        )
        .add(Item::new("s_hidden", "Manikin's Hidden Blade", 1500)
            .with_stats(stats!(
                "Physical Power" => "60",
                "Magical Power" => "90",
                "Physical Protection" => "30",
                "Damage Reduction" => "5"
            ))
            .with_effects(effects!(
                passive: "If you have not taken or dealt damage in the last 5s and hit an enemy god, Jungle Monster, or Jungle Boss, they immediately take 20% of their Maximum Health as Physical Damage and are slowed by 20% for 5s."
            ))
        )
        .add(Item::new("s_sands", "Sands of Time", 700)
            .with_stats(stats!(
                "Magical Power" => "25",
                "MP5" => "10",
                "Cooldown Reduction" => "10%"
            ))
            .with_effects(effects!(
                passive: "This item grants 2 MP5 per 10% of your missing Mana. Your damaging abilities deal an extra 10 true damage to minions."
            ))
        )
        .add(Item::new("s_pendulum", "Pendulum of Ages", 1500)
            .with_stats(stats!(
                "Magical Power" => "90",
                "MP5" => "20",
                "Cooldown Reduction" => "20%"
            ))
            .with_effects(effects!(
                passive: "This item grants 4 MP5 per 10% of your missing Mana. This item grants 10 Magical power per 10% of your available Mana."
            ))
        );

    // Magical Items
    items
        .add(Item::new("bancrofts", "Bancroft's Talon", 2500)
            .with_stats(stats!(
                "Magical Power" => "100",
                "Magical Lifesteal" => "20%",
                "Mana" => "150"
            ))
            .with_effects(effects!(
                passive: "Gain up to 100 Magical Power and 20% Magical Lifesteal based on % missing Health"
            ))
        )
        .add(Item::new("soulreaver", "Soul Reaver", 2600)
            .with_stats(stats!(
                "Magical Power" => "95",
                "Mana" => "300"
            ))
            .with_effects(effects!(
                passive: "Your abilities deal an additional 2% of the target's maximum Health as Magical Damage. If the target has over 2000 Health, your ability bonus damage scales up. This effect reaches a maximum of 9% Maximum Health damage at 2750 Health. Subsequent hits on the same target do half the bonus damage for the next 3s."
            ))
        )
        .add(Item::new("tahuti", "Rod of Tahuti", 3000)
            .with_stats(stats!(
                "Magical Power" => "135",
                "Magical Penetration" => "10%",
                "MP5" => "30"
            ))
            .with_effects(effects!(
                passive: "Basic Attacks and Abilities gain 25% additional Magical Power against targets below 50% Health."
            ))
        )
        .add(Item::new("tahuti_calamitous", "Calamitous Rod of Tahuti", 3600)
            .with_stats(stats!(
                "Magical Power" => "135",
                "Magical Penetration" => "10%",
                "MP5" => "30"
            ))
            .with_effects(effects!(
                passive: "Basic Attacks and Abilities gain 25% additional Magical Power against targets below 50% Health.",
                glyph: "Successfully hitting an enemy god with an ability calls down a meteor that lands after 1s, dealing 100 (+35% of your Magical Power) damage in a 15 unit radius. This effect can only occur once every 30s.",
                "tahuti"
            ))
        )
        .add(Item::new("tahuti_perfected", "Perfected Rod of Tahuti", 3600)
            .with_stats(stats!(
                "Magical Power" => "135",
                "Magical Penetration" => "10%",
                "MP5" => "30"
            ))
            .with_effects(effects!(
                passive: "Basic Attacks and Abilities gain 25% additional Magical Power against targets below 50% Health.",
                glyph: "Successfully damaging an enemy god with an ability applies a mark for 3s. Damaging them a second time with another ability consumes the mark and provides you 10% movement speed for 2s and reduces your abilities cooldowns by 1.5s. Only one mark can be active at a time and cannot be applied for 8s after consuming a mark.",
                "tahuti"
            ))
        )
        .add(Item::new("obshard", "Obsidian Shard", 2450)
            .with_stats(stats!(
                "Magical Power" => "100",
                "Magical Penetration" => "20%"
            ))
            .with_effects(effects!(
                passive: "Your first ability cast gains 10% Magical Penetration. This can only occur once every 10 seconds."
            ))
        )
        .add(Item::new("spearofdeso", "Spear of Desolation", 2500)
            .with_stats(stats!(
                "Magical Power" => "110",
                "Magical Penetration" => "15",
                "Cooldown Reduction" => "10%"
            ))
            .with_effects(effects!(
                passive: "If you receive a kill or assist on an Enemy God all of you non-ultimate cooldowns are reduced by 2 seconds and your ultimate cooldown is reduced by 6s."
            ))
        )
        .add(Item::new("myrdin", "Staff of Myrddin", 2600)
            .with_stats(stats!(
                "Magical Power" => "110",
                "Magical Penetration" => "10%",
                "Cooldown Reduction" => "10%"
            ))
            .with_effects(effects!(
                passive: "When your ultimate ability has finished casting you gain Myrddins Rage which provides 25% Increased Damage Dealt, decaying to 2.5% Increased Damage Dealt over 8s. At the end of the 8s you lose Myrddins Rage. This can only occur once every 45s."
            ))
        );

    // Physical Items
    items
        .add(Item::new("arondight", "Arondight", 2600)
            .with_stats(stats!(
                "Physical Power" => "75",
                "Cooldown Reduction" => "10%"
            ))
            .with_effects(effects!(
                passive: "When your Ultimate ability has finished casting, reveal all enemy gods within 120 units for 8s. While moving towards revealed enemies gain 30% Movement Speed. When first striking a revealed target they take an additional 20 + 40% of your Physical Power. This can only occur once every 45 seconds."
            ))
        )
        .add(Item::new("trans", "Transcendence", 2600)
            .with_stats(stats!(
                "Physical Power" => "45",
                "Cooldown Reduction" => "10%",
                "Mana" => "1050",
                "MP5" => "10"
            ))
            .with_effects(effects!(
                passive: "3% of your Mana is converted to Physical Power."
            ))
        )
        .add(Item::new("heartseeker", "Heartseeker", 2750)
            .with_stats(stats!(
                "Physical Power" => "65",
                "Physical Penetration" => "10%",
                "Mana" => "200",
                "MP5" => "20"
            ))
            .with_effects(effects!(
                passive: "Your abilities deal an additional 3% of the targets maximum Health as Physical Damage. If you have over 200 Physical Power, your ability bonus damage scales up. This effect reaches a maximum of 6% Maximum Health damage at 350 Physical Power. Subsequent hits on the same target do 75% bonus damage for the next 3s."
            ))
        )
        .add(Item::new("dom", "Dominance", 2500)
            .with_stats(stats!(
                "Physical Power" => "55",
                "Physical Penetration" => "10%",
                "Mana" => "200",
                "MP5" => "20"
            ))
            .with_effects(effects!(
                passive: "Your Basic Attacks benefit from an additional 15% Physical Penetration."
            ))
        )
        .add(Item::new("hydras", "Hydra's Lament", 2300)
            .with_stats(stats!(
                "Physical Power" => "40",
                "Physical Penetration" => "10%",
                "Cooldown Reduction" => "10%",
                "MP5" => "10"
            ))
            .with_effects(effects!(
                passive: "For 8s after using an ability, your next Basic Attack will deal an additional 35% damage. Abilities that function like basic attacks do not benefit from this."
            ))
        )
        .add(Item::new("souleater2", "Soul Eater", 2100)
            .with_stats(stats!(
                "Physical Power" => "40",
                "Physical Lifesteal" => "15%",
                "Physical Penetration" => "10%",
                "Cooldown Reduction" => "10%"
            ))
            .with_effects(effects!(
                passive: "Your abilities heal you for 20% of the damage dealt to targets."
            ))
        )
        .add(Item::new("jotunns", "Jotunn's Wrath", 2200)
            .with_stats(stats!(
                "Physical Power" => "45",
                "Physical Penetration" => "10",
                "Cooldown Reduction" => "20%",
                "Mana" => "150"
            ))
        )
        .add(Item::new("jotunnsvigor", "Jotunn's Vigor", 2800)
            .with_stats(stats!(
                "Physical Power" => "45",
                "Physical Penetration" => "10",
                "Cooldown Reduction" => "20%",
                "Mana" => "150"
            ))
            .with_effects(effects!(
                glyph: "If you drop beneath 40% health, gain 10% Movement Speed and 30% Physical Ability Lifesteal for 5s. This effect may only occur once every 15s.",
                "jotunns"
            ))
        )
        .add(Item::new("titans", "Titan's Bane", 2550)
            .with_stats(stats!(
                "Physical Power" => "50",
                "Physical Penetration" => "20%"
            ))
            .with_effects(effects!(
                passive: "Your first ability cast gains 20% Physical Penetration. This can only occur once every 6 seconds."
            ))
        )
        .add(Item::new("crusher", "The Crusher", 2400)
            .with_stats(stats!(
                "Physical Power" => "45",
                "Attack Speed" => "10%",
                "Physical Penetration" => "15"
            ))
            .with_effects(effects!(
                passive: "Enemies hit by your damaging Abilities take an additional 20 Physical Damage + 20% of your Physical Power over 2s."
            ))
        );

    items
}); 