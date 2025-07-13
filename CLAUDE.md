# Grappul Project Memory

Reduce cache size as much as possible. this is a big project with a lot of images, and scripts. Unless specifically referenced, dont include images in the cache unless they are brought up


## Critical Scraper Information

### URL Structure (DO NOT FORGET)
The Smite wiki scraper MUST follow this exact URL pattern:

1. **God List and Icons**: https://smite.fandom.com/wiki/Smite_Wiki
   - This page contains the `mp-heroes` div with all god icons
   - Extract god names and icon images from here
   - Use this to build the list of gods to scrape

2. **Individual God Pages**: https://smite.fandom.com/wiki/[God]
   - Example: https://smite.fandom.com/wiki/Achilles
   - These pages contain:
     - God details (class, pantheon, roles, etc.)
     - All ability information
     - Ability descriptions and stats

**IMPORTANT**: If the scraper is not using these exact URLs, it WILL fail.

### Wiki Structure Changes
- The wiki no longer uses `portable-infobox` classes
- Need to adapt to current HTML structure
- God icons are in `mp-heroes` div on main page
- Each god icon link points to their individual page

### Ability Extraction (CRITICAL)
- Each god has exactly 5 abilities (Passive + 4 active abilities)
- On each god page, find the span with `id="Abilities"`
- The 5 `wikitable` class divs that come after this span contain the ability information
- These wikitables contain all ability stats (damage, cooldown, cost, etc.)

### Scraper Failsafes
- Never save empty data
- Create backups before overwriting
- Validate data before saving
- Clear error reporting when scraping fails