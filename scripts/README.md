# Smite Wiki Scraper

A consolidated web scraper for extracting game data from the Smite Fandom Wiki.

## Features

- Scrapes items, gods, and abilities data
- Downloads images for all entities
- Saves data in JSON format compatible with the Rust application
- Two-stage scraping: items first, then gods with their abilities
- Automatic retry logic for failed requests
- Configurable delay between requests

## Installation

1. Install Python dependencies:
```bash
pip install -r requirements.txt
```

## Usage

### Basic Usage

Run the scraper with default settings:
```bash
python smite_scraper.py
```

### Options

- `--delay`: Set delay between requests in seconds (default: 0.5)
- `--verbose`: Enable verbose logging

Example:
```bash
python smite_scraper.py --delay 0.1 --verbose
```

## Output

The scraper saves data to:
- **JSON data**: `../src/data/json/`
  - `items.json`: All items including relics, consumables, etc.
  - `gods.json`: All gods with their metadata
  - `abilities.json`: All abilities with descriptions and details
- **Images**: `../assets/`
  - `gods/`: God portraits
  - `items/`: Item icons
  - `abilities/`: Ability icons

## Data Structure

### Items
```json
{
  "beads": {
    "display_name": "Purification Beads",
    "price": 0,
    "stats": [],
    "effects": ["Using this item removes Crowd Control..."],
    "tags": ["Relic", "Tier1"]
  }
}
```

### Gods
```json
{
  "achilles": {
    "display_name": "Achilles",
    "class": "Warrior",
    "abilities": ["gift_of_the_gods", "shield_of_achilles", ...]
  }
}
```

### Abilities
```json
{
  "shield_of_achilles": {
    "display_name": "Shield of Achilles",
    "description": "Achilles punches forward with the edge of his Shield...",
    "details": {
      "damage": "100/155/210/265/320",
      "cooldown": "14s"
    }
  }
}
```

## Testing

Run the structure test without scraping:
```bash
python test_scraper_simple.py
```

## Notes

- The scraper includes automatic retry logic for failed requests
- Images are downloaded with cleaned filenames using the slugify function
- All names are normalized to lowercase with underscores (matching the Rust slugify)
- Item tags are automatically determined based on item characteristics