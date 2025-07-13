#!/usr/bin/env python3
"""
Migrate items.json from Vec<String> effects to BTreeMap<Effect, String> effects
"""

import json
import sys
from typing import Dict, List, Any, Optional, Tuple

def detect_effect_type(effect_text: str) -> Optional[str]:
    """
    Detect the type of effect from the text.
    Returns "Active", "Passive", or "Glyph"
    """
    effect_lower = effect_text.lower()
    
    # Check for explicit indicators
    if effect_text.startswith(("ACTIVE:", "Active:", "Using this item")):
        return "Active"
    elif effect_text.startswith(("PASSIVE:", "Passive:", "AURA:", "Aura:")):
        return "Passive"
    elif effect_text.startswith(("GLYPH:", "Glyph:")):
        return "Glyph"
    
    # Check for active indicators in the text
    if any(phrase in effect_lower for phrase in [
        "using this item", "activate to", "when activated", "on use",
        "cooldown:", "active ability", "can be activated"
    ]):
        return "Active"
    
    # Check for glyph indicators
    if "glyph" in effect_lower:
        return "Glyph"
    
    # Default to passive for most effects
    if effect_text.strip():
        return "Passive"
    
    return None

def clean_effect_text(effect_text: str, effect_type: str) -> str:
    """
    Clean the effect text by removing type prefixes
    """
    # Remove common prefixes
    prefixes_to_remove = [
        "PASSIVE:", "Passive:", "ACTIVE:", "Active:", 
        "GLYPH:", "Glyph:", "AURA:", "Aura:"
    ]
    
    cleaned = effect_text
    for prefix in prefixes_to_remove:
        if cleaned.startswith(prefix):
            cleaned = cleaned[len(prefix):].strip()
            break
    
    return cleaned

def migrate_item(item_data: Dict[str, Any]) -> Dict[str, Any]:
    """
    Migrate a single item to the new format
    """
    # Convert effects from List to Dict
    old_effects = item_data.get("effects", [])
    new_effects = {}
    
    for effect in old_effects:
        if not effect or not effect.strip():
            continue
            
        effect_type = detect_effect_type(effect)
        if effect_type:
            cleaned_text = clean_effect_text(effect, effect_type)
            # Only add if we don't already have an effect of this type
            if effect_type not in new_effects:
                new_effects[effect_type] = cleaned_text
    
    # Update the item data
    item_data["effects"] = new_effects
    
    # Also need to migrate stats from list of strings to dict
    old_stats = item_data.get("stats", [])
    new_stats = {}
    
    stat_mapping = {
        "Physical Power": "PhysicalPower",
        "Magical Power": "MagicalPower", 
        "Physical Protection": "PhysicalProtection",
        "Magical Protection": "MagicalProtection",
        "Health": "Health",
        "Mana": "Mana",
        "HP5": "HP5",
        "MP5": "MP5",
        "Attack Speed": "AttackSpeed",
        "Physical Lifesteal": "PhysicalLifesteal",
        "Magical Lifesteal": "MagicalLifesteal",
        "Physical Penetration": "PhysicalPenetration",
        "Magical Penetration": "MagicalPenetration",
        "Critical Strike Chance": "CriticalStrikeChance",
        "Cooldown Reduction": "CooldownReduction",
        "Movement Speed": "MovementSpeed",
        "Basic Attack Damage": "BasicAttackDamage",
        "Damage Reduction": "DamageReduction",
        "Penetration": "PhysicalPenetration"  # Generic penetration
    }
    
    for stat_str in old_stats:
        if isinstance(stat_str, str) and ": " in stat_str:
            stat_name, value_str = stat_str.split(": ", 1)
            
            # Map to the enum value
            if stat_name in stat_mapping:
                enum_key = stat_mapping[stat_name]
                
                # Parse the value
                try:
                    # Remove % sign if present
                    value_str = value_str.rstrip('%')
                    value = int(float(value_str))
                    new_stats[enum_key] = value
                except ValueError:
                    print(f"Warning: Could not parse stat value '{value_str}' for {stat_name}")
    
    item_data["stats"] = new_stats
    
    return item_data

def main():
    # Read the backup file
    input_file = "/home/jakeo/proj/grappul/backup/json/items.json"
    output_file = "/home/jakeo/proj/grappul/src/data/json/items_migrated.json"
    
    print(f"Reading from {input_file}")
    
    with open(input_file, 'r') as f:
        items = json.load(f)
    
    # Migrate each item
    migrated_items = {}
    for item_key, item_data in items.items():
        migrated_items[item_key] = migrate_item(item_data)
    
    # Write the migrated data
    print(f"Writing to {output_file}")
    with open(output_file, 'w') as f:
        json.dump(migrated_items, f, indent=2)
    
    # Print summary
    total_items = len(migrated_items)
    items_with_effects = sum(1 for item in migrated_items.values() if item.get("effects"))
    
    print(f"\nMigration complete!")
    print(f"Total items: {total_items}")
    print(f"Items with effects: {items_with_effects}")
    
    # Sample a few items with effects
    print("\nSample migrated items with effects:")
    count = 0
    for key, item in migrated_items.items():
        if item.get("effects") and count < 5:
            print(f"\n{key}: {item['display_name']}")
            for effect_type, effect_text in item["effects"].items():
                print(f"  {effect_type}: {effect_text[:60]}...")
            count += 1

if __name__ == "__main__":
    main()