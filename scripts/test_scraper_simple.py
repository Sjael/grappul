#!/usr/bin/env python3
"""
Simple test of scraper functionality without dependencies
"""

import json
from pathlib import Path


def slugify(name: str) -> str:
    """Test implementation of slugify"""
    import re
    # Remove apostrophes and quotes
    name = name.replace("'", "").replace('"', '')
    # Remove special characters except alphanumeric, spaces and hyphens
    name = re.sub(r'[^\w\s-]', '', name)
    # Replace spaces and hyphens with underscores
    name = re.sub(r'[-\s]+', '_', name)
    return name.lower()


def test_data_structure():
    """Test that our existing data has the correct structure"""
    print("Testing Data Structure...")
    print("-" * 50)
    
    data_dir = Path("../src/data/json")
    
    # Test existing JSON files
    print("✓ Checking JSON files structure:")
    
    # Check items.json
    items_path = data_dir / "items.json"
    if items_path.exists():
        with open(items_path) as f:
            items = json.load(f)
        
        print(f"\n  items.json: {len(items)} items")
        
        # Check a sample item
        sample_key = list(items.keys())[0]
        sample_item = items[sample_key]
        
        required_fields = ['display_name', 'price', 'stats', 'effects', 'tags']
        print(f"  Sample item '{sample_key}':")
        for field in required_fields:
            has_field = field in sample_item
            status = "✓" if has_field else "✗"
            print(f"    {status} has '{field}' field")
        
        # Check tag types
        if 'tags' in sample_item:
            print(f"    Tags: {sample_item['tags']}")
    
    # Check gods.json
    gods_path = data_dir / "gods.json"
    if gods_path.exists():
        with open(gods_path) as f:
            gods = json.load(f)
        
        print(f"\n  gods.json: {len(gods)} gods")
        
        # Check a sample god
        sample_key = list(gods.keys())[0]
        sample_god = gods[sample_key]
        
        required_fields = ['display_name', 'class', 'abilities']
        print(f"  Sample god '{sample_key}':")
        for field in required_fields:
            has_field = field in sample_god
            status = "✓" if has_field else "✗"
            print(f"    {status} has '{field}' field")
        
        if 'abilities' in sample_god:
            print(f"    Abilities: {len(sample_god['abilities'])} abilities")
    
    # Check abilities.json
    abilities_path = data_dir / "abilities.json"
    if abilities_path.exists():
        with open(abilities_path) as f:
            abilities = json.load(f)
        
        print(f"\n  abilities.json: {len(abilities)} abilities")
        
        # Check a sample ability
        sample_key = list(abilities.keys())[0]
        sample_ability = abilities[sample_key]
        
        required_fields = ['display_name', 'description', 'details']
        print(f"  Sample ability '{sample_key}':")
        for field in required_fields:
            has_field = field in sample_ability
            status = "✓" if has_field else "✗"
            print(f"    {status} has '{field}' field")
    
    # Test slugify
    print("\n✓ Testing slugify function:")
    test_names = [
        ("Cu Chulainn", "cu_chulainn"),
        ("Ah Muzen Cab", "ah_muzen_cab"),
        ("Chang'e", "change"),
        ("Nu Wa", "nu_wa"),
        ("He Bo", "he_bo"),
    ]
    
    for original, expected in test_names:
        result = slugify(original)
        status = "✓" if result == expected else "✗"
        print(f"  {status} '{original}' -> '{result}' (expected: '{expected}')")
    
    print("\n" + "-" * 50)
    print("Structure test complete!")


if __name__ == '__main__':
    test_data_structure()