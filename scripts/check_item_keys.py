#!/usr/bin/env python3
"""
Check if item keys match slugified display names
"""

import json
import re
from pathlib import Path


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
    
    with open(items_path, 'r') as f:
        items = json.load(f)
    
    mismatches = []
    correct = 0
    
    for key, item_data in items.items():
        display_name = item_data.get('display_name', '')
        expected_key = slugify(display_name)
        
        if key != expected_key:
            mismatches.append({
                'current_key': key,
                'display_name': display_name,
                'expected_key': expected_key
            })
        else:
            correct += 1
    
    print(f"Total items: {len(items)}")
    print(f"Correct keys: {correct}")
    print(f"Mismatched keys: {len(mismatches)}")
    
    if mismatches:
        print("\nMismatches found:")
        for m in mismatches[:10]:  # Show first 10
            print(f"  Current: '{m['current_key']}' | Display: '{m['display_name']}' | Expected: '{m['expected_key']}'")
        
        if len(mismatches) > 10:
            print(f"  ... and {len(mismatches) - 10} more")


if __name__ == '__main__':
    main()