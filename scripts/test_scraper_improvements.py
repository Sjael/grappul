#!/usr/bin/env python3
"""
Test the improved scraper features
"""

import json
from pathlib import Path
from collections import OrderedDict


def test_improvements():
    """Test that the improvements are working correctly"""
    print("Testing Scraper Improvements...")
    print("="*50)
    
    data_dir = Path("../src/data/json")
    
    # Test 1: Check if data is alphabetically sorted
    print("\n1. Testing alphabetical sorting:")
    
    for filename in ["gods.json", "items.json", "abilities.json"]:
        filepath = data_dir / filename
        if filepath.exists():
            with open(filepath) as f:
                data = json.load(f)
            
            keys = list(data.keys())
            sorted_keys = sorted(keys)
            
            if keys == sorted_keys:
                print(f"   ✓ {filename} is alphabetically sorted")
            else:
                print(f"   ✗ {filename} is NOT alphabetically sorted")
                print(f"     First 5 keys: {keys[:5]}")
                print(f"     Should be: {sorted_keys[:5]}")
    
    # Test 2: Check if items have effects
    print("\n2. Testing item effects extraction:")
    items_path = data_dir / "items.json"
    if items_path.exists():
        with open(items_path) as f:
            items = json.load(f)
        
        # Check some known items that should have effects
        test_items = [
            "purification_beads",
            "aegis_amulet", 
            "blink_rune",
            "magic_shell",
            "temporal_beads"
        ]
        
        for item_key in test_items:
            if item_key in items:
                item = items[item_key]
                has_effects = len(item.get('effects', [])) > 0
                if has_effects:
                    print(f"   ✓ {item['display_name']} has effects: {len(item['effects'])} effect(s)")
                else:
                    print(f"   ✗ {item['display_name']} has NO effects")
    
    # Test 3: Check if abilities have details
    print("\n3. Testing ability details extraction:")
    abilities_path = data_dir / "abilities.json"
    if abilities_path.exists():
        with open(abilities_path) as f:
            abilities = json.load(f)
        
        # Sample first 10 abilities
        sample_abilities = list(abilities.items())[:10]
        abilities_with_details = 0
        
        for ability_key, ability in sample_abilities:
            if ability.get('details') and len(ability['details']) > 0:
                abilities_with_details += 1
        
        print(f"   Abilities with details: {abilities_with_details}/10")
        
        # Show a sample ability with details
        for ability_key, ability in sample_abilities:
            if ability.get('details') and len(ability['details']) > 0:
                print(f"\n   Sample ability '{ability['display_name']}':")
                for detail_key, detail_value in list(ability['details'].items())[:3]:
                    print(f"     - {detail_key}: {detail_value}")
                break
    
    # Test 4: Check for image paths
    print("\n4. Testing image path fields:")
    
    # Check if structs have image_path field
    for filename in ["gods.json", "items.json", "abilities.json"]:
        filepath = data_dir / filename
        if filepath.exists():
            with open(filepath) as f:
                data = json.load(f)
            
            # Check first item
            if data:
                first_key = list(data.keys())[0]
                first_item = data[first_key]
                has_image_path = 'image_path' in first_item
                
                if has_image_path:
                    print(f"   ✓ {filename} items have image_path field")
                else:
                    print(f"   ✗ {filename} items missing image_path field")
    
    print("\n" + "="*50)
    print("Test complete!")
    
    # Summary of what the v2 scraper improves:
    print("\nImprovements in v2 scraper:")
    print("1. Better passive/effect extraction for items")
    print("2. Wikitable parsing for ability details")
    print("3. Ability image extraction from god pages")
    print("4. Alphabetical sorting of all data")
    print("5. Multiple fallback methods for data extraction")


def simulate_sorting():
    """Simulate how the scraper would sort data"""
    print("\n\nSimulating data sorting...")
    print("-"*30)
    
    # Example data
    example_gods = {
        "zeus": {"display_name": "Zeus"},
        "achilles": {"display_name": "Achilles"},
        "ymir": {"display_name": "Ymir"},
        "bellona": {"display_name": "Bellona"},
        "agni": {"display_name": "Agni"}
    }
    
    print("Before sorting:")
    for key in example_gods:
        print(f"  {key}")
    
    # Sort using OrderedDict like the scraper does
    sorted_gods = OrderedDict(sorted(example_gods.items()))
    
    print("\nAfter sorting:")
    for key in sorted_gods:
        print(f"  {key}")


if __name__ == '__main__':
    test_improvements()
    simulate_sorting()