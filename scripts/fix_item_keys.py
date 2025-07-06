#!/usr/bin/env python3
"""
Fix item keys to match slugified display names
"""

import json
import re
from pathlib import Path
from collections import OrderedDict


def slugify(name: str) -> str:
    """Convert name to lowercase with underscores, matching Rust slugify"""
    # Remove apostrophes and quotes
    name = name.replace("'", "").replace('"', '')
    # Remove special characters except alphanumeric, spaces and hyphens
    name = re.sub(r'[^\w\s-]', '', name)
    # Replace spaces and hyphens with underscores
    name = re.sub(r'[-\s]+', '_', name)
    return name.lower()


def main():
    items_path = Path("../src/data/json/items.json")
    
    # Load current items
    with open(items_path, 'r') as f:
        items = json.load(f)
    
    # Create new items dict with correct keys
    new_items = OrderedDict()
    key_mapping = {}  # Track old key -> new key mapping
    
    for old_key, item_data in items.items():
        display_name = item_data.get('display_name', '')
        new_key = slugify(display_name)
        
        # Handle potential duplicates
        if new_key in new_items:
            print(f"Warning: Duplicate key '{new_key}' for '{display_name}' (old key: '{old_key}')")
            # Add a suffix to make it unique
            suffix = 2
            while f"{new_key}_{suffix}" in new_items:
                suffix += 1
            new_key = f"{new_key}_{suffix}"
        
        new_items[new_key] = item_data
        key_mapping[old_key] = new_key
        
        if old_key != new_key:
            print(f"Renamed: '{old_key}' -> '{new_key}'")
    
    # Sort alphabetically
    sorted_items = OrderedDict(sorted(new_items.items()))
    
    # Save updated items
    with open(items_path, 'w') as f:
        json.dump(sorted_items, f, indent=2)
    
    print(f"\nTotal items processed: {len(items)}")
    print(f"Keys updated: {sum(1 for old, new in key_mapping.items() if old != new)}")
    
    # Save key mapping for reference
    mapping_path = Path("item_key_mapping.json")
    with open(mapping_path, 'w') as f:
        json.dump(key_mapping, f, indent=2)
    print(f"\nKey mapping saved to: {mapping_path}")


if __name__ == '__main__':
    main()