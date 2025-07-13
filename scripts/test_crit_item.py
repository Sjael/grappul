#!/usr/bin/env python3
"""
Test scraping a specific item with Critical Strike Chance
"""

import sys
from pathlib import Path
import logging

# Add the scripts directory to the path
sys.path.insert(0, str(Path(__file__).parent))

from smite_scraper_v2 import SmiteScraper

# Configure logging
logging.basicConfig(
    level=logging.DEBUG,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)

def main():
    scraper = SmiteScraper(delay=0.1)
    
    # Test with Deathbringer - a classic crit item
    test_url = "https://smite.fandom.com/wiki/Deathbringer"
    logger.info(f"Testing item scrape: {test_url}")
    
    item_data = scraper._scrape_individual_item("Deathbringer", test_url, "tier3")
    
    if item_data:
        import json
        logger.info(f"Item data: {json.dumps(item_data, indent=2)}")
        
        # Check if CriticalStrikeChance is in stats
        if 'stats' in item_data and 'CriticalStrikeChance' in item_data['stats']:
            logger.info(f"SUCCESS: Found Critical Strike Chance: {item_data['stats']['CriticalStrikeChance']}%")
        else:
            logger.warning("WARNING: Critical Strike Chance not found in stats")
            logger.info(f"Stats found: {item_data.get('stats', {})}")
    else:
        logger.error("Failed to scrape item data")

if __name__ == '__main__':
    main()