#!/usr/bin/env python3
"""
Update tags for existing items in items.json without re-scraping
"""

import json
import re
from pathlib import Path
from typing import Dict, List

def determine_item_tags(display_name: str, price: int, stats: Dict[str, int], effects: Dict[str, str]) -> List[str]:
    """Determine tags for an item based on all available information"""
    tags = []
    display_lower = display_name.lower()
    
    # Check if it's a relic (relics have 0 or 800 price and usually have Active effects)
    relic_words = ['aegis', 'blink', 'bracer', 'meditation', 'barrier', 'wings', 'emblem', 
                   'shell', 'beads', 'thorns', 'sundering', 'teleport', 'phantom', 'temporal']
    if ((price == 0 or price == 800) and 'Active' in effects and 
        not any(word in display_lower for word in ['potion', 'ward', 'bomb'])) or \
       any(word in display_lower for word in relic_words):
        tags.append('Relic')
        return tags
    
    # Check if it's a consumable (but not other items with these words)
    consumable_words = ['potion', 'ward', 'bomb', 'elixir']
    exclude_words = ['warding sigil', 'heartward']
    if any(word in display_lower for word in consumable_words) and \
       not any(exclude in display_lower for exclude in exclude_words):
        tags.append('Consumable')
        return tags
    
    # Check if it's a shard (shards typically have "shard" in the name)
    if 'shard' in display_lower:
        tags.append('Shard')
        return tags
    
    # Check if it's a starter item (but not deathbringer)
    starter_words = ['mask', 'blessing', 'gift', 'toll', 'focus', 'embrace', 
                     'cowl', 'sentinel', 'bluestone', 'arrow', 'diamond', 
                     'eye of the jungle', 'tainted', 'leather', 'warding sigil', 'benevolence',
                     'animosity', 'compassion', 'spartan', 'pendulum', 'brooch', 'vampiric',
                     'protector\'s mask', 'sentinel\'s', 'bumba\'s', 'war flag', 'manikin',
                     'warrior\'s axe', 'conduit gem', 'sands of time', 'heroism', 'infused',
                     'sundering axe', 'archmage\'s', 'alternate timeline']
    if any(word in display_lower for word in starter_words) and 'deathbringer' not in display_lower:
        tags.append('Starter')
    
    # Check if it's a glyph item (has Glyph effect or specific names)
    glyph_names = ['eldritch', 'amulet of silence', 'amulet of the stronghold', 'sphinx']
    if 'Glyph' in effects or 'glyph' in display_lower or any(word in display_lower for word in glyph_names):
        tags.append('Glyph')
        if 'Tier4' not in tags:
            tags.append('Tier4')
    
    # Check if it's an evolved item
    if 'evolved' in display_lower:
        tags.append('Evolved')
        # Remove any tier tags that might have been added
        tags = [t for t in tags if t not in ['Tier1', 'Tier2']]
        if not any(t in tags for t in ['Tier3', 'Tier4']):
            tags.append('Tier3')
    
    # Determine tier based on price if not already set
    if not any(t in tags for t in ['Tier1', 'Tier2', 'Tier3', 'Tier4', 'Consumable', 'Relic', 'Starter']):
        if price > 0 and price <= 800:
            tags.append('Tier1')
        elif price > 800 and price <= 1500:
            tags.append('Tier2')
        elif price > 1500:
            tags.append('Tier3')
    
    # Ensure we have at least some tag
    if not tags:
        # Default to tier based on price
        if price <= 800:
            tags.append('Tier1')
        elif price <= 1500:
            tags.append('Tier2')
        else:
            tags.append('Tier3')
    
    return tags

def main():
    # Load existing items
    items_path = Path("../src/data/json/items.json")
    
    with open(items_path, 'r', encoding='utf-8') as f:
        items = json.load(f)
    
    # Update tags for each item
    updated_count = 0
    for item_key, item_data in items.items():
        display_name = item_data.get('display_name', '')
        price = item_data.get('price', 0)
        stats = item_data.get('stats', {})
        effects = item_data.get('effects', {})
        
        # Determine new tags
        new_tags = determine_item_tags(display_name, price, stats, effects)
        
        # Update if different
        old_tags = item_data.get('tags', [])
        if set(new_tags) != set(old_tags):
            item_data['tags'] = new_tags
            updated_count += 1
            print(f"Updated {display_name}: {old_tags} -> {new_tags}")
    
    # Save updated items
    with open(items_path, 'w', encoding='utf-8') as f:
        json.dump(items, f, indent=2, ensure_ascii=False)
    
    print(f"\nUpdated tags for {updated_count} items")

if __name__ == "__main__":
    main()