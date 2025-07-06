#!/usr/bin/env python3
"""
Test the scraper structure without actually scraping
"""

import sys
import json
from pathlib import Path

# Add the scripts directory to Python path
sys.path.insert(0, str(Path(__file__).parent))

from smite_scraper import SmiteScraper


def test_scraper_structure():
    """Test that the scraper structure is correct"""
    print("Testing Smite Scraper Structure...")
    print("-" * 50)
    
    # Create scraper instance
    scraper = SmiteScraper(delay=0.1)
    
    # Test directory creation
    print("✓ Scraper initialized")
    print(f"  Data directory: {scraper.DATA_DIR}")
    print(f"  Assets directory: {scraper.ASSETS_DIR}")
    
    # Test slugify function
    test_names = [
        ("Cu Chulainn", "cu_chulainn"),
        ("Ah Muzen Cab", "ah_muzen_cab"),
        ("Chang'e", "change"),
        ("Nu Wa", "nu_wa"),
        ("He Bo", "he_bo"),
        ("Zeus's Hammer", "zeuss_hammer"),
        ("Titan's Bane", "titans_bane")
    ]
    
    print("\n✓ Testing slugify function:")
    all_passed = True
    for original, expected in test_names:
        result = scraper._slugify(original)
        status = "✓" if result == expected else "✗"
        print(f"  {status} '{original}' -> '{result}' (expected: '{expected}')")
        if result != expected:
            all_passed = False
    
    # Test tag determination
    print("\n✓ Testing item tag determination:")
    test_items = [
        ("Purification Beads", "relic", ["Relic", "Tier1"]),
        ("Greater Purification Beads", "relic", ["Relic", "Tier2"]),
        ("Temporal Beads", "relic", ["Relic", "Tier3"]),
        ("Healing Potion", "consumable", ["Consumable"]),
        ("Bumba's Dagger", "starter", ["Starter"]),
        ("Bumba's Spear", "starter", ["Starter", "Evolved"]),
        ("Deathbringer", "tier3", ["Tier3"]),
        ("Evolved Rage", "evolved", ["Evolved", "Tier3"]),
    ]
    
    for item_name, category, expected_tags in test_items:
        result_tags = scraper._determine_item_tags(item_name, category)
        status = "✓" if result_tags == expected_tags else "✗"
        print(f"  {status} '{item_name}' ({category}) -> {result_tags} (expected: {expected_tags})")
    
    # Check if directories exist
    print("\n✓ Checking directories:")
    dirs_to_check = [
        scraper.DATA_DIR,
        scraper.ASSETS_DIR / "gods",
        scraper.ASSETS_DIR / "abilities", 
        scraper.ASSETS_DIR / "items"
    ]
    
    for dir_path in dirs_to_check:
        exists = dir_path.exists()
        status = "✓" if exists else "✗"
        print(f"  {status} {dir_path} {'exists' if exists else 'does not exist'}")
    
    # Check existing JSON files
    print("\n✓ Checking existing JSON files:")
    json_files = ["gods.json", "items.json", "abilities.json"]
    
    for json_file in json_files:
        file_path = scraper.DATA_DIR / json_file
        if file_path.exists():
            try:
                with open(file_path) as f:
                    data = json.load(f)
                print(f"  ✓ {json_file}: {len(data)} entries")
            except Exception as e:
                print(f"  ✗ {json_file}: Error reading file - {e}")
        else:
            print(f"  - {json_file}: File does not exist (will be created on scrape)")
    
    print("\n" + "-" * 50)
    print("Structure test complete!")
    print("\nTo run the actual scraper, use:")
    print("  python smite_scraper.py")
    print("\nTo run with verbose output:")
    print("  python smite_scraper.py --verbose")


if __name__ == '__main__':
    test_scraper_structure()