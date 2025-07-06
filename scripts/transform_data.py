#!/usr/bin/env python3
"""
Transform scraped data to match Rust application structures
"""

import json
from pathlib import Path


def transform_gods(input_file: Path, output_file: Path):
    """Transform gods data to match Rust God struct"""
    with open(input_file, 'r') as f:
        scraped_gods = json.load(f)
    
    transformed_gods = {}
    
    for god_id, god_data in scraped_gods.items():
        # Transform to match Rust structure
        transformed_gods[god_id] = {
            "name": god_data["name"],
            "class": god_data["class_name"],  # Rename class_name to class
            "abilities": god_data["abilities"],  # Empty for now
            "roles": god_data["roles"]
        }
    
    with open(output_file, 'w') as f:
        json.dump(transformed_gods, f, indent=2)
    
    print(f"Transformed {len(transformed_gods)} gods")
    return transformed_gods


def extract_item_names(input_file: Path, output_file: Path):
    """Extract just item names for use in the application"""
    with open(input_file, 'r') as f:
        scraped_items = json.load(f)
    
    # Group items by category
    item_names = {
        "consumables": [],
        "relics": [],
        "starters": [],
        "items": []
    }
    
    for item_id, item_data in scraped_items.items():
        category = item_data.get("category", "")
        
        if category == "consumable":
            item_names["consumables"].append(item_id)
        elif category == "relic":
            item_names["relics"].append(item_id)
        elif category == "starter":
            item_names["starters"].append(item_id)
        else:
            item_names["items"].append(item_id)
    
    # Sort each category
    for category in item_names:
        item_names[category].sort()
    
    with open(output_file, 'w') as f:
        json.dump(item_names, f, indent=2)
    
    print(f"Extracted {sum(len(v) for v in item_names.values())} item names")
    return item_names


def main():
    output_dir = Path("output")
    
    # Transform gods data
    gods_input = output_dir / "gods.json"
    gods_output = output_dir / "gods_transformed.json"
    if gods_input.exists():
        transform_gods(gods_input, gods_output)
    else:
        print(f"Gods file not found: {gods_input}")
    
    # Extract item names
    items_input = output_dir / "items.json"
    items_output = output_dir / "item_names.json"
    if items_input.exists():
        extract_item_names(items_input, items_output)
    else:
        print(f"Items file not found: {items_input}")


if __name__ == "__main__":
    main()