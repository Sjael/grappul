use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
        ]).with_roles(vec!["Jungle"]))
        // Adding more gods to expand the roster
        .add(God::new("achilles", "Warrior")
            .with_abilities(vec!["Shield of Achilles", "Radiant Glory", "Combat Dodge", "Fatal Strike"])
            .with_roles(vec!["Solo", "Jungle"]))
        .add(God::new("apollo", "Hunter")
            .with_abilities(vec!["So Beautiful", "Serenade", "The Moves", "Across The Sky"])
            .with_roles(vec!["ADC"]))
        .add(God::new("ares", "Guardian")
            .with_abilities(vec!["Shackles", "Bolster Defenses", "Searing Flesh", "No Escape"])
            .with_roles(vec!["Support"]))
        .add(God::new("artemis", "Hunter")
            .with_abilities(vec!["Transgressor's Fate", "Vengeful Assault", "Suppress The Insolent", "Calydonian Boar"])
            .with_roles(vec!["ADC"]))
        .add(God::new("athena", "Guardian")
            .with_abilities(vec!["Reach", "Confound", "Shield Wall", "Defender of Olympus"])
            .with_roles(vec!["Support", "Solo"]))
        .add(God::new("bacchus", "Guardian")
            .with_abilities(vec!["Chug", "Belly Flop", "Belch of the Gods", "Intoxicate"])
            .with_roles(vec!["Support", "Solo"]))
        .add(God::new("bellona", "Warrior")
            .with_abilities(vec!["Shield Bash", "Bludgeon", "Scourge", "Eagle's Rally"])
            .with_roles(vec!["Solo"]))
        .add(God::new("cerberus", "Guardian")
            .with_abilities(vec!["Paralyzing Spit", "Ghastly Breath", "Soul Expulsion", "Stygian Torment"])
            .with_roles(vec!["Support", "Solo"]))
        .add(God::new("chang_e", "Mage")
            .with_abilities(vec!["Crescent Moon Dance", "Moonlit Waltz", "Moonflower Dance", "Waxing Moon"])
            .with_roles(vec!["Mid", "Solo"]))
        .add(God::new("chronos", "Mage")
            .with_abilities(vec!["Time Rift", "Accelerate", "Stop Time", "Rewind"])
            .with_roles(vec!["ADC", "Mid"]))
        .add(God::new("cu_chulainn", "Warrior")
            .with_abilities(vec!["Barbed Spear", "Vent Anger", "Salmon's Leap", "Spear of Mortal Pain"])
            .with_roles(vec!["Solo"]))
        .add(God::new("cupid", "Hunter")
            .with_abilities(vec!["Lovestruck", "Share The Love", "Flutter", "Fields Of Love"])
            .with_roles(vec!["ADC"]))
        .add(God::new("danzaburou", "Hunter")
            .with_abilities(vec!["Fool's Gold", "Alluring Spirits", "Tanuki Trickery", "Uproarious Rocket"])
            .with_roles(vec!["ADC"]))
        .add(God::new("discordia", "Mage")
            .with_abilities(vec!["Unruly Magic", "Strife", "Erratic Behavior", "Golden Apple of Discord"])
            .with_roles(vec!["Mid"]))
        .add(God::new("fenrir", "Assassin")
            .with_abilities(vec!["Unchained", "Seething Howl", "Brutalize", "Ragnarok"])
            .with_roles(vec!["Jungle"]))
        .add(God::new("ganesha", "Guardian")
            .with_abilities(vec!["Turn of Fate", "Ohm", "Remove Obstacles", "Dharmic Pillars"])
            .with_roles(vec!["Support"]))
        .add(God::new("geb", "Guardian")
            .with_abilities(vec!["Roll Out", "Shock Wave", "Stone Shield", "Cataclysm"])
            .with_roles(vec!["Support"]))
        .add(God::new("hades", "Mage")
            .with_abilities(vec!["Death From Below", "Shroud of Darkness", "Devour Souls", "Pillar of Agony"])
            .with_roles(vec!["Mid", "Solo"]))
        .add(God::new("hel", "Mage")
            .with_abilities(vec!["Decay/Restoration", "Hinder/Cleanse", "Repulse/Inspire", "Switch Stances"])
            .with_roles(vec!["Mid", "Solo"]))
        .add(God::new("hercules", "Warrior")
            .with_abilities(vec!["Driving Strike", "Earthbreaker", "Mitigate Wounds", "Excavate"])
            .with_roles(vec!["Solo"]))
        .add(God::new("hou_yi", "Hunter")
            .with_abilities(vec!["Ricochet", "Mark of the Golden Crow", "Divebomb", "Sunbreaker"])
            .with_roles(vec!["ADC"]))
        .add(God::new("hun_batz", "Assassin")
            .with_abilities(vec!["Somersault", "Overhead Smash", "Sacred Monkey", "Fear No Evil"])
            .with_roles(vec!["Jungle"]))
        .add(God::new("izanami", "Hunter")
            .with_abilities(vec!["Sickle Storm", "Spectral Projection", "Fade Away", "Dark Portal"])
            .with_roles(vec!["ADC"]))
        .add(God::new("janus", "Mage")
            .with_abilities(vec!["Portal", "Unstable Vortex", "Threshold", "Through Space and Time"])
            .with_roles(vec!["Mid"]))
        .add(God::new("jing_wei", "Hunter")
            .with_abilities(vec!["Persistent Gust", "Explosive Bolts", "Agility", "Air Strike"])
            .with_roles(vec!["ADC"]))
        .add(God::new("kali", "Assassin")
            .with_abilities(vec!["Nimble Strike", "Lash", "Incense", "Destruction"])
            .with_roles(vec!["Jungle"]))
        .add(God::new("khepri", "Guardian")
            .with_abilities(vec!["Abduct", "Rising Dawn", "Solar Flare", "Scarab's Blessing"])
            .with_roles(vec!["Support"]))
        .add(God::new("king_arthur", "Warrior")
            .with_abilities(vec!["Overhead Slash", "Battle Stomp", "Twin Cleave", "Excalibur's Wrath"])
            .with_roles(vec!["Solo"]))
        .add(God::new("kukulkan", "Mage")
            .with_abilities(vec!["Zephyr", "Slipstream", "Whirlwind", "Spirit of the Nine Winds"])
            .with_roles(vec!["Mid"]))
        .add(God::new("kumbhakarna", "Guardian")
            .with_abilities(vec!["Throw Back", "Groggy Strike", "Mighty Yawn", "Epic Uppercut"])
            .with_roles(vec!["Support"]))
        .add(God::new("loki", "Assassin")
            .with_abilities(vec!["Vanish", "Behind You!", "Flurry Strike", "Assassinate"])
            .with_roles(vec!["Jungle"]))
        .add(God::new("medusa", "Hunter")
            .with_abilities(vec!["Viper Shot", "Acid Spray", "Lacerate", "Petrify"])
            .with_roles(vec!["ADC"]))
        .add(God::new("neith", "Hunter")
            .with_abilities(vec!["Spirit Arrow", "Unravel", "Back Flip", "World Weaver"])
            .with_roles(vec!["ADC"]))
        .add(God::new("nemesis", "Assassin")
            .with_abilities(vec!["Swift Vengeance", "Slice and Dice", "Retribution", "Divine Judgement"])
            .with_roles(vec!["Jungle"]))
        .add(God::new("nike", "Warrior")
            .with_abilities(vec!["Rend", "Plan of Action", "Valiant Leap", "Sentinel of Zeus"])
            .with_roles(vec!["Solo"]))
        .add(God::new("nu_wa", "Mage")
            .with_abilities(vec!["Mysterious Fog", "Clay Soldiers", "Shining Metal", "Fire Shards"])
            .with_roles(vec!["Mid"]))
        .add(God::new("odin", "Warrior")
            .with_abilities(vec!["Lunge", "Raven Shout", "Gungnir's Might", "Ring of Spears"])
            .with_roles(vec!["Solo", "Support"]))
        .add(God::new("olorun", "Mage")
            .with_abilities(vec!["Touch of Fate", "Overflowing Divinity", "Consecration", "Sanctified Field"])
            .with_roles(vec!["ADC", "Mid"]))
        .add(God::new("osiris", "Warrior")
            .with_abilities(vec!["Sickle Strike", "Spirit Flail", "Judgement Tether", "Lord of the Afterlife"])
            .with_roles(vec!["Solo"]))
        .add(God::new("pele", "Assassin")
            .with_abilities(vec!["Pyroclast", "Eruption", "Magma Rush", "Volcanic Lightning"])
            .with_roles(vec!["Jungle"]))
        .add(God::new("persephone", "Mage")
            .with_abilities(vec!["Bone Rush", "Harvest", "Flourish", "Grasp of Death"])
            .with_roles(vec!["Mid"]))
        .add(God::new("ra", "Mage")
            .with_abilities(vec!["Celestial Beam", "Divine Light", "Solar Blessing", "Searing Pain"])
            .with_roles(vec!["Mid"]))
        .add(God::new("rama", "Hunter")
            .with_abilities(vec!["Astral Strike", "Pick Me Up", "Rolling Assault", "Astral Barrage"])
            .with_roles(vec!["ADC"]))
        .add(God::new("ratatoskr", "Assassin")
            .with_abilities(vec!["Dart", "Flurry", "Acorn Blast", "Through the Cosmos"])
            .with_roles(vec!["Jungle"]))
        .add(God::new("ravana", "Assassin")
            .with_abilities(vec!["Chain of Blows", "Overhead Kick", "10-Hand Shadow Fist", "Mystic Rush"])
            .with_roles(vec!["Jungle"]))
        .add(God::new("scylla", "Mage")
            .with_abilities(vec!["Sic 'Em", "Crush", "Sentinel", "I'm a Monster"])
            .with_roles(vec!["Mid"]))
        .add(God::new("serqet", "Assassin")
            .with_abilities(vec!["Deathbane", "Cobra's Kiss", "Ambush", "Last Breath"])
            .with_roles(vec!["Jungle"]))
        .add(God::new("set", "Assassin")
            .with_abilities(vec!["Skewer", "Spawn of Set", "Sandstorm", "Kingslayer"])
            .with_roles(vec!["Jungle"]))
        .add(God::new("skadi", "Hunter")
            .with_abilities(vec!["Piercing Cold", "Rune of the Hunt", "Permafrost", "Winter's Grasp"])
            .with_roles(vec!["ADC"]))
        .add(God::new("sobek", "Guardian")
            .with_abilities(vec!["Charge Prey", "Tail Whip", "Sickening Strike", "Lurking In The Waters"])
            .with_roles(vec!["Support"]))
        .add(God::new("sol", "Mage")
            .with_abilities(vec!["Radiance", "Stellar Burst", "Disapparate", "Supernova"])
            .with_roles(vec!["ADC", "Mid"]))
        .add(God::new("sun_wukong", "Warrior")
            .with_abilities(vec!["The Magic Cudgel", "Master's Will", "72 Transformations", "Somersault Cloud"])
            .with_roles(vec!["Solo"]))
        .add(God::new("susano", "Assassin")
            .with_abilities(vec!["Storm Kata", "Wind Siphon", "Jet Stream", "Typhoon"])
            .with_roles(vec!["Jungle"]))
        .add(God::new("sylvanus", "Guardian")
            .with_abilities(vec!["Verdant Growth", "Wisps", "Nature's Protection", "Wrath of Terra"])
            .with_roles(vec!["Support"]))
        .add(God::new("terra", "Guardian")
            .with_abilities(vec!["Force of Nature", "Crushing Earth", "Monolith", "Terra's Blessing"])
            .with_roles(vec!["Support"]))
        .add(God::new("thoth", "Mage")
            .with_abilities(vec!["Hieroglyphic Assault", "Evade and Punish", "Glyph of Pain", "Final Judgement"])
            .with_roles(vec!["Mid"]))
        .add(God::new("tyr", "Warrior")
            .with_abilities(vec!["Fearless", "Power Cleave", "Change Stance", "Lawbringer"])
            .with_roles(vec!["Solo"]))
        .add(God::new("ullr", "Hunter")
            .with_abilities(vec!["Bladed Arrow", "Expose Weakness", "Hail of Arrows", "Wield Axes"])
            .with_roles(vec!["ADC"]))
        .add(God::new("vamana", "Warrior")
            .with_abilities(vec!["Clear The Path", "Armored Umbrella", "Umbrellarang", "Colossal Fury"])
            .with_roles(vec!["Solo"]))
        .add(God::new("vulcan", "Mage")
            .with_abilities(vec!["Backfire", "Inferno Cannon", "Magma Bomb", "Earthshaker"])
            .with_roles(vec!["Mid"]))
        .add(God::new("xbalanque", "Hunter")
            .with_abilities(vec!["Branching Bola", "Poison Darts", "Rising Jaguar", "Darkest of Nights"])
            .with_roles(vec!["ADC"]))
        .add(God::new("xing_tian", "Guardian")
            .with_abilities(vec!["Furious Roar", "Hook Slam", "Sky-Cutting Axe", "Whirlwind of Rage and Steel"])
            .with_roles(vec!["Support", "Solo"]))
        .add(God::new("yemoja", "Guardian")
            .with_abilities(vec!["Bouncing Bubble", "Mending Waters", "Riptide", "River's Rebuke"])
            .with_roles(vec!["Support"]))
        .add(God::new("ymir", "Guardian")
            .with_abilities(vec!["Ice Wall", "Glacial Strike", "Frost Breath", "Shards of Ice"])
            .with_roles(vec!["Support", "Solo"]))
        .add(God::new("zeus", "Mage")
            .with_abilities(vec!["Chain Lightning", "Aegis Assault", "Detonate Charge", "Lightning Storm"])
            .with_roles(vec!["Mid"]))
        .add(God::new("zhong_kui", "Mage")
            .with_abilities(vec!["Expose Evil", "Exorcism", "Book of Demons", "Recall Demons"])
            .with_roles(vec!["Mid", "Solo"]));

    gods
}); 