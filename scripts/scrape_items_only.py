#!/usr/bin/env python3
"""
Script to scrape only items from Smite wiki
"""

import sys
from pathlib import Path

# Add the scripts directory to the path so we can import the scraper
sys.path.insert(0, str(Path(__file__).parent))

from smite_scraper_v2 import SmiteScraper
import logging

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)

def main():
    logger.info("Starting items-only scraping...")
    
    # Create scraper instance
    scraper = SmiteScraper(delay=0.1)
    
    # Scrape only items
    logger.info("Scraping items...")
    items_data = scraper.scrape_items()
    
    if items_data:
        logger.info(f"Successfully scraped {len(items_data)} items")
        # Save the data
        scraper._save_json(items_data, "items.json")
        logger.info("Items data saved to items.json")
    else:
        logger.error("Failed to scrape items")
        sys.exit(1)

if __name__ == '__main__':
    main()