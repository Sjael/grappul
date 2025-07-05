# SMITE Wiki Scraper

This script scrapes the SMITE Wiki (smite.fandom.com) to generate JSON files containing information about gods and items.

## Setup

1. Install Python 3.7 or higher
2. Install dependencies:
   ```bash
   pip install -r requirements.txt
   ```

## Usage

Run the script:
```bash
python wiki_scraper.py
```

The script will:
1. Create an `output` directory if it doesn't exist
2. Scrape all gods and their abilities into `output/abilities.json`
3. Scrape all items and their stats into `output/items.json`

## Features

The scraper collects:

### For Gods
- Passive ability
- All 4 abilities (1-4)
- For each ability:
  - Name
  - Description
  - Detailed stats (damage, cooldown, cost, etc.)

### For Items
- Name
- Stats
- Passive effects
- Active effects
- Price
- Tier

## Notes

- The script includes a 1-second delay between requests to be nice to the wiki
- It uses a custom user agent to identify itself
- All text is cleaned of wiki formatting
- Duplicate entries are automatically removed 