#!/usr/bin/env python3
"""
Consolidate and transform Smite data from various sources
into the required JSON format for the Rust application
"""

import json
import os
import re
from pathlib import Path
from typing import Dict, List, Any, Optional
import shutil

def slugify(name: str) -> str:
    """Convert name to lowercase with underscores, removing special characters
    This matches the Rust slugify function in utils/storage.rs"""
    # Remove apostrophes and quotes
    name = name.replace("'", "").replace('"', '')
    # Remove other special characters except spaces and hyphens
    name = re.sub(r'[^\w\s-]', '', name)
    # Replace spaces and hyphens with underscores
    name = re.sub(r'[-\s]+', '_', name)
    # Convert to lowercase
    return name.lower()

def load_json(filepath: Path) -> Dict:
    """Load JSON file"""
    if filepath.exists():
        with open(filepath, 'r', encoding='utf-8') as f:
            return json.load(f)
    return {}

def save_json(data: Any, filepath: Path):
    """Save data as JSON"""
    with open(filepath, 'w', encoding='utf-8') as f:
        json.dump(data, f, indent=2, ensure_ascii=False)

def transform_gods_data(scraped_gods: Dict, existing_abilities: Dict) -> Dict:
    """Transform gods data to required format"""
    transformed = {}
    
    for god_key, god_data in scraped_gods.items():
        # Get ability names from existing data if available
        ability_names = []
        if god_key in existing_abilities and isinstance(existing_abilities[god_key], dict):
            # Extract ability display names from existing data
            for ability_type in ['passive', 'ability1', 'ability2', 'ability3', 'ability4']:
                if ability_type in existing_abilities[god_key]:
                    ability_name = existing_abilities[god_key][ability_type].get('name', '')
                    if ability_name:
                        ability_names.append(ability_name)
        
        # If no abilities found in existing data, use scraped abilities
        if not ability_names and 'abilities' in god_data:
            ability_names = god_data['abilities']
        
        transformed[god_key] = {
            'display_name': god_data.get('display_name', ''),
            'class': god_data.get('class_name', ''),
            'abilities': ability_names
        }
    
    return transformed

def transform_abilities_data(existing_abilities: Dict) -> Dict:
    """Transform abilities data to required format"""
    transformed = {}
    
    for god_key, god_abilities in existing_abilities.items():
        if isinstance(god_abilities, dict):
            for ability_type, ability_data in god_abilities.items():
                if isinstance(ability_data, dict) and 'name' in ability_data:
                    ability_key = slugify(ability_data['name'])
                    
                    # Extract details into a flat structure
                    details = ability_data.get('details', {})
                    
                    transformed[ability_key] = {
                        'display_name': ability_data['name'],
                        'description': ability_data.get('description', ''),
                        'details': details
                    }
    
    return transformed

def transform_items_data(scraped_items: Dict) -> Dict:
    """Transform items data to required format"""
    transformed = {}
    
    for item_key, item_data in scraped_items.items():
        # Convert stats to proper format
        stats = []
        if 'stats' in item_data and item_data['stats']:
            # Handle both dict and list formats
            if isinstance(item_data['stats'], dict):
                for stat_name, stat_value in item_data['stats'].items():
                    # Convert stat names to display format
                    display_name = stat_name.replace('_', ' ').title()
                    stats.append(f"{display_name}: {stat_value}")
            elif isinstance(item_data['stats'], list):
                # Stats are already formatted as strings
                stats = item_data['stats']
        
        # Combine passive and active into effects
        effects = []
        if item_data.get('passive'):
            effects.append(f"Passive: {item_data['passive']}")
        if item_data.get('active'):
            # Don't add 'Active:' prefix if it's already in the string
            active_text = item_data['active']
            if not active_text.startswith('Active:') and not active_text.startswith('Using this item'):
                active_text = f"Active: {active_text}"
            effects.append(active_text)
        
        # Use cost or total_cost for price
        price = item_data.get('total_cost') or item_data.get('cost', 0)
        
        transformed[item_key] = {
            'display_name': item_data.get('display_name', ''),
            'price': price,
            'stats': stats,
            'effects': effects
        }
    
    return transformed

def copy_images(source_dir: Path, dest_dir: Path, image_type: str):
    """Copy images from source to destination, ensuring proper naming"""
    source_path = source_dir / image_type
    dest_path = dest_dir / image_type
    
    if not source_path.exists():
        print(f"Source directory {source_path} does not exist")
        return
    
    # Create destination directory if it doesn't exist
    dest_path.mkdir(parents=True, exist_ok=True)
    
    # Copy all images
    image_count = 0
    for image_file in source_path.glob('*.png'):
        dest_file = dest_path / image_file.name
        shutil.copy2(image_file, dest_file)
        image_count += 1
    
    print(f"Copied {image_count} {image_type} images")

def main():
    """Main consolidation process"""
    # Define paths
    script_dir = Path(__file__).parent
    output_dir = script_dir / 'output'
    src_data_dir = script_dir.parent / 'src' / 'data' / 'json'
    assets_dir = script_dir.parent / 'assets' / 'images'
    
    print("Loading data...")
    
    # Load scraped data - prefer manually updated versions
    gods_file = output_dir / 'gods_with_abilities.json'
    if not gods_file.exists():
        gods_file = output_dir / 'gods.json'
    scraped_gods = load_json(gods_file)
    
    items_file = output_dir / 'items_with_effects.json'
    if not items_file.exists():
        items_file = output_dir / 'items.json'
    scraped_items = load_json(items_file)
    
    # Load existing data
    existing_abilities = load_json(src_data_dir / 'abilities.json')
    
    print(f"Loaded {len(scraped_gods)} gods")
    print(f"Loaded {len(scraped_items)} items")
    print(f"Loaded abilities for {len(existing_abilities)} gods")
    
    # Transform data
    print("\nTransforming data...")
    
    gods_data = transform_gods_data(scraped_gods, existing_abilities)
    abilities_data = transform_abilities_data(existing_abilities)
    items_data = transform_items_data(scraped_items)
    
    print(f"Transformed {len(gods_data)} gods")
    print(f"Transformed {len(abilities_data)} abilities")
    print(f"Transformed {len(items_data)} items")
    
    # Save transformed data
    print("\nSaving transformed data...")
    
    output_gods_file = output_dir / 'gods_final.json'
    output_abilities_file = output_dir / 'abilities_final.json'
    output_items_file = output_dir / 'items_final.json'
    
    save_json(gods_data, output_gods_file)
    save_json(abilities_data, output_abilities_file)
    save_json(items_data, output_items_file)
    
    print(f"Saved gods to {output_gods_file}")
    print(f"Saved abilities to {output_abilities_file}")
    print(f"Saved items to {output_items_file}")
    
    # Copy images
    print("\nCopying images...")
    copy_images(output_dir, assets_dir, 'gods')
    copy_images(output_dir, assets_dir, 'items')
    copy_images(output_dir, assets_dir, 'abilities')
    
    print("\nConsolidation complete!")
    
    # Print summary of what needs to be done next
    print("\nNext steps:")
    print("1. Copy the *_final.json files to src/data/json/")
    print("2. Update Rust structs to match the JSON structure")
    print("3. Update Rust code to load from JSON files")

if __name__ == '__main__':
    main()