#!/usr/bin/env python3
"""
Convert item stats from array of strings to dictionary format
"""

import json
import re
from pathlib import Path
from collections import OrderedDict


def convert_stats(stats_array):
    """Convert array of stat strings to dictionary format"""
    stats_dict = {}
    
    # Map display names to ItemStat enum values
    stat_mapping = {
        'Physical Power': 'PhysicalPower',
        'Magical Power': 'MagicalPower',
        'Physical Protection': 'PhysicalProtection',
        'Magical Protection': 'MagicalProtection',
        'Health': 'Health',
        'Mana': 'Mana',
        'HP5': 'HP5',
        'MP5': 'MP5',
        'Attack Speed': 'AttackSpeed',
        'Physical Lifesteal': 'PhysicalLifesteal',
        'Magical Lifesteal': 'MagicalLifesteal',
        'Physical Penetration': 'PhysicalPenetration',
        'Magical Penetration': 'MagicalPenetration',
        'Critical Strike Chance': 'CriticalStrikeChance',
        'Cooldown Reduction': 'CooldownReduction',
        'Movement Speed': 'MovementSpeed',
        'Basic Attack Damage': 'BasicAttackDamage',
        'Damage Reduction': 'DamageReduction',
    }
    
    for stat_str in stats_array:
        # Parse "Stat Name: +Value" or "Stat Name: Value%"
        match = re.match(r'([^:]+):\s*\+?(\d+)%?', stat_str)
        if match:
            stat_name = match.group(1).strip()
            value = int(match.group(2))
            
            if stat_name in stat_mapping:
                stats_dict[stat_mapping[stat_name]] = value
    
    return stats_dict


def main():
    items_path = Path("../src/data/json/items.json")
    
    # Load current items
    with open(items_path, 'r') as f:
        items = json.load(f)
    
    # Convert each item's stats
    converted_count = 0
    for key, item_data in items.items():
        if 'stats' in item_data and isinstance(item_data['stats'], list):
            old_stats = item_data['stats']
            new_stats = convert_stats(old_stats)
            item_data['stats'] = new_stats
            
            if new_stats:
                converted_count += 1
        
        # Remove the redundant 'name' field if it exists
        if 'name' in item_data:
            del item_data['name']
        
        # Remove the 'image_path' field if it exists
        if 'image_path' in item_data:
            del item_data['image_path']
    
    # Sort items alphabetically
    sorted_items = OrderedDict(sorted(items.items()))
    
    # Save updated items
    with open(items_path, 'w') as f:
        json.dump(sorted_items, f, indent=2)
    
    print(f"Converted stats for {converted_count} items")
    print("Items are now using dictionary format for stats")


if __name__ == '__main__':
    main()