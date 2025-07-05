use std::collections::{BTreeMap, HashMap};
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ItemStat {
    // Offensive Stats
    PhysicalPower,
    MagicalPower,
    AttackSpeed,
    BasicAttackDamage,
    CriticalStrikeChance,
    
    // Penetration Stats
    PhysicalPenetration,
    MagicalPenetration,
    Penetration,
    
    // Defensive Stats
    Health,
    PhysicalProtection,
    MagicalProtection,
    DamageReduction,
    
    // Utility Stats
    Mana,
    MP5,
    CooldownReduction,
    MovementSpeed,
    
    // Lifesteal Stats
    PhysicalLifesteal,
    MagicalLifesteal,
}

impl ItemStat {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "Physical Power" => Some(Self::PhysicalPower),
            "Magical Power" => Some(Self::MagicalPower),
            "Attack Speed" => Some(Self::AttackSpeed),
            "Basic Attack Damage" => Some(Self::BasicAttackDamage),
            "Critical Strike Chance" => Some(Self::CriticalStrikeChance),
            "Physical Penetration" => Some(Self::PhysicalPenetration),
            "Magical Penetration" => Some(Self::MagicalPenetration),
            "Penetration" => Some(Self::Penetration),
            "Health" => Some(Self::Health),
            "Physical Protection" => Some(Self::PhysicalProtection),
            "Magical Protection" => Some(Self::MagicalProtection),
            "Damage Reduction" => Some(Self::DamageReduction),
            "Mana" => Some(Self::Mana),
            "MP5" => Some(Self::MP5),
            "Cooldown Reduction" => Some(Self::CooldownReduction),
            "Movement Speed" => Some(Self::MovementSpeed),
            "Physical Lifesteal" => Some(Self::PhysicalLifesteal),
            "Magical Lifesteal" => Some(Self::MagicalLifesteal),
            _ => None
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::PhysicalPower => "Physical Power",
            Self::MagicalPower => "Magical Power",
            Self::AttackSpeed => "Attack Speed",
            Self::BasicAttackDamage => "Basic Attack Damage",
            Self::CriticalStrikeChance => "Critical Strike Chance",
            Self::PhysicalPenetration => "Physical Penetration",
            Self::MagicalPenetration => "Magical Penetration",
            Self::Penetration => "Penetration",
            Self::Health => "Health",
            Self::PhysicalProtection => "Physical Protection",
            Self::MagicalProtection => "Magical Protection",
            Self::DamageReduction => "Damage Reduction",
            Self::Mana => "Mana",
            Self::MP5 => "MP5",
            Self::CooldownReduction => "Cooldown Reduction",
            Self::MovementSpeed => "Movement Speed",
            Self::PhysicalLifesteal => "Physical Lifesteal",
            Self::MagicalLifesteal => "Magical Lifesteal",
        }.to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemStats(#[serde(with = "item_stats_serde")] pub BTreeMap<ItemStat, String>);

mod item_stats_serde {
    use super::*;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(stats: &BTreeMap<ItemStat, String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let string_map: BTreeMap<String, String> = stats
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect();
        string_map.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<BTreeMap<ItemStat, String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string_map: BTreeMap<String, String> = BTreeMap::deserialize(deserializer)?;
        Ok(string_map
            .into_iter()
            .filter_map(|(k, v)| ItemStat::from_str(&k).map(|stat| (stat, v)))
            .collect())
    }
}

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
    ($($stat:expr => $value:expr),* $(,)?) => {{
        let mut map = BTreeMap::new();
        $(
            if let Some(stat) = ItemStat::from_str($stat) {
                map.insert(stat, s($value));
            }
        )*
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
    pub fn get_stat(&self, stat: ItemStat) -> Option<&str> {
        self.0.get(&stat).map(String::as_str)
    }
    
    pub fn get_stat_as_i32(&self, stat: ItemStat) -> Option<i32> {
        self.get_stat(stat).and_then(|s| s.parse().ok())
    }
    
    pub fn get_stat_as_f32(&self, stat: ItemStat) -> Option<f32> {
        self.get_stat(stat).and_then(|s| 
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
        .add(Item::new("spearmagus", "Spear of the Magus", 2400)
            .with_stats(stats!(
                "Magical Power" => "110",
                "Magical Penetration" => "15",
            ))
            .with_effects(effects!(
                passive: "After dealing damage to an enemy god, they recieve 8% more damage from all sources for 8s."
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

    // New Tier 3 items
    items
        .add(Item::new("chronos", "Chronos' Pendant", 2600)
            .with_stats(stats!(
                "Magical Power" => "100",
                "MP5" => "20",
                "Cooldown Reduction" => "20%"
            ))
            .with_effects(effects!(
                passive: "Every 10 seconds, your cooldowns are reduced by 1 second. This can only occur once every 10 seconds."
            ))
        )
        .add(Item::new("ethereal", "Ethereal Staff", 2600)
            .with_stats(stats!(
                "Magical Power" => "90",
                "Health" => "200",
                "MP5" => "20"
            ))
            .with_effects(effects!(
                passive: "Your abilities deal bonus damage equal to 5% of the target's maximum health as magical damage. This effect can only occur once every 10 seconds."
            ))
        )
        .add(Item::new("typhons", "Typhon's Fang", 2650)
            .with_stats(stats!(
                "Magical Power" => "100",
                "Magical Lifesteal" => "15%",
                "Magical Penetration" => "10%"
            ))
            .with_effects(effects!(
                passive: "Your Healing obtained from Magical Lifesteal is increased by 40%. Your Magical power is increased by twice the amount of Magical Lifesteal you have."
            ))
        )
        .add(Item::new("doom_orb", "Doom Orb", 2700)
            .with_stats(stats!(
                "Magical Power" => "145",
                "MP5" => "25",
                "Movement Speed" => "6%"
            ))
            .with_effects(effects!(
                passive: "Getting a kill or assist on an enemy god or scoring a kill on an objective grants you a stack that provides 3% Movement Speed. This effect can stack up to 2 times and lasts 15s."
            ))
        )
        .add(Item::new("bloodforge", "Bloodforge", 2600)
            .with_stats(stats!(
                "Physical Power" => "75",
                "Physical Lifesteal" => "15%"
            ))
            .with_effects(effects!(
                passive: "Getting a kill or assist on an enemy god forges a shield from their blood with health equal to 200 + 10 per player level for 20s. While the shield holds you gain +10% movement speed."
            ))
        )
        .add(Item::new("qins", "Qin's Sais", 2600)
            .with_stats(stats!(
                "Physical Power" => "40",
                "Attack Speed" => "20%"
            ))
            .with_effects(effects!(
                passive: "On Basic Attack hits, deal Physical Damage equal to 3% of the target's maximum Health. If the target has over 2000 Health, the bonus damage scales up. This effect reaches a maximum of 5% of the target's maximum Health at 2750 Health."
            ))
        )
        .add(Item::new("failnot", "Fail-not", 2650)
            .with_stats(stats!(
                "Physical Power" => "40",
                "Critical Strike Chance" => "20%",
                "Cooldown Reduction" => "20%"
            ))
            .with_effects(effects!(
                passive: "When your Ultimate ability has finished casting, your next ability or basic attack within 8s that damages an enemy god marks them, increasing Critical Strike Chance against them by 20% for 10s. This can only occur once every 45 seconds."
            ))
        )
        .add(Item::new("deathbringer", "Deathbringer", 2900)
            .with_stats(stats!(
                "Physical Power" => "50",
                "Critical Strike Chance" => "30%"
            ))
            .with_effects(effects!(
                passive: "Critical Strike bonus damage increased by 30%."
            ))
        )
        .add(Item::new("rage", "Rage", 2500)
            .with_stats(stats!(
                "Physical Power" => "30",
                "Critical Strike Chance" => "30%"
            ))
            .with_effects(effects!(
                passive: "Killing or getting an assist on an enemy god gives you 1 stack. Each stack provides 1% Critical Strike Chance. At max stacks of 5, gain 10% Critical Strike Chance. Stacks are permanent and are only lost on death."
            ))
        )
        .add(Item::new("winddemon", "Wind Demon", 2550)
            .with_stats(stats!(
                "Physical Power" => "25",
                "Attack Speed" => "20%",
                "Critical Strike Chance" => "20%"
            ))
            .with_effects(effects!(
                passive: "Successfully hitting an enemy with a Critical Strike grants you 10% Penetration, 10% Movement Speed, and reduces your enemy's healing by 40% for 5s."
            ))
        )
        .add(Item::new("rage2", "Evolved Rage", 0)
            .with_stats(stats!(
                "Physical Power" => "55",
                "Critical Strike Chance" => "45%"
            ))
        );

    items
        .add(Item::new("bookofthoth", "Book of Thoth", 2500)
            .with_stats(stats!(
                "Magical Power" => "80",
                "Mana" => "1000",
                "MP5" => "20"
            ))
            .with_effects(effects!(
                passive: "5% of your Mana is converted to Magical Power. Abilities deal bonus damage to minions equal to 3% of your maximum Mana."
            ))
        )
        .add(Item::new("bookofthoth2", "Evolved Book of Thoth", 0)
            .with_stats(stats!(
                "Magical Power" => "80",
                "Mana" => "1000",
                "MP5" => "20"
            ))
            .with_effects(effects!(
                passive: "7% of your Mana is converted to Magical Power. Abilities deal bonus damage to minions equal to 3% of your maximum Mana."
            ))
        )
        .add(Item::new("asi", "Asi", 2400)
            .with_stats(stats!(
                "Physical Power" => "30",
                "Attack Speed" => "25%",
                "Physical Penetration" => "15",
                "Physical Lifesteal" => "15%"
            ))
            .with_effects(effects!(
                passive: "If you drop below 35% Health, you gain an additional 25% Physical Lifesteal for 5 seconds. Can only occur once every 15 seconds."
            ))
        )
        .add(Item::new("spiritrobe", "Spirit Robe", 2400)
            .with_stats(stats!(
                "Physical Protection" => "40",
                "Magical Protection" => "40",
                "Cooldown Reduction" => "10%",
                "Damage Reduction" => "10%"
            ))
            .with_effects(effects!(
                passive: "After being hit by a Hard Crowd Control Effect you gain 15% Damage Mitigation for 3s. This can only occur once every 15 seconds."
            ))
        )
        .add(Item::new("thebes", "Gauntlet of Thebes", 2350)
            .with_stats(stats!(
                "Health" => "300",
                "Physical Protection" => "30",
                "Magical Protection" => "30"
            ))
            .with_effects(effects!(
                passive: "Every minion or jungle monster that dies near you grants 1 stack, and every god kill or assist grants 5 stacks. At 50 stacks this item evolves, gaining 10 more Magical and Physical Protection."
            ))
        )
        .add(Item::new("thebes2", "Evolved Gauntlet of Thebes", 0)
            .with_stats(stats!(
                "Health" => "300",
                "Physical Protection" => "40",
                "Magical Protection" => "40"
            ))
            .with_effects(effects!(
                passive: "Aura - Allied gods within 70 units gain 10 Physical Protection and 10 Magical Protection."
            ))
        )
        .add(Item::new("stoneofbinding", "Stone of Binding", 2300)
            .with_stats(stats!(
                "Physical Protection" => "35",
                "Magical Protection" => "35",
                "MP5" => "20"
            ))
            .with_effects(effects!(
                passive: "When you hit an enemy god with Crowd Control they have their Magical and Physical Protections reduced by 10% for 5s."
            ))
        )
        .add(Item::new("fatalis", "Hastened Fatalis", 2600)
            .with_stats(stats!(
                "Attack Speed" => "30%",
                "Movement Speed" => "7%",
                "Physical Penetration" => "10"
            ))
            .with_effects(effects!(
                passive: "Successfully hitting an enemy with a Basic Attack grants Haste for 1s, allowing you to move without movement penalty. Additional Basic Attack hits refresh the duration."
            ))
        )
        .add(Item::new("serrated", "Serrated Edge", 2500)
            .with_stats(stats!(
                "Physical Power" => "40",
                "Physical Lifesteal" => "15%",
                "Movement Speed" => "7%"
            ))
            .with_effects(effects!(
                passive: "For each non-ultimate ability on cooldown you gain 10 Physical Power and 7% Physical Penetration."
            ))
        );

    items
        .add(Item::new("s_warriors", "Warrior's Axe", 650)
            .with_stats(stats!(
                "Health" => "100",
                "Physical Protection" => "15",
                "MP5" => "10"
            ))
            .with_effects(effects!(
                passive: "Every time you are damaged by an enemy god you gain a stack that provides 2 Physical Protection and 2 Magical Protection. Stacks up to 3 times, stacks last 6s."
            ))
        )
        .add(Item::new("s_bluestone", "Bluestone Pendant", 700)
            .with_stats(stats!(
                "Physical Power" => "15",
                "MP5" => "10"
            ))
            .with_effects(effects!(
                passive: "Enemies hit by your damaging abilities take an additional 25 Physical Damage over 2s."
            ))
        )
        .add(Item::new("s_protectors", "Protector's Mask", 600)
            .with_stats(stats!(
                "Health" => "75",
                "Physical Protection" => "10",
                "Magical Protection" => "10",
                "MP5" => "10"
            ))
        )
        .add(Item::new("s_eye", "Eye of the Jungle", 650)
            .with_stats(stats!(
                "Physical Power" => "15",
                "MP5" => "10"
            ))
            .with_effects(effects!(
                passive: "Dealing damage to Jungle Monsters marks them for 10s. Marked monsters take 15% increased damage from you and restore 10% of your missing Health and Mana when killed."
            ))
        )
        .add(Item::new("magic-focus", "Tiny Trinket", 550)
            .with_stats(stats!(
                "Magical Power" => "20",
                "Magical Lifesteal" => "6%"
            ))
        )
        .add(Item::new("mace", "Mace", 650)
            .with_stats(stats!(
                "Physical Power" => "15",
                "Physical Penetration" => "10"
            ))
        )
        .add(Item::new("spellbook", "Spell Focus", 600)
            .with_stats(stats!(
                "Magical Power" => "25",
                "MP5" => "5"
            ))
        )
        .add(Item::new("spiked", "Spiked Gauntlet", 600)
            .with_stats(stats!(
                "Physical Power" => "15",
                "Physical Lifesteal" => "8%"
            ))
        )
        .add(Item::new("druidstone", "Magic Focus", 550)
            .with_stats(stats!(
                "Magical Power" => "25"
            ))
        )
        .add(Item::new("s_bluestonebrooch", "Bluestone Brooch", 2500)
            .with_stats(stats!(
                "Physical Power" => "65",
                "MP5" => "20",
                "Physical Protection" => "35"
            ))
            .with_effects(effects!(
                glyph: "Enemies hit by your damaging abilities take an additional 7.5% of their Maximum Health as Physical Damage over 2s. This effect can only occur once every 10s.", "s_bluestone"
            ))
        )
        .add(Item::new("tahuti_calamitous", "Rod of Tahuti - Calamitous Power", 2700)
            .with_stats(stats!(
                "Magical Power" => "140",
                "MP5" => "30",
                "Movement Speed" => "6%"
            ))
            .with_effects(effects!(
                passive: "Magical power scaling of your abilities is increased by 25% against targets below 50% health.",
                glyph: "Your abilities gain 15% increased scaling. Additionally, your first ability cast from out of combat deals 20% increased damage.", "tahuti"
            ))
        )
        .add(Item::new("jotunnsvigor", "Jotunn's Vigor", 2600)
            .with_stats(stats!(
                "Physical Power" => "45",
                "MP5" => "10",
                "Physical Penetration" => "10",
                "Cooldown Reduction" => "20%"
            ))
            .with_effects(effects!(
                glyph: "When you cast an Ultimate ability, your next non-ultimate ability cast within the next 10 seconds has its cooldown reduced by 3 seconds.", "jotunns"
            ))
        );

    items
        .add(Item::new("db_mal", "Malicious Deathbringer", 3500)
            .with_stats(stats!(
                "Physical Power" => "30",
                "Critical Strike Chance" => "30%"
            ))
            .with_effects(effects!(
                passive: "Critical Strike bonus damage dealt is increased by 75%",
                glyph: "Successfully hitting an Enemy God with a Critical Strike will subtract 1s from all of your abilities currently on cooldown.", "deathbringer"
            ))
        )
        .add(Item::new("db_envenom", "Envenomed Deathbringer", 3500)
            .with_stats(stats!(
                "Physical Power" => "30",
                "Critical Strike Chance" => "30%"
            ))
            .with_effects(effects!(
                passive: "Critical Strike bonus damage dealt is increased by 25%",
                glyph: "Successfully hitting an Enemy God with a Critical Strike will afflict them with poison for 3s. This poison reduces their healing by 40% and Shields applied on them are reduced by 50%.", "deathbringer"
            ))
        );

    items
        .add(Item::new("pridwen", "Pridwen", 2500)
            .with_stats(stats!(
                "Physical Protection" => "30",
                "Magical Protection" => "30",
                "Cooldown Reduction" => "20%"
            ))
            .with_effects(effects!(
                passive: "When your Ultimate ability has finished casting, you gain a Shield equal to your Protections for 5s. When this Shield is destroyed or expires, it deals 50% of the Shield's initial health as Magical damage to nearby enemies and slows them by 25% for 3s. This effect can only occur once every 45s."
            ))
        )
        .add(Item::new("s_lonos", "Lono's Mask", 600)
            .with_stats(stats!(
                "Health" => "75",
                "Physical Protection" => "10",
                "Magical Protection" => "10",
                "MP5" => "7"
            ))
            .with_effects(effects!(
                passive: "You deal 15% less damage to Enemy Gods and Structures, but take 15% less damage from Enemy Gods."
            ))
        );

    items
        .add(Item::new("s_conduit", "Conduit Gem", 700)
            .with_stats(stats!(
                "Magical Power" => "25",
                "MP5" => "10",
                "Magical Penetration" => "5"
            ))
            .with_effects(effects!(
                passive: "Your abilities deal bonus damage equal to 15% of your Magical Power. This effect can only occur once every 5s."
            ))
        )
        .add(Item::new("demonic", "Demonic Grip", 2300)
            .with_stats(stats!(
                "Magical Power" => "75",
                "Attack Speed" => "30%",
                "Magical Penetration" => "10%"
            ))
            .with_effects(effects!(
                passive: "Successfully hitting an enemy with a Basic Attack reduces their Magical Protection by 7% for 3s (max 3 Stacks)."
            ))
        )
        .add(Item::new("bancroftsclaw", "Bancroft's Claw", 3000)
            .with_stats(stats!(
                "Magical Power" => "100",
                "Magical Lifesteal" => "20%",
                "Mana" => "150"
            ))
            .with_effects(effects!(
                passive: "Gain up to 100 Magical Power and 20% Magical Lifesteal based on % missing Health",
                glyph: "When you drop below 25% Health, gain 20% Movement Speed and your abilities deal 20% increased damage for 8s. This can only occur once every 20s.", 
                "bancrofts"
            ))
        );

    items
}); 