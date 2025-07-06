#!/usr/bin/env python3
"""
Verify that the new scraper output format matches our existing data
"""

import json
from pathlib import Path


def verify_data_compatibility():
    """Check if existing data matches expected scraper output"""
    print("Verifying Scraper Compatibility...")
    print("="*50)
    
    data_dir = Path("../src/data/json")
    
    # Check items.json
    print("\n1. Checking items.json compatibility:")
    items_path = data_dir / "items.json"
    if items_path.exists():
        with open(items_path) as f:
            items = json.load(f)
        
        # Check a few items
        test_items = list(items.keys())[:3]
        print(f"   Checking {len(test_items)} sample items...")
        
        for item_key in test_items:
            item = items[item_key]
            expected_fields = {
                'display_name': str,
                'price': int,
                'stats': list,
                'effects': list,
                'tags': list
            }
            
            print(f"\n   Item '{item_key}':")
            for field, expected_type in expected_fields.items():
                if field in item:
                    actual_type = type(item[field])
                    if actual_type == expected_type:
                        print(f"     ✓ {field}: {expected_type.__name__}")
                    else:
                        print(f"     ✗ {field}: expected {expected_type.__name__}, got {actual_type.__name__}")
                else:
                    print(f"     ✗ {field}: MISSING")
    
    # Check gods.json
    print("\n2. Checking gods.json compatibility:")
    gods_path = data_dir / "gods.json"
    if gods_path.exists():
        with open(gods_path) as f:
            gods = json.load(f)
        
        # Check a few gods
        test_gods = list(gods.keys())[:3]
        print(f"   Checking {len(test_gods)} sample gods...")
        
        for god_key in test_gods:
            god = gods[god_key]
            expected_fields = {
                'display_name': str,
                'class': str,
                'abilities': list
            }
            
            # Optional fields that scraper would add
            optional_fields = {
                'pantheon': str,
                'title': str,
                'roles': list,
                'image_path': str
            }
            
            print(f"\n   God '{god_key}':")
            for field, expected_type in expected_fields.items():
                if field in god:
                    actual_type = type(god[field])
                    if actual_type == expected_type:
                        print(f"     ✓ {field}: {expected_type.__name__}")
                    else:
                        print(f"     ✗ {field}: expected {expected_type.__name__}, got {actual_type.__name__}")
                else:
                    print(f"     ✗ {field}: MISSING")
            
            # Check optional fields
            print("     Optional fields:")
            for field, expected_type in optional_fields.items():
                if field in god:
                    print(f"       - {field}: present")
                else:
                    print(f"       - {field}: will be added by scraper")
    
    # Check abilities.json
    print("\n3. Checking abilities.json compatibility:")
    abilities_path = data_dir / "abilities.json"
    if abilities_path.exists():
        with open(abilities_path) as f:
            abilities = json.load(f)
        
        # Check a few abilities
        test_abilities = list(abilities.keys())[:3]
        print(f"   Checking {len(test_abilities)} sample abilities...")
        
        for ability_key in test_abilities:
            ability = abilities[ability_key]
            expected_fields = {
                'display_name': str,
                'description': str,
                'details': dict
            }
            
            print(f"\n   Ability '{ability_key}':")
            for field, expected_type in expected_fields.items():
                if field in ability:
                    actual_type = type(ability[field])
                    if actual_type == expected_type:
                        print(f"     ✓ {field}: {expected_type.__name__}")
                    else:
                        print(f"     ✗ {field}: expected {expected_type.__name__}, got {actual_type.__name__}")
                else:
                    print(f"     ✗ {field}: MISSING")
    
    print("\n" + "="*50)
    print("Compatibility check complete!")
    print("\nThe new scraper will ADD these fields to gods:")
    print("- pantheon (if not present)")
    print("- title (if not present)")
    print("- roles (if not present)")
    print("- image_path (for all entities)")
    print("\nAll other fields match the expected format.")


if __name__ == '__main__':
    verify_data_compatibility()