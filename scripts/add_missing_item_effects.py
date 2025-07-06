#!/usr/bin/env python3
"""
Add missing item effects (passives) for non-relic items
"""

import json
from pathlib import Path
from collections import OrderedDict


# Known item passives from the wiki
ITEM_PASSIVES = {
    "bancrofts": ["PASSIVE - You gain additional Magical Power and Lifesteal scaled from missing Health. This caps at 70 power and 35% Lifesteal at 40% Health."],
    "bancroftsclaw": ["PASSIVE - You gain additional Magical Power and Lifesteal scaled from missing Health. This caps at 100 power and 55% Lifesteal at 40% Health."],
    "soulreaver": ["PASSIVE - Your abilities deal an additional 2% of the target's maximum Health as Magical Damage. If the target has over 2000 Health, your ability bonus damage scales up linearly to reach 10% damage at 5000 Health. Subsequent hits on the same target do 70% of the bonus damage for the next 3s."],
    "tahuti": ["PASSIVE - Basic Attacks and Abilities deal 25% more damage to targets at or below 50% Health. This effect increases to 35% more damage against non-god targets."],
    "bloodforge": ["PASSIVE - Killing an enemy god grants you a shield equal to 200 +20 per level for 20s. While the Blood Shield is active you gain +10% movement speed."],
    "qins": ["PASSIVE - On Basic Attack hits, deal Physical Damage equal to 4% of the target's maximum Health. If the target has over 2000 Health, the bonus damage scales up linearly to reach 8% at 2750 Health."],
    "deathbringer": ["PASSIVE - Critical Strike damage is increased by 25%."],
    "failnot": ["PASSIVE - On Critical Strike all enemies within 15 units of the target are Rooted for 1s. This can only happen once every 5s."],
    "rage": ["PASSIVE - Gain 1 stack per Basic Attack hit, 2 stacks for Critical Strike hits. At 5 stacks your next Basic Attack is a Critical Strike. All stacks are lost on a successful Critical Strike."],
    "spiritrobe": ["PASSIVE - You gain 40% Damage Mitigation for 3s when you are hit with a Hard Crowd Control Effect. This can only occur once every 15 seconds."],
    "pridwen": ["PASSIVE - When your Ultimate ability has finished casting, you gain a Shield equal to 75% of your Protections for 5s. When destroyed, by timing out or being depleted, it explodes and deals Magical damage equal to 50% of the Shield's initial Health to enemies within 30 units. This can only occur once every 45 seconds."],
    "thebes": ["PASSIVE - Provide 10 Physical and Magical Protection to allies within 70 units. This effect is doubled to 20 of each Protection if you are near an allied structure or an ally is below 50% Health."],
    "thebes2": ["PASSIVE - Provide 15 Physical and Magical Protection to allies within 70 units. This effect is doubled to 30 of each Protection if you are near an allied structure or an ally is below 50% Health."],
    "stoneofbinding": ["PASSIVE - Successfully hitting an enemy with a Crowd Control ability will place a debuff on the enemy, reducing their Physical and Magical Protections by 15 for 5s."],
    "hydras": ["PASSIVE - For 8s after using an ability, your next Basic Attack will deal an additional 30% damage. Abilities that function like Basic Attacks do not benefit from this."],
    "jotunns": ["PASSIVE - When you kill an enemy god your non-ultimate cooldowns are reduced by 2s and you gain 20% movement speed for 5s."],
    "titans": ["PASSIVE - Your Physical Abilities gain 10% Physical Penetration."],
    "chronos": ["PASSIVE - Every 10s the Pendant activates, subtracting 1s from all of your abilities currently on Cooldown. The initial countdown will not start until you leave the fountain."],
    "ethereal": ["PASSIVE - Abilities steal 3% + 6 Maximum Mana converted to Magical Power from each enemy hit. The Mana is returned to the enemies after 5s."],
    "typhons": ["PASSIVE - Your Healing gained from Magical Lifesteal is increased by 50%. Your Magical power is increased by 2 times the amount of Magical Lifesteal you have."],
    "doom_orb": ["PASSIVE - Killing or assisting a minion provides you with 1 stack, granting 1% Movement Speed and 3 Magical Power per stack. Stacks last for 15s. Stack count is halved on death. Killing a god grants max stacks."],
    "winddemon": ["PASSIVE - Your Critical Hits provide you with 15% Attack Speed and 10% Movement Speed for 5s."],
    "asi": ["PASSIVE - While below 50% Health you gain 20% Physical Lifesteal and 30% Attack Speed for 5 seconds. This effect can only occur once every 15 seconds."],
    "demonic": ["PASSIVE - Your Basic Attacks reduce a target's Magical Protections by 10 + 1 per level, stacking up to 3 times. Lasts 5s."],
    "fatalis": ["PASSIVE - When a Basic Attack hits an enemy, the attacking movement speed debuff is removed from you for 1s."],
    "arondight": ["PASSIVE - When your Ultimate ability has finished casting, reveal all enemy gods within 120 units for 8s. While moving towards revealed enemies gain 30% Movement Speed. When a revealed enemy is killed by you or dies within 8s of being damaged by you, gain a success. Each success restores 10% of your maximum health and mana."],
    "trans": ["PASSIVE - You gain +30 Maximum Mana per god level. You gain 4 Physical Power for every 100 Maximum Mana you own."],
    "heartseeker": ["PASSIVE - Your abilities deal an additional 3% of the targets maximum Health as Physical Damage. If you have over 300 Mana, the bonus damage scales up. This effect reaches a maximum of 7% Maximum Health damage at 1000 Mana. Subsequent hits on the same target do 70% of the bonus damage for the next 3s."],
    "souleater2": ["PASSIVE - Your damaging abilities heal you for 10% of the damage dealt. Any healing over 100% Health is granted as a shield that lasts 5s."],
    "crusher": ["PASSIVE - After using an ability your next Basic Attack deals bonus damage equal to 30% of your Physical Power over 2s."],
    "bookofthoth": ["PASSIVE - You gain +20 Mana per god level. You gain 7 Magical Power for every 100 Mana you have."],
    "bookofthoth2": ["PASSIVE - You gain +20 Mana per god level. You gain 9 Magical Power for every 100 Mana you have."],
    "myrdin": ["PASSIVE - When you damage an enemy god below 50% health with a non-Ultimate ability you immediately gain 1 stack of Arcane Energy and your non-Ultimate cooldowns are reduced by 1s. This can only happen once per enemy god for a single cast of the ability. If you kill an enemy god with an ability your non-ultimate cooldowns are reduced by 3s. Each stack of Arcane Energy provides 2.5% Cooldown Reduction, stacks up to 4 times. At 4 stacks your next non-Ultimate ability cast nullifies the shield."],
    "obshard": ["PASSIVE - Your abilities gain 5% Magical Penetration for each non-Ultimate ability on Cooldown."],
    "spearofdeso": ["PASSIVE - When you kill an enemy god all of your non-ultimate cooldowns are reduced by 2 seconds and you gain 20% movement speed for 5s."],
    "spearmagus": ["PASSIVE - Whenever you damage an enemy god with an ability, you mark them to take 7.5% increased damage from all sources. This effect lasts for 7s and can only occur once every 15s."],
    "s_bumbas": ["PASSIVE - Your Basic Attacks deal +20 True Damage and your Abilities deal +30% Damage versus Jungle Monsters. When you kill a Jungle Monster you restore 10% Health and 20 Mana."],
    "s_bumbasspear": ["PASSIVE - Your Basic Attacks deal +50 True Damage and your Abilities deal +35% damage versus Jungle Monsters. After casting an ability your next Basic Attack deals an additional 30 True Damage. When you kill an enemy you gain 10% Movement Speed for 8s and restore 10% Health and Mana. Can be upgraded at level 20."],
    "s_manikin": ["PASSIVE - Basic Attacks apply a Damage over Time to Jungle Monsters. This effect lasts 3s and can stack up to 4 times. If you defeat a Large Jungle Monster while it is afflicted by the Burn, you gain 5% increased damage to Large Jungle Monsters and 3% Attack Speed for the next 30s. This effect can stack up to 3 times."],
    "s_hidden": ["PASSIVE - Basic Attacks apply a Damage over Time to Jungle Monsters. This effect lasts 3s and can stack up to 4 times. When you are attacked by an enemy god, you gain 5% Damage Reduction for 5s. This effect stacks up to 4 times. Can be upgraded at level 20."],
    "s_bluestone": ["PASSIVE - Enemies hit by your damaging Abilities take an additional 30 Physical Damage over 2s. Enemy gods take 45 Physical Damage over 2s. Can be upgraded at level 15."],
    "s_bluestonebrooch": ["PASSIVE - Enemies hit by your abilities take an additional 100 Physical Damage over 2s and are slowed by 15% for the duration. Can be upgraded at level 20."],
    "s_warriors": ["PASSIVE - Damaging an enemy god with an ability grants 50 protections for 3s. Slaying an enemy grants 100 Health and 100 Mana."],
    "s_sands": ["PASSIVE - This item grants 10 MP5 to allies within 70 units."],
    "s_pendulum": ["PASSIVE - This item grants 10 MP5 to allies within 70 units. Every 10s the Pendulum activates, subtracting 1s from all of your abilities currently on Cooldown. This ability only affects you."],
    "s_conduit": ["PASSIVE - Every second you gain a stack of Arcane Energy, causing your next damaging ability to deal an additional 2.5 True Damage per stack and remove all stacks. This effect stacks up to 20 times."],
    "s_eye": ["PASSIVE - Place a ward every 120s. Gain +20 HP5 while within 40 units of a ward. Your wards grant vision of targets through walls."],
    "s_protectors": ["PASSIVE - Allies within 55 units gain 15% Damage Mitigation. Enemy gods who damage you or allies take 20 + 5 per level damage."],
    "s_lonos": ["PASSIVE - You gain +20 HP5 while within assist range of an ally god. Being near an ally structure provides double the effect."],
    "rage2": ["PASSIVE - Gain 1 stack per Basic Attack hit, 2 stacks for Critical Strike hits. At 10 stacks your next Basic Attack is a Critical Strike. Stacks reset on successful Critical Strike."],
    "db_mal": ["PASSIVE - Critical Strike damage is increased by 30%. Critical Strikes also cause the target to Bleed, dealing 15% of your Physical Power every 0.5s for 2s."],
    "db_envenom": ["PASSIVE - Critical Strike damage is increased by 20%. Critical Strikes on enemy gods afflict them with poison for 2s. This poison slows them by 10% and reduces their damage output by 15%."],
    "serrated": ["PASSIVE - For every non-ultimate ability on cooldown you gain a stack of Cruelty. Each stack provides 7 Physical Power and 2% Physical Lifesteal."],
}


def main():
    # Load current items
    items_path = Path("../src/data/json/items.json")
    with open(items_path, 'r') as f:
        items = json.load(f)
    
    # Add missing effects
    updated_count = 0
    for item_key, item_data in items.items():
        if item_key in ITEM_PASSIVES and not item_data.get('effects'):
            item_data['effects'] = ITEM_PASSIVES[item_key]
            updated_count += 1
    
    # Sort items alphabetically
    sorted_items = OrderedDict(sorted(items.items()))
    
    # Save updated items
    with open(items_path, 'w') as f:
        json.dump(sorted_items, f, indent=2)
    
    print(f"Added effects to {updated_count} items")
    print("Items are now alphabetically sorted")
    
    # Also sort gods and abilities
    for filename in ["gods.json", "abilities.json"]:
        filepath = Path("../src/data/json") / filename
        if filepath.exists():
            with open(filepath, 'r') as f:
                data = json.load(f)
            
            sorted_data = OrderedDict(sorted(data.items()))
            
            with open(filepath, 'w') as f:
                json.dump(sorted_data, f, indent=2)
            
            print(f"{filename} is now alphabetically sorted")


if __name__ == '__main__':
    main()