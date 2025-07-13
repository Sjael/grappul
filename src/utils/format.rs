use crate::data::items::ItemStat;

/// Convert god name from code format to image filename format
/// Since we now use slugified names consistently, this just returns the name as-is
pub fn format_god_image_name(god_name: &str) -> String {
    god_name.to_string()
}

/// Format an ItemStat enum variant into a human-readable string
pub fn format_stat_name(stat: &ItemStat) -> &'static str {
    match stat {
        ItemStat::PhysicalPower => "Physical Power",
        ItemStat::MagicalPower => "Magical Power",
        ItemStat::PhysicalProtection => "Physical Protection",
        ItemStat::MagicalProtection => "Magical Protection",
        ItemStat::Health => "Health",
        ItemStat::Mana => "Mana",
        ItemStat::HP5 => "HP5",
        ItemStat::MP5 => "MP5",
        ItemStat::AttackSpeed => "Attack Speed",
        ItemStat::PhysicalLifesteal => "Physical Lifesteal",
        ItemStat::MagicalLifesteal => "Magical Lifesteal",
        ItemStat::PhysicalPenetration => "Physical Penetration",
        ItemStat::MagicalPenetration => "Magical Penetration",
        ItemStat::PhysicalPenetrationPercent => "Physical Penetration",
        ItemStat::MagicalPenetrationPercent => "Magical Penetration",
        ItemStat::CriticalStrikeChance => "Critical Strike Chance",
        ItemStat::CooldownReduction => "Cooldown Reduction",
        ItemStat::MovementSpeed => "Movement Speed",
        ItemStat::BasicAttackDamage => "Basic Attack Damage",
        ItemStat::DamageReduction => "Damage Reduction",
    }
}