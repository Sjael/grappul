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
}

// Helper trait for guide insertion
trait GuideInserter {
    fn add(&mut self, guide: Guide) -> &mut Self;
}

impl GuideInserter for Vec<Guide> {
    fn add(&mut self, guide: Guide) -> &mut Self {
        self.push(guide);
        self
    }
}

pub static GUIDES: Lazy<HashMap<String, Vec<Guide>>> = Lazy::new(|| {
    let mut guides = HashMap::new();
    
    // Agni Guides
    let agni_guides = vec![
        Guide::new("agni", "mid")
            .with_build(vec![
                "s_pendulum",
                "spearmagus",
                "spearofdeso",
                "myrdin",
                "tahuti_calamitous",
                "obshard"
            ])
            .with_relics(vec!["beads", "aegis"])
            .with_timeline(vec![
                TimelineEntry {
                    percent: 0,
                    items: Some(vec!["s_sands".to_string(), "magic-focus".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 15,
                    items: Some(vec!["spearmagus".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 30,
                    items: Some(vec!["spearofdeso".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 45,
                    items: Some(vec!["myrdin".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 60,
                    items: Some(vec!["tahuti".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 75,
                    items: Some(vec!["s_pendulum".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 90,
                    items: Some(vec!["tahuti_calamitous".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 100,
                    items: Some(vec!["obshard".to_string()]),
                    tip: None,
                },
            ])
    ];
    guides.insert("agni".to_string(), agni_guides);

    // Chaac Guides
    let chaac_guides = vec![
        Guide::new("chaac", "mid")
            .with_build(vec![
                "crusher",
                "jotunnsvigor",
                "trans",
                "souleater2",
                "heartseeker",
                "titans"
            ])
            .with_relics(vec!["blink", "beads"])
            .with_timeline(vec![
                TimelineEntry {
                    percent: 0,
                    items: Some(vec!["s_warriors".to_string(), "mace".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 15,
                    items: Some(vec!["jotunns".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 30,
                    items: Some(vec!["trans".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 45,
                    items: Some(vec!["souleater".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 60,
                    items: Some(vec!["jotunnsvigor".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 75,
                    items: Some(vec!["heartseeker".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 90,
                    items: Some(vec!["titans".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 100,
                    items: Some(vec!["crusher".to_string()]),
                    tip: None,
                },
            ])
    ];
    guides.insert("chaac".to_string(), chaac_guides);

    // Cliodhna Guides
    let cliodhna_guides = vec![
        Guide::new("cliodhna", "mid")
            .with_build(vec![
                "s_bluestonebrooch",
                "souleater2",
                "crusher",
                "jotunnsvigor",
                "arondight",
                "titans"
            ])
            .with_relics(vec!["blink", "beads"])
            .with_timeline(vec![
                TimelineEntry {
                    percent: 0,
                    items: Some(vec!["s_bluestone".to_string(), "spiked".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 15,
                    items: Some(vec!["souleater".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 30,
                    items: Some(vec!["crusher".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 45,
                    items: Some(vec!["jotunns".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 60,
                    items: Some(vec!["jotunnsvigor".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 75,
                    items: Some(vec!["arondight".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 90,
                    items: Some(vec!["s_bluestonebrooch".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 100,
                    items: Some(vec!["titans".to_string()]),
                    tip: None,
                },
            ])
    ];
    guides.insert("cliodhna".to_string(), cliodhna_guides);

    // Eset Guides
    let eset_guides = vec![
        Guide::new("eset", "support")
            .with_build(vec![
                "s_lonos",
                "stoneofbinding",
                "thebes2",
                "pridwen",
                "spiritrobe",
                "soulreaver"
            ])
            .with_relics(vec!["shell", "ankh"])
            .with_timeline(vec![
                TimelineEntry {
                    percent: 0,
                    items: Some(vec!["s_protectors".to_string(), "druidstone".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 15,
                    items: Some(vec!["stoneofbinding".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 30,
                    items: Some(vec!["thebes".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 45,
                    items: Some(vec!["pridwen".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 60,
                    items: Some(vec!["spiritrobe".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 75,
                    items: Some(vec!["s_lonos".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 90,
                    items: Some(vec!["soulreaver".to_string()]),
                    tip: None,
                },
            ])
    ];
    guides.insert("eset".to_string(), eset_guides);

    // He Bo Guides
    let he_bo_guides = vec![
        Guide::new("he_bo", "jungle")
            .with_build(vec![
                "s_bumbasspear",
                "spearmagus",
                "tahuti_perfected",
                "spearofdeso",
                "obshard",
                "soulreaver"
            ])
            .with_relics(vec!["blink", "beads"])
            .with_timeline(vec![
                TimelineEntry {
                    percent: 0,
                    items: Some(vec!["s_bumbas".to_string(), "magic-focus".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 15,
                    items: Some(vec!["spearmagus".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 30,
                    items: Some(vec!["tahuti".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 45,
                    items: Some(vec!["tahuti_perfected".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 60,
                    items: Some(vec!["spearofdeso".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 75,
                    items: Some(vec!["obshard".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 90,
                    items: Some(vec!["s_bumbasspear".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 100,
                    items: Some(vec!["soulreaver".to_string()]),
                    tip: None,
                },
            ])
    ];
    guides.insert("he_bo".to_string(), he_bo_guides);

    // Poseidon Guides
    let poseidon_guides = vec![
        Guide::new("poseidon", "mid")
            .with_build(vec![
                "s_pendulum",
                "bookofthoth2",
                "spearmagus",
                "myrdin",
                "spearofdeso",
                "obshard"
            ])
            .with_relics(vec!["beads", "aegis"])
            .with_timeline(vec![
                TimelineEntry {
                    percent: 0,
                    items: Some(vec!["s_sands".to_string(), "spellbook".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 15,
                    items: Some(vec!["bookofthoth".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 30,
                    items: Some(vec!["spearmagus".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 45,
                    items: Some(vec!["myrdin".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 60,
                    items: Some(vec!["spearofdeso".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 75,
                    items: Some(vec!["s_pendulum".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 90,
                    items: Some(vec!["obshard".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 100,
                    items: Some(vec!["powerpot".to_string()]),
                    tip: None,
                },
            ])
    ];
    guides.insert("poseidon".to_string(), poseidon_guides);

    // Mercury Guides
    let mercury_guides = vec![
        Guide::new("mercury", "jungle")
            .with_build(vec![
                "dom",
                "rage2",
                "winddemon",
                "db_envenom",
                "asi",
                "serrated"
            ])
            .with_relics(vec!["blink", "beads"])
            .with_timeline(vec![
                TimelineEntry {
                    percent: 0,
                    items: Some(vec!["s_eye".to_string(), "hiddendag".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 15,
                    items: Some(vec!["rage".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 30,
                    items: Some(vec!["winddemon".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 45,
                    items: Some(vec!["deathbringer".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 60,
                    items: Some(vec!["asi".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 75,
                    items: Some(vec!["serrated".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 90,
                    items: Some(vec!["dom".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 100,
                    items: Some(vec!["db_envenom".to_string()]),
                    tip: None,
                },
            ])
    ];
    guides.insert("mercury".to_string(), mercury_guides);

    // Thanatos Guides
    let thanatos_guides = vec![
        Guide::new("thanatos", "jungle")
            .with_build(vec![
                "s_hidden",
                "jotunns",
                "hydras",
                "arondight",
                "titans",
                "heartseeker"
            ])
            .with_relics(vec!["blink", "beads"])
            .with_timeline(vec![
                TimelineEntry {
                    percent: 0,
                    items: Some(vec!["s_manikin".to_string(), "mace".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 15,
                    items: Some(vec!["jotunns".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 30,
                    items: Some(vec!["hydras".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 45,
                    items: Some(vec!["arondight".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 60,
                    items: Some(vec!["titans".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 75,
                    items: Some(vec!["s_hidden".to_string()]),
                    tip: None,
                },
                TimelineEntry {
                    percent: 90,
                    items: Some(vec!["dom".to_string()]),
                    tip: None,
                },
            ])
    ];
    guides.insert("thanatos".to_string(), thanatos_guides);

    guides
}); 