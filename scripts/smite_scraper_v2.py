#!/usr/bin/env python3
"""
Improved Smite Wiki Scraper with better passive/effect extraction and wikitable parsing
"""

import json
import logging
import re
import time
from collections import OrderedDict
from pathlib import Path
from typing import Dict, List, Optional, Any, Tuple
from urllib.parse import urljoin

import requests
from bs4 import BeautifulSoup
from requests.adapters import HTTPAdapter
from urllib3.util.retry import Retry


# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)


class SmiteScraper:
    """Improved scraper for all Smite wiki data"""
    
    BASE_URL = "https://smite.fandom.com"
    DATA_DIR = Path("../src/data/json")
    ASSETS_DIR = Path("../assets")
    
    def __init__(self, delay: float = 0.1):
        self.delay = delay
        self.session = self._create_session()
        self._ensure_directories()
    
    def _create_session(self) -> requests.Session:
        """Create session with retry logic"""
        session = requests.Session()
        retry = Retry(
            total=3,
            backoff_factor=0.3,
            status_forcelist=[500, 502, 503, 504]
        )
        adapter = HTTPAdapter(max_retries=retry)
        session.mount('http://', adapter)
        session.mount('https://', adapter)
        session.headers.update({
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36'
        })
        return session
    
    def _ensure_directories(self):
        """Create necessary directories"""
        self.DATA_DIR.mkdir(parents=True, exist_ok=True)
        for subdir in ['gods', 'abilities', 'items']:
            (self.ASSETS_DIR / subdir).mkdir(parents=True, exist_ok=True)
    
    def _get_soup(self, url: str) -> Optional[BeautifulSoup]:
        """Fetch and parse a page"""
        try:
            response = self.session.get(url)
            response.raise_for_status()
            time.sleep(self.delay)
            return BeautifulSoup(response.content, 'html.parser')
        except Exception as e:
            logger.error(f"Error fetching {url}: {e}")
            return None
    
    def _download_image(self, url: str, filename: str, subdir: str) -> str:
        """Download image and return relative path"""
        if not url:
            return ""
        
        try:
            # Clean the URL (remove any version/revision parameters)
            clean_url = url.split('/revision/')[0]
            
            response = self.session.get(clean_url)
            response.raise_for_status()
            
            filepath = self.ASSETS_DIR / subdir / filename
            with open(filepath, 'wb') as f:
                f.write(response.content)
            
            return f"{subdir}/{filename}"
        except Exception as e:
            logger.error(f"Error downloading image from {url}: {e}")
            return ""
    
    def _slugify(self, name: str) -> str:
        """Convert name to lowercase with underscores, matching Rust slugify"""
        # Remove apostrophes and quotes
        name = name.replace("'", "").replace('"', '')
        # Remove special characters except alphanumeric, spaces and hyphens
        name = re.sub(r'[^\w\s-]', '', name)
        # Replace spaces and hyphens with underscores
        name = re.sub(r'[-\s]+', '_', name)
        return name.lower()
    
    def scrape_god_icons(self):
        """Scrape god icons and names from the main Smite Wiki page
        
        Returns a dictionary mapping god slugs to god data
        """
        main_page_url = urljoin(self.BASE_URL, "/wiki/Smite_Wiki")
        soup = self._get_soup(main_page_url)
        if not soup:
            logger.error("Could not fetch main wiki page for god icons")
            return {}
        
        # Find the mp-heroes div
        heroes_div = soup.find('div', {'class': 'mp-heroes'})
        if not heroes_div:
            logger.error("Could not find mp-heroes div on main page")
            return {}
        
        god_data = {}
        icon_count = 0
        
        # Find all container divs (each contains a god icon and name)
        containers = heroes_div.find_all('div', style=lambda x: x and 'inline-block' in x)
        
        for container in containers:
            # Find the icon image
            icon_img = container.find('img', {'data-image-name': lambda x: x and 'Default Icon.png' in x})
            if not icon_img:
                continue
            
            image_name = icon_img.get('data-image-name', '')
            
            # Find the transparent overlay link to get the wiki URL
            overlay_link = container.find('a', href=lambda x: x and '/wiki/' in x and 'File:' not in x)
            wiki_path = overlay_link.get('href', '') if overlay_link else ''
            
            # Find the text div with the god name
            text_div = container.find('div', style=lambda x: x and 'text-align: center' in x and 'font-size' in x)
            god_name = text_div.get_text(strip=True) if text_div else ''
            
            if not god_name and image_name:
                # Fallback: extract from image name
                match = re.match(r'T[_ ](.+?)[_ ]Default[_ ]Icon\.png', image_name, re.IGNORECASE)
                if match:
                    god_name = match.group(1).replace('_', ' ')
            
            if god_name and image_name:
                # Get the actual image URL
                img_url = icon_img.get('data-src') or icon_img.get('src', '')
                
                # Skip data URIs
                if img_url.startswith('data:'):
                    parent_a = icon_img.find_parent('a', {'class': 'mw-file-description'})
                    if parent_a:
                        img_url = parent_a.get('href', '')
                
                if img_url and not img_url.startswith('data:'):
                    # Ensure it's a full URL
                    if img_url.startswith('//'):
                        img_url = 'https:' + img_url
                    elif img_url.startswith('/'):
                        img_url = urljoin(self.BASE_URL, img_url)
                    
                    # Remove scaling parameters
                    full_url = img_url.split('/scale-to-width-down/')[0]
                    full_url = full_url.split('/revision/')[0]
                    
                    # Download the icon
                    slugified_name = self._slugify(god_name)
                    icon_filename = f"{slugified_name}.png"
                    self._download_image(full_url, icon_filename, 'gods')
                    icon_count += 1
                    
                    # Store god data
                    god_data[slugified_name] = {
                        'display_name': god_name,
                        'wiki_path': wiki_path
                    }
                    
                    logger.debug(f"Downloaded icon for {god_name}, wiki path: {wiki_path}")
        
        logger.info(f"Downloaded {icon_count} god icons and found {len(god_data)} gods")
        return god_data
    
    def scrape_all(self):
        """Main entry point - scrape everything in stages"""
        logger.info("Starting Smite wiki scraping...")
        
        # Stage 0: Scrape god icons and names from main page
        logger.info("\n=== Stage 0: Scraping God Icons and Names ===")
        god_data_from_main = self.scrape_god_icons()
        
        # Stage 1: Scrape items
        logger.info("\n=== Stage 1: Scraping Items ===")
        items_data = self.scrape_items()
        logger.info(f"Found {len(items_data)} items")
        
        # Stage 2: Scrape gods and their abilities using data from stage 0
        logger.info("\n=== Stage 2: Scraping Gods and Abilities ===")
        gods_data, abilities_data = self.scrape_gods_and_abilities(god_data_from_main)
        logger.info(f"Found {len(gods_data)} gods and {len(abilities_data)} abilities")
        
        # Validate data before saving
        success = True
        
        if not items_data:
            logger.error("Failed to scrape any items - not saving items.json")
            success = False
        else:
            # Sort items alphabetically
            sorted_items = OrderedDict(sorted(items_data.items()))
            self._save_json(sorted_items, "items.json")
        
        if not gods_data:
            logger.error("Failed to scrape any gods - not saving gods.json")
            success = False
        else:
            # Sort data alphabetically
            sorted_gods = OrderedDict(sorted(gods_data.items()))
            self._save_json(sorted_gods, "gods.json")
        
        if not abilities_data:
            logger.error("Failed to scrape any abilities - not saving abilities.json")
            success = False
        else:
            sorted_abilities = OrderedDict(sorted(abilities_data.items()))
            self._save_json(sorted_abilities, "abilities.json")
        
        # Print summary
        self._print_summary(items_data, gods_data, abilities_data)
        
        if not success:
            logger.error("\nSCRAPER FAILED: One or more data types could not be scraped.")
            logger.error("The wiki structure may have changed. Please check the scraper logic.")
    
    def scrape_items(self) -> Dict[str, Dict]:
        """Scrape all items including relics"""
        items = {}
        
        # First, scrape regular items from the items page
        items_url = urljoin(self.BASE_URL, "/wiki/Items")
        soup = self._get_soup(items_url)
        if soup:
            items.update(self._scrape_items_from_page(soup))
        
        # Then, scrape relics from the dedicated relics page
        relics_url = urljoin(self.BASE_URL, "/wiki/Relics")
        soup = self._get_soup(relics_url)
        if soup:
            items.update(self._scrape_relics_from_page(soup))
        
        return items
    
    def _scrape_items_from_page(self, soup: BeautifulSoup) -> Dict[str, Dict]:
        """Scrape items from the main items page"""
        items = {}
        
        # Look for items in the items-overview-grid divs
        grid_containers = soup.find_all('div', {'class': 'items-overview-grid'})
        
        if not grid_containers:
            logger.warning("No items-overview-grid containers found, trying fallback method")
            # Fallback to old method
            return self._scrape_items_fallback(soup)
        
        logger.info(f"Found {len(grid_containers)} item grid containers")
        
        for grid in grid_containers:
            # Each item is in its own div within the grid
            item_divs = grid.find_all('div', recursive=False)
            logger.info(f"Found {len(item_divs)} items in this grid")
            
            for item_div in item_divs:
                # Find the item link
                link = item_div.find('a', href=True)
                if link:
                    href = link['href']
                    if '/wiki/' in href and not any(skip in href for skip in ['File:', 'Category:', 'Template:']):
                        # Get item name from title or text
                        display_name = link.get('title', '').strip()
                        if not display_name:
                            display_name = link.get_text(strip=True)
                        
                        if display_name and display_name != 'page does not exist':
                            # Determine category based on position or container
                            category = self._determine_item_category(grid, item_div)
                            
                            # Scrape individual item page for full details
                            key = self._slugify(display_name)
                            item_url = urljoin(self.BASE_URL, href)
                            
                            logger.debug(f"Scraping item: {display_name} from {item_url}")
                            item_data = self._scrape_individual_item(display_name, item_url, category)
                            
                            if item_data:
                                items[key] = item_data
                            else:
                                # Fallback: create basic item data if individual scrape fails
                                tags = self._determine_item_tags(display_name, category)
                                items[key] = {
                                    'display_name': display_name,
                                    'price': 0,
                                    'stats': {},
                                    'effects': [],
                                    'tags': tags
                                }
        
        logger.info(f"Total items found: {len(items)}")
        return items
    
    def _determine_item_category(self, grid: BeautifulSoup, item_div: BeautifulSoup) -> str:
        """Determine item category based on context"""
        # Look for a header before this grid
        prev = grid.find_previous(['h2', 'h3'])
        if prev:
            header_text = prev.get_text(strip=True).lower()
            if 'starter' in header_text:
                return 'starter'
            elif 'consumable' in header_text:
                return 'consumable'
            elif 'tier 1' in header_text:
                return 'tier1'
            elif 'tier 2' in header_text:
                return 'tier2'
            elif 'tier 3' in header_text:
                return 'tier3'
            elif 'glyph' in header_text:
                return 'glyph'
            elif 'evolved' in header_text:
                return 'evolved'
        
        # Default category
        return 'normal'
    
    def _scrape_items_fallback(self, soup: BeautifulSoup) -> Dict[str, Dict]:
        """Fallback method for scraping items"""
        items = {}
        
        # Item categories to look for
        categories = [
            ('Starter Items', 'starter'),
            ('Consumables', 'consumable'),
            ('Tier 1 Items', 'tier1'),
            ('Tier 2 Items', 'tier2'),
            ('Tier 3 Items', 'tier3'),
            ('Glyph Items', 'glyph'),
            ('Evolved Items', 'evolved')
        ]
        
        content = soup.find('div', {'class': 'mw-parser-output'})
        if not content:
            return items
        
        for section_name, category in categories:
            logger.info(f"Scraping {section_name}...")
            
            # Find section header
            header = None
            for h in content.find_all(['h2', 'h3']):
                if section_name.lower() in h.get_text(strip=True).lower():
                    header = h
                    break
            
            if not header:
                continue
            
            # Extract items from this section
            items_in_section = self._extract_items_from_section(header, category)
            items.update(items_in_section)
        
        return items
    
    def _extract_items_from_section(self, header, category: str) -> Dict[str, Dict]:
        """Extract items from a wiki section"""
        items = {}
        
        # Look for item links after the header
        current = header.find_next_sibling()
        while current and current.name not in ['h2', 'h3']:
            # Look for item links
            for link in current.find_all('a', href=True):
                href = link['href']
                if '/wiki/' in href and not any(skip in href for skip in ['File:', 'Category:', 'Template:']):
                    display_name = link.get('title', link.get_text()).strip()
                    if display_name and display_name != 'page does not exist':
                        # Scrape individual item
                        item_url = urljoin(self.BASE_URL, href)
                        item_data = self._scrape_individual_item(display_name, item_url, category)
                        if item_data:
                            # Use slugified name as key
                            key = self._slugify(display_name)
                            items[key] = item_data
            
            current = current.find_next_sibling()
        
        return items
    
    def _scrape_individual_item(self, display_name: str, url: str, category: str) -> Optional[Dict]:
        """Scrape data for a single item with improved passive extraction"""
        logger.debug(f"Scraping individual item: {display_name} from {url}")
        soup = self._get_soup(url)
        if not soup:
            logger.error(f"Failed to get soup for {url}")
            return None
        
        try:
            # Extract cost from infobox or page content
            cost = 0
            infobox = soup.find('aside', {'class': 'portable-infobox'})
            
            # If no portable-infobox, try other infobox structures
            if not infobox:
                logger.debug(f"No portable-infobox found for {display_name}, trying other structures")
                
                # First try to find the first table on the page (often the infobox)
                tables = soup.find_all('table')
                if tables:
                    # Check first table - it's usually the infobox
                    first_table = tables[0]
                    # Verify it has item-related content
                    table_text = first_table.get_text(strip=True).lower()
                    if any(keyword in table_text for keyword in ['cost:', 'stats:', 'tier:', 'passive', 'active']):
                        infobox = first_table
                        logger.debug(f"Found infobox as first table for {display_name}")
                
                # If still not found, try other approaches
                if not infobox:
                    infobox = soup.find('table', {'class': ['infobox', 'wikitable']})
                    if not infobox:
                        # Try to find any table that looks like an infobox
                        for table in soup.find_all('table'):
                            # Check if it has item-related headers
                            if any(keyword in str(table).lower() for keyword in ['cost:', 'stats:', 'tier:']):
                                infobox = table
                                logger.debug(f"Found alternative infobox for {display_name}")
                                break
            else:
                logger.debug(f"Found portable-infobox for {display_name}")
            
            # Try multiple methods to find cost - prefer Total Cost over Cost
            if infobox:
                # First try to get Total Cost
                cost_text = self._extract_infobox_value(infobox, 'Total Cost')
                if not cost_text:
                    # If no Total Cost, try regular Cost
                    cost_text = self._extract_infobox_value(infobox, 'Cost')
                
                if not cost_text:
                    # Try looking for cost in other formats
                    for elem in infobox.find_all(['div', 'td', 'th']):
                        text = elem.get_text(strip=True)
                        if 'Total Cost:' in text or 'Total cost:' in text:
                            cost_text = text
                            break
                        elif 'Cost:' in text or 'Gold:' in text or 'gold' in text.lower():
                            cost_text = text
                            # Don't break yet, keep looking for Total Cost
                
                match = re.search(r'(\d+)', cost_text) if cost_text else None
                cost = int(match.group(1)) if match else 0
            
            # If still no cost, look in page content
            if cost == 0:
                content = soup.find('div', {'class': 'mw-parser-output'})
                if content:
                    # Look for cost in any text
                    for elem in content.find_all(['p', 'div', 'td']):
                        text = elem.get_text(strip=True)
                        if re.search(r'(total cost|total price).*?(\d+)\s*gold', text, re.IGNORECASE):
                            match = re.search(r'(\d+)', text)
                            if match:
                                cost = int(match.group(1))
                                break
                        elif re.search(r'(cost|price).*?(\d+)\s*gold', text, re.IGNORECASE):
                            match = re.search(r'(\d+)', text)
                            if match:
                                cost = int(match.group(1))
                                # Don't break yet, keep looking for Total Cost
            
            # Extract stats from infobox or page
            stats = self._extract_item_stats_improved(soup, infobox)
            
            # Extract effects (passive/active) with improved method
            effects = self._extract_item_effects_improved(soup)
            
            # Determine tags
            tags = self._determine_item_tags(display_name, category)
            
            # Download image
            image_url = ""
            if infobox:
                img = infobox.find('img')
                if img and img.get('src'):
                    image_url = img['src']
            
            # Download image using slugified name
            slugified_name = self._slugify(display_name)
            image_filename = f"{slugified_name}.png"
            if image_url:
                self._download_image(image_url, image_filename, 'items')
            
            return {
                'display_name': display_name,
                'price': cost,
                'stats': stats,
                'effects': effects,
                'tags': tags
            }
            
        except Exception as e:
            logger.error(f"Error scraping item {display_name}: {e}")
            return None
    
    def _extract_item_stats_improved(self, soup: BeautifulSoup, infobox: BeautifulSoup) -> Dict[str, int]:
        """Extract item stats from infobox or page content as dictionary"""
        stats = {}
        
        # Map display names to ItemStat enum values
        stat_mapping = {
            'Physical Power': 'PhysicalPower',
            'Magical Power': 'MagicalPower',
            'Physical Protection': 'PhysicalProtection',
            'Magical Protection': 'MagicalProtection',
            'Health': 'Health',
            'Mana': 'Mana',
            'HP5': 'HP5',
            'MP5': 'MP5',
            'Attack Speed': 'AttackSpeed',
            'Physical Lifesteal': 'PhysicalLifesteal',
            'Magical Lifesteal': 'MagicalLifesteal',
            'Physical Penetration': 'PhysicalPenetration',
            'Magical Penetration': 'MagicalPenetration',
            'Critical Strike Chance': 'CriticalStrikeChance',
            'Cooldown Reduction': 'CooldownReduction',
            'Movement Speed': 'MovementSpeed',
            'Basic Attack Damage': 'BasicAttackDamage',
            'Damage Reduction': 'DamageReduction',
        }
        
        # Also handle variations and abbreviations
        stat_variations = {
            'Phys. Power': 'PhysicalPower',
            'Mag. Power': 'MagicalPower',
            'Phys. Protection': 'PhysicalProtection',
            'Mag. Protection': 'MagicalProtection',
            'Phys. Penetration': 'PhysicalPenetration',
            'Mag. Penetration': 'MagicalPenetration',
            'Phys. Lifesteal': 'PhysicalLifesteal',
            'Mag. Lifesteal': 'MagicalLifesteal',
            'Crit Chance': 'CriticalStrikeChance',
            'Critical Chance': 'CriticalStrikeChance',
            'CDR': 'CooldownReduction',
            'Move Speed': 'MovementSpeed',
        }
        
        # Combine mappings
        all_mappings = {**stat_mapping, **stat_variations}
        
        # First try to get stats from infobox
        if infobox:
            # Look for stat values in infobox data rows
            for stat_elem in infobox.find_all(['div', 'tr', 'li']):
                # Try different label/value structures
                label_elem = stat_elem.find(['h3', 'th'], {'class': ['pi-data-label', 'pi-secondary-font']})
                value_elem = stat_elem.find(['div', 'td'], {'class': ['pi-data-value', 'pi-font']})
                
                if not label_elem or not value_elem:
                    # Try simpler structure
                    cells = stat_elem.find_all(['td', 'th'])
                    if len(cells) >= 2:
                        label_elem = cells[0]
                        value_elem = cells[1]
                
                # Also try to parse the whole element text
                if not (label_elem and value_elem):
                    elem_text = stat_elem.get_text(strip=True)
                    # Look for pattern like "Physical Power: +30"
                    for stat_name, stat_key in all_mappings.items():
                        pattern = rf'{re.escape(stat_name)}[:\s]*[+]?(\d+)'
                        match = re.search(pattern, elem_text, re.IGNORECASE)
                        if match:
                            stats[stat_key] = int(match.group(1))
                
                if label_elem and value_elem:
                    label_text = label_elem.get_text(strip=True).replace(':', '')
                    value_text = value_elem.get_text(strip=True)
                    # Extract numeric value
                    match = re.search(r'[+]?(\d+)', value_text)
                    if match:
                        value = int(match.group(1))
                        if label_text in all_mappings:
                            stats[all_mappings[label_text]] = value
        
        # If no stats in infobox, try page content
        if not stats:
            content = soup.find('div', {'class': 'mw-parser-output'})
            if content:
                # Look for stats table
                for table in content.find_all('table', {'class': 'wikitable'}):
                    # Check if this is a stats table
                    headers = table.find_all('th')
                    if any('stat' in h.get_text(strip=True).lower() for h in headers):
                        for row in table.find_all('tr')[1:]:  # Skip header row
                            cells = row.find_all(['td', 'th'])
                            if len(cells) >= 2:
                                stat_name = cells[0].get_text(strip=True)
                                stat_value = cells[1].get_text(strip=True)
                                # Extract numeric value
                                match = re.search(r'[+]?(\d+)', stat_value)
                                if match:
                                    value = int(match.group(1))
                                    if stat_name in all_mappings:
                                        stats[all_mappings[stat_name]] = value
        
        # If still no stats, try extracting from the Stats: row in infobox
        if not stats and infobox:
            stats_text = self._extract_infobox_value(infobox, 'Stats')
            if stats_text:
                # Parse combined stats string like "+10 Physical Protection+10 Magical Protection+100 Health+7 MP5"
                # Split by + but keep the + sign for positive values
                stat_parts = re.findall(r'\+?(\d+)\s+([^+]+)', stats_text)
                for value_str, stat_name in stat_parts:
                    stat_name = stat_name.strip()
                    value = int(value_str)
                    # Try to match the stat name
                    for possible_name, stat_key in all_mappings.items():
                        if possible_name.lower() in stat_name.lower() or stat_name.lower() in possible_name.lower():
                            stats[stat_key] = value
                            break
        
        return stats
    
    def _extract_item_effects_improved(self, soup: BeautifulSoup) -> List[str]:
        """Extract item effects with improved passive detection"""
        effects = []
        
        content = soup.find('div', {'class': 'mw-parser-output'})
        if not content:
            return effects
        
        # Method 1: Look for passive/active sections
        for header in content.find_all(['h2', 'h3', 'h4']):
            header_text = header.get_text(strip=True).lower()
            if any(keyword in header_text for keyword in ['passive', 'active', 'effect', 'aura']):
                # Get the description from following elements
                current = header.find_next_sibling()
                effect_text = []
                
                while current and current.name not in ['h2', 'h3', 'h4']:
                    if current.name == 'p':
                        text = current.get_text(strip=True)
                        if text and len(text) > 10:  # Avoid very short text
                            effect_text.append(text)
                    elif current.name == 'ul':
                        # Handle bullet points
                        for li in current.find_all('li'):
                            text = li.get_text(strip=True)
                            if text and len(text) > 10:
                                effect_text.append(text)
                    elif current.name == 'dl':
                        # Handle definition lists
                        for dd in current.find_all('dd'):
                            text = dd.get_text(strip=True)
                            if text and len(text) > 10:
                                effect_text.append(text)
                    current = current.find_next_sibling()
                    if effect_text:
                        break
                
                if effect_text:
                    effects.extend(effect_text)
        
        # Method 2: Look for specific passive indicators in paragraphs
        if not effects:
            for p in content.find_all('p'):
                text = p.get_text(strip=True)
                if any(indicator in text for indicator in ['PASSIVE:', 'Passive:', 'ACTIVE:', 'Active:', 'Using this item', 'AURA:', 'Aura:']):
                    if len(text) > 20:  # Make sure it's a meaningful description
                        effects.append(text)
        
        # Method 3: Look for tables with passive information
        if not effects:
            for table in content.find_all('table', {'class': 'wikitable'}):
                # Check if table contains passive info
                table_text = table.get_text(strip=True).lower()
                if 'passive' in table_text or 'effect' in table_text:
                    # Extract text from table cells
                    for cell in table.find_all(['td', 'th']):
                        text = cell.get_text(strip=True)
                        if len(text) > 30 and not text.isdigit():  # Long text, not just numbers
                            effects.append(text)
                            break
        
        # Method 4: Check infobox for passive field
        if not effects:
            # Try multiple infobox types
            infobox = soup.find('aside', {'class': 'portable-infobox'})
            if not infobox:
                # Try to find the first table (often the infobox)
                tables = soup.find_all('table')
                if tables:
                    infobox = tables[0]
            
            if infobox:
                # Look for passive in various formats
                for label in ['Passive Effect', 'Passive', 'Effect', 'Active Effect', 'Active', 'Aura']:
                    passive_value = self._extract_infobox_value(infobox, label)
                    if passive_value and len(passive_value) > 20:
                        effects.append(passive_value)
                        break
        
        # Clean up effects - remove duplicates and very short entries
        unique_effects = []
        seen = set()
        for effect in effects:
            # Normalize the effect text
            normalized = effect.lower().strip()
            if normalized not in seen and len(effect) > 20:
                seen.add(normalized)
                unique_effects.append(effect)
        
        return unique_effects[:3]  # Limit to 3 effects to avoid too much text
    
    def _scrape_relics_from_page(self, soup: BeautifulSoup) -> Dict[str, Dict]:
        """Scrape relics from the relics page"""
        relics = {}
        
        content = soup.find('div', {'class': 'mw-parser-output'})
        if not content:
            return relics
        
        # Look for relic tiers
        for tier in [1, 2, 3]:
            logger.info(f"Scraping Tier {tier} Relics...")
            
            # Find tier header
            header = None
            for h in content.find_all(['h2', 'h3']):
                header_text = h.get_text(strip=True)
                if f'Tier {tier}' in header_text and 'Relic' in header_text:
                    header = h
                    break
            
            if not header:
                continue
            
            # Extract relics from this tier
            current = header.find_next_sibling()
            while current and current.name not in ['h2', 'h3']:
                # Look for relic links
                for link in current.find_all('a', href=True):
                    href = link['href']
                    if '/wiki/' in href and not any(skip in href for skip in ['File:', 'Category:', 'Template:']):
                        display_name = link.get('title', link.get_text()).strip()
                        if display_name and display_name != 'page does not exist':
                            # Scrape individual relic
                            relic_url = urljoin(self.BASE_URL, href)
                            relic_data = self._scrape_individual_relic(display_name, relic_url, tier)
                            if relic_data:
                                # Use slugified name as key
                                key = self._slugify(display_name)
                                relics[key] = relic_data
                
                current = current.find_next_sibling()
        
        return relics
    
    def _scrape_individual_relic(self, display_name: str, url: str, tier: int) -> Optional[Dict]:
        """Scrape data for a single relic"""
        soup = self._get_soup(url)
        if not soup:
            return None
        
        try:
            # Relics have no stats, just effects
            stats = []
            effects = []
            
            # Extract active effect - look in multiple places
            content = soup.find('div', {'class': 'mw-parser-output'})
            if content:
                # Look for effect description
                for elem in content.find_all(['p', 'div', 'li']):
                    text = elem.get_text(strip=True)
                    if 'Using this item' in text or 'Active:' in text or 'Effect:' in text:
                        effects.append(text)
                        break
                
                # If no effect found, look for first meaningful paragraph
                if not effects:
                    for p in content.find_all('p'):
                        text = p.get_text(strip=True)
                        if len(text) > 30 and not any(skip in text for skip in ['Relic', 'item', 'Tier']):
                            effects.append(text)
                            break
            
            # Determine tags based on tier
            tags = ['Relic']
            if tier == 1:
                tags.append('Tier1')
            elif tier == 2:
                tags.append('Tier2')
            elif tier == 3:
                tags.append('Tier3')
            
            # Download image
            image_url = ""
            infobox = soup.find('aside', {'class': 'portable-infobox'})
            if infobox:
                img = infobox.find('img')
                if img and img.get('src'):
                    image_url = img['src']
            
            # Download image using slugified name
            slugified_name = self._slugify(display_name)
            image_filename = f"{slugified_name}.png"
            if image_url:
                self._download_image(image_url, image_filename, 'items')
            
            return {
                'display_name': display_name,
                'price': 0,  # Relics are free
                'stats': stats,
                'effects': effects,
                'tags': tags
            }
            
        except Exception as e:
            logger.error(f"Error scraping relic {display_name}: {e}")
            return None
    
    def scrape_gods_and_abilities(self, god_data_from_main: Dict[str, Dict]) -> Tuple[Dict[str, Dict], Dict[str, Dict]]:
        """Scrape all gods and their abilities using pre-scraped god data
        
        Args:
            god_data_from_main: Dictionary mapping god slugs to god data with display_name and wiki_path
        """
        gods = {}
        abilities = {}
        
        if not god_data_from_main:
            logger.error("No god data provided from main page scraping")
            return gods, abilities
        
        # Process each god
        total_gods = len(god_data_from_main)
        for i, (god_slug, god_info) in enumerate(god_data_from_main.items()):
            god_name = god_info['display_name']
            wiki_path = god_info['wiki_path']
            
            # Build full URL
            if wiki_path.startswith('/'):
                god_url = urljoin(self.BASE_URL, wiki_path)
            else:
                god_url = wiki_path
            
            logger.info(f"Scraping god {i+1}/{total_gods}: {god_name}")
            
            # Scrape abilities from the god's page
            god_soup = self._get_soup(god_url)
            if not god_soup:
                logger.warning(f"Could not fetch page for {god_name}")
                continue
            
            # Extract abilities without relying on infobox
            god_abilities = self._extract_abilities_improved(god_soup, god_name)
            
            # Build god data
            god_data = {
                'display_name': god_name,
                'abilities': [self._slugify(ability['display_name']) for ability in god_abilities]
            }
            
            gods[god_slug] = god_data
            
            # Add abilities to the abilities dict
            for ability in god_abilities:
                ability_slug = self._slugify(ability['display_name'])
                abilities[ability_slug] = ability
        
        return gods, abilities
    
    def _extract_god_links_from_main_page(self, soup: BeautifulSoup) -> Dict[str, str]:
        """Extract god links from the main wiki page - specifically from mp-heroes div"""
        god_links = {}
        
        # CRITICAL: Look for the mp-heroes div which contains all god icons and links
        heroes_div = soup.find('div', {'class': 'mp-heroes'})
        if not heroes_div:
            logger.error("Could not find mp-heroes div on main page - scraper will fail!")
            return god_links
        
        # Find all links within mp-heroes that have god icons
        for link in heroes_div.find_all('a', href=True):
            href = link['href']
            
            # Skip file/image links
            if 'File:' in href or '.png' in href or '.jpg' in href:
                continue
                
            # Look for actual wiki page links
            if '/wiki/' in href and not any(skip in href.lower() for skip in 
                ['file:', 'category:', 'template:', 'list_of', 'smite_wiki', 
                 'special:', 'user:', 'talk:', '#', '.png', '.jpg']):
                
                # Try to get god name from the link
                god_name = link.get('title', '').strip()
                
                # If no title, try to extract from href
                if not god_name and '/wiki/' in href:
                    god_name = href.split('/wiki/')[-1].replace('_', ' ')
                
                # Clean up god name
                god_name = god_name.replace(' (page does not exist)', '')
                
                # Validate god name
                if god_name and len(god_name) > 1 and god_name not in ['Smite Wiki', 'Gods', 'Items']:
                    # Build full URL
                    if not href.startswith('http'):
                        full_url = urljoin(self.BASE_URL, href)
                    else:
                        full_url = href
                    
                    god_links[god_name] = full_url
                    logger.debug(f"Found god: {god_name} -> {full_url}")
        
        logger.info(f"Found {len(god_links)} god pages from mp-heroes div")
        
        # If we found too few, something is wrong
        if len(god_links) < 100:  # Smite has 130+ gods
            logger.warning(f"Only found {len(god_links)} gods - expected 100+. Wiki structure may have changed.")
        
        return god_links
    
    def _extract_god_links(self, soup: BeautifulSoup) -> Dict[str, str]:
        """Extract all god names and their wiki links"""
        god_links = {}
        
        # Find the gods table
        table = None
        for t in soup.find_all('table'):
            # Look for a table that likely contains gods
            if t.find('th', string=re.compile('Name|God', re.I)):
                table = t
                break
        
        if not table:
            logger.error("Could not find gods table")
            return god_links
        
        # Extract gods from table rows
        for row in table.find_all('tr')[1:]:  # Skip header
            cells = row.find_all(['td', 'th'])
            
            # Look for god link in cells
            for cell in cells:
                link = cell.find('a', href=re.compile(r'/wiki/[^:]+$'))
                if link and link.get('href'):
                    # Filter out utility pages
                    href = link['href']
                    if any(skip in href.lower() for skip in ['list_of', 'category:', 'template:', 'file:']):
                        continue
                    
                    god_name = link.get('title', link.text).strip()
                    god_name = god_name.replace(' (page does not exist)', '')
                    
                    if god_name and god_name not in ['Smite Wiki', 'Gods']:
                        god_links[god_name] = urljoin(self.BASE_URL, href)
                        break
        
        return god_links
    
    def _scrape_god_and_abilities(self, display_name: str, url: str) -> Tuple[Optional[Dict], List[Dict]]:
        """Scrape a god and their abilities with improved extraction"""
        soup = self._get_soup(url)
        if not soup:
            return None, []
        
        try:
            # Extract god info from infobox
            infobox = soup.find('aside', {'class': 'portable-infobox'})
            if not infobox:
                logger.warning(f"No infobox found for {display_name}")
                return None, []
            
            # Extract basic info
            god_class = self._extract_infobox_value(infobox, 'Class') or "Unknown"
            pantheon = self._extract_infobox_value(infobox, 'Pantheon') or "Unknown"
            title = self._extract_infobox_value(infobox, 'Title') or ""
            
            # Extract abilities with improved method
            abilities_data = self._extract_abilities_improved(soup, display_name)
            
            # Download god image
            god_image_url = ""
            img = infobox.find('img')
            if img and img.get('src'):
                god_image_url = img['src']
            
            # Download god image using slugified name
            slugified_name = self._slugify(display_name)
            god_image_filename = f"{slugified_name}.png"
            if god_image_url:
                self._download_image(god_image_url, god_image_filename, 'gods')
            
            god_data = {
                'display_name': display_name,
                'class': god_class,
                'pantheon': pantheon,
                'title': title,
                'abilities': [self._slugify(a['display_name']) for a in abilities_data]
            }
            
            return god_data, abilities_data
            
        except Exception as e:
            logger.error(f"Error scraping god {display_name}: {e}")
            return None, []
    
    def _extract_abilities_improved(self, soup: BeautifulSoup, god_name: str) -> List[Dict]:
        """Extract abilities from the 5 wikitable divs after the Abilities span"""
        abilities = []
        
        content = soup.find('div', {'class': 'mw-parser-output'})
        if not content:
            return abilities
        
        # Find the span with id="Abilities"
        abilities_span = content.find('span', {'id': 'Abilities'})
        if not abilities_span:
            logger.warning(f"Could not find Abilities span for {god_name}")
            return abilities
        
        # Get the parent element and search for wikitables after this point
        current = abilities_span.parent if abilities_span.parent else abilities_span
        wikitables_found = []
        ability_names = ['Passive', 'First Ability', 'Second Ability', 'Third Ability', 'Ultimate']
        
        # Look for the next 5 wikitable elements
        while current and len(wikitables_found) < 5:
            current = current.find_next_sibling()
            if not current:
                # If no more siblings, try going to parent's next sibling
                if hasattr(current, 'parent') and current.parent:
                    current = current.parent.find_next_sibling()
                if not current:
                    break
            
            # Check if current element is a wikitable
            if current.name == 'table' and 'wikitable' in current.get('class', []):
                wikitables_found.append(current)
            # Also check children for wikitables
            elif current.name:
                for table in current.find_all('table', {'class': 'wikitable'}, recursive=True):
                    if len(wikitables_found) < 5:
                        wikitables_found.append(table)
        
        logger.info(f"Found {len(wikitables_found)} ability wikitables for {god_name}")
        
        # Process each wikitable as an ability
        for idx, table in enumerate(wikitables_found):
            ability_type = ability_names[idx] if idx < len(ability_names) else f"Ability {idx + 1}"
            
            # Extract ability name from the table or nearby headers
            ability_name = self._extract_ability_name_from_table(table, ability_type)
            
            # Extract ability details from the wikitable
            ability_data = self._extract_ability_from_wikitable(table, ability_name, ability_type, god_name)
            
            if ability_data:
                # Debug: Log if we're using a generic description
                if ability_data['description'].endswith('ability'):
                    logger.debug(f"Generic description for {god_name} - {ability_name}")
                abilities.append(ability_data)
        
        return abilities
    
    def _extract_ability_name_from_table(self, table: BeautifulSoup, ability_type: str) -> str:
        """Extract ability name from wikitable"""
        # The ability name is in the first row of the table
        first_row = table.find('tr')
        if first_row:
            first_cell = first_row.find(['th', 'td'])
            if first_cell:
                # Look for spans in the cell
                spans = first_cell.find_all('span')
                if len(spans) >= 2:
                    # First span contains type (e.g., "Passive -", "1st Ability -")
                    # Second span contains the ability name
                    ability_name = spans[1].get_text(strip=True)
                    if ability_name and len(ability_name) > 1:
                        return ability_name
                
                # Fallback: try to extract from cell text
                cell_text = first_cell.get_text(strip=True)
                # Try to split by common separators
                for separator in [' - ', ': ', ' – ']:
                    if separator in cell_text:
                        parts = cell_text.split(separator, 1)
                        if len(parts) > 1:
                            ability_name = parts[1].strip()
                            if ability_name and len(ability_name) > 1:
                                return ability_name
        
        # If no name found, use generic name
        return f"{ability_type}"
    
    def _extract_ability_from_wikitable(self, table: BeautifulSoup, ability_name: str, 
                                       ability_type: str, god_name: str) -> Optional[Dict]:
        """Extract ability details from a wikitable"""
        try:
            # Parse the wikitable for ability stats
            details = self._parse_ability_wikitable(table)
            
            # Check if description was found in the wikitable
            description = ""
            if '_description' in details:
                description = details.pop('_description')  # Remove from details and use as description
            
            # If no description found in wikitable, try to find from surrounding text
            if not description:
                prev = table.find_previous_sibling()
                for _ in range(3):
                    if prev and prev.name == 'p':
                        desc_text = prev.get_text(strip=True)
                        if desc_text and len(desc_text) > 20:  # Reasonable description length
                            description = desc_text
                            break
                    if prev:
                        prev = prev.find_previous_sibling()
            
            # Look for ability image in the 3rd row (index 2), 1st td
            image_url = ""
            rows = table.find_all('tr')
            if len(rows) > 2:
                third_row = rows[2]
                first_td = third_row.find('td')
                if first_td:
                    img = first_td.find('img')
                    if img:
                        image_url = img.get('src', '') or img.get('data-src', '')
                        # Convert to full URL if needed
                        if image_url:
                            if image_url.startswith('//'):
                                image_url = 'https:' + image_url
                            elif image_url.startswith('/'):
                                image_url = urljoin(self.BASE_URL, image_url)
            
            # Download ability image if found
            if image_url:
                slugified_name = self._slugify(ability_name)
                image_filename = f"{slugified_name}.png"
                self._download_image(image_url, image_filename, 'abilities')
            
            return {
                'display_name': ability_name,
                'description': description or f"{ability_name} - {ability_type} ability",
                'details': details
            }
            
        except Exception as e:
            logger.error(f"Error extracting ability {ability_name} for {god_name}: {e}")
            return None
    
    def _extract_single_ability_improved(self, content: BeautifulSoup, header: BeautifulSoup, 
                                       ability_name: str, ability_type: str, god_slug: str) -> Optional[Dict]:
        """Extract details for a single ability with improved wikitable parsing"""
        try:
            description = ""
            details = {}
            image_url = ""
            
            # Look for ability image near the header
            # Check in header's parent and siblings for images
            search_area = header.parent if header.parent else header
            for img in search_area.find_all('img'):
                img_alt = img.get('alt', '').lower()
                img_src = img.get('src', '')
                if ability_name.lower() in img_alt or 'ability' in img_alt:
                    image_url = img_src
                    break
            
            # If no image found, check the next few siblings
            if not image_url:
                current = header
                for _ in range(5):  # Check next 5 elements
                    current = current.find_next_sibling()
                    if not current:
                        break
                    for img in current.find_all('img'):
                        img_alt = img.get('alt', '').lower()
                        img_src = img.get('src', '')
                        if ability_name.lower() in img_alt or 'icon' in img_alt:
                            image_url = img_src
                            break
                    if image_url:
                        break
            
            # Extract description
            current = header.find_next_sibling()
            desc_parts = []
            
            while current and current.name not in ['h2', 'h3', 'h4']:
                if current.name == 'p':
                    text = current.get_text(strip=True)
                    if text and not text.startswith('Ability Type:'):
                        desc_parts.append(text)
                
                # Look for wikitable with ability details
                elif current.name == 'table' and 'wikitable' in current.get('class', []):
                    # This is likely the ability details table
                    details.update(self._parse_ability_wikitable(current))
                
                current = current.find_next_sibling()
                
                # Stop if we have description and details
                if desc_parts and details:
                    break
            
            description = ' '.join(desc_parts)
            
            # Download ability image using slugified name
            slugified_name = self._slugify(ability_name)
            image_filename = f"{slugified_name}.png"
            if image_url:
                self._download_image(image_url, image_filename, 'abilities')
            
            return {
                'display_name': ability_name,
                'description': description or f"{ability_name} ability",
                'details': details
            }
            
        except Exception as e:
            logger.error(f"Error extracting ability {ability_name}: {e}")
            return None
    
    def _parse_ability_wikitable(self, table: BeautifulSoup) -> Dict[str, Any]:
        """Parse ability details from a wikitable"""
        details = {}
        description = ""
        
        # First pass: Look for the description specifically in row 3 (index 2)
        rows = table.find_all('tr')
        
        # Based on wiki structure, description is typically in the 3rd row with colspan="2"
        if len(rows) > 2:
            desc_row = rows[2]  # 3rd row (0-indexed)
            desc_cells = desc_row.find_all('td')
            
            for cell in desc_cells:
                # Look for cell with colspan="2" which typically contains the description
                if cell.get('colspan') == '2':
                    # Get text with proper spacing
                    description = cell.get_text(separator=' ', strip=True)
                    # Clean up any extra whitespace
                    description = ' '.join(description.split())
                    break
        
        # Fallback: If no description found in expected location, search more broadly
        if not description:
            stat_indicators = ['damage:', 'cost:', 'cooldown:', 'range:', 'heal:', 'duration:', 
                              'mana', 'ability type:', 'affects:', 'radius:', 'lifetime:', 'notes:']
            
            for row_idx, row in enumerate(rows):
                if row_idx == 0:  # Skip header row
                    continue
                    
                cells = row.find_all(['td', 'th'])
                
                for cell in cells:
                    text = cell.get_text(separator=' ', strip=True)
                    
                    # Skip empty or short cells
                    if not text or len(text) < 30:
                        continue
                    
                    # Skip cells with images
                    if cell.find('img'):
                        continue
                    
                    # Check for cells with colspan (often contain descriptions)
                    colspan = cell.get('colspan')
                    
                    if colspan and len(text) > 30:
                        # Make sure it's not a stat line
                        if not any(indicator in text.lower() for indicator in stat_indicators):
                            description = text
                            break
                
                if description:
                    break
        
        # Second pass: Parse stats from all cells
        for row in rows:
            cells = row.find_all(['td', 'th'])
            
            # Process each cell individually to find details
            for cell in cells:
                cell_text = cell.get_text(strip=True)
                
                # Skip empty cells or cells we've already processed as description
                if not cell_text or cell_text == description:
                    continue
                
                # Check if this cell contains a colon (indicating a detail)
                if ':' in cell_text:
                    # Split on the first colon
                    parts = cell_text.split(':', 1)
                    if len(parts) == 2:
                        label = parts[0].strip()
                        value = parts[1].strip()
                        
                        # Skip if this looks like a description
                        if len(value) > 100 and label.lower() not in ['ability type', 'affects', 'damage type']:
                            continue
                        
                        # Store the detail with the label as key
                        if label and value:
                            # Convert label to a consistent format (lowercase, underscores)
                            key = re.sub(r'[^\w\s]', '', label).strip().replace(' ', '_').lower()
                            if key:
                                details[key] = value
        
        # Store description in details if found
        if description:
            details['_description'] = description
        
        return details
    
    def _extract_infobox_value(self, infobox: BeautifulSoup, label: str) -> str:
        """Extract a value from the infobox by label (handles both portable infobox and regular tables)"""
        # First try portable infobox format
        for section in infobox.find_all('div', {'class': 'pi-item'}):
            label_elem = section.find('h3', {'class': 'pi-data-label'})
            if label_elem and label.lower() in label_elem.text.lower():
                value_elem = section.find('div', {'class': 'pi-data-value'})
                if value_elem:
                    return value_elem.text.strip()
        
        # If not found, try regular table format
        # Look for rows in the table
        for row in infobox.find_all('tr'):
            cells = row.find_all(['td', 'th'])
            if len(cells) >= 2:
                # Check if the label matches
                label_text = cells[0].get_text(strip=True).replace(':', '')
                if label.lower() in label_text.lower():
                    # Return the value from the second cell
                    return cells[1].get_text(strip=True)
        
        return ""
    
    def _determine_item_tags(self, display_name: str, category: str) -> List[str]:
        """Determine tags for an item"""
        tags = []
        display_lower = display_name.lower()
        
        # Map categories to tags
        if category == 'starter':
            tags.append('Starter')
            if any(word in display_lower for word in ['spear', 'blade', 'pendulum', 'brooch']):
                tags.append('Evolved')
        elif category == 'consumable':
            tags.append('Consumable')
        elif category == 'tier1':
            tags.append('Tier1')
        elif category == 'tier2':
            tags.append('Tier2')
        elif category == 'tier3':
            tags.append('Tier3')
        elif category == 'glyph':
            tags.append('Glyph')
            tags.append('Tier4')
        elif category == 'evolved':
            tags.append('Evolved')
            tags.append('Tier3')
        
        return tags
    
    def _save_json(self, data: Dict, filename: str):
        """Save data as JSON with failsafe to prevent saving empty data"""
        # Failsafe: Don't save empty or nearly empty data
        if not data or len(data) == 0:
            logger.warning(f"FAILSAFE: Refusing to save empty data to {filename}")
            return
        
        # Additional check for suspicious data (e.g., all entries are empty)
        if all(not v or (isinstance(v, dict) and not any(v.values())) for v in data.values()):
            logger.warning(f"FAILSAFE: Refusing to save data with all empty values to {filename}")
            return
        
        filepath = self.DATA_DIR / filename
        
        # Create backup if file exists and has content
        if filepath.exists() and filepath.stat().st_size > 0:
            backup_path = filepath.with_suffix('.json.backup')
            import shutil
            shutil.copy2(filepath, backup_path)
            logger.info(f"Created backup at {backup_path}")
        
        with open(filepath, 'w', encoding='utf-8') as f:
            json.dump(data, f, indent=2, ensure_ascii=False)
        logger.info(f"Saved {filename} with {len(data)} entries")
    
    def _print_summary(self, items: Dict, gods: Dict, abilities: Dict):
        """Print scraping summary"""
        print("\n" + "="*50)
        print("SCRAPING COMPLETE")
        print("="*50)
        print(f"Items scraped: {len(items)}")
        print(f"Gods scraped: {len(gods)}")
        print(f"Abilities scraped: {len(abilities)}")
        print(f"\nData saved to: {self.DATA_DIR.absolute()}")
        print(f"Images saved to: {self.ASSETS_DIR.absolute()}")
        print("="*50)


def test_single_item():
    """Test scraping a single item for debugging"""
    scraper = SmiteScraper(delay=0.1)
    
    # Test with a Tier 2 item that should have both Cost and Total Cost
    test_url = "https://smite.fandom.com/wiki/Sentinel%27s_Boon"
    logger.info(f"Testing single item scrape: {test_url}")
    
    item_data = scraper._scrape_individual_item("Sentinel's Boon", test_url, "starter")
    
    if item_data:
        logger.info(f"Item data: {json.dumps(item_data, indent=2)}")
    else:
        logger.error("Failed to scrape item data")
    
    return item_data


def main():
    """Run the scraper"""
    import argparse
    
    parser = argparse.ArgumentParser(description="Smite Wiki Scraper v2")
    parser.add_argument('--delay', type=float, default=0.1, help='Delay between requests (seconds)')
    parser.add_argument('--verbose', action='store_true', help='Enable verbose logging')
    parser.add_argument('--test', action='store_true', help='Test single item scraping')
    
    args = parser.parse_args()
    
    if args.verbose:
        logging.getLogger().setLevel(logging.DEBUG)
    
    if args.test:
        # Run single item test
        test_single_item()
    else:
        scraper = SmiteScraper(delay=args.delay)
        scraper.scrape_all()


if __name__ == '__main__':
    main()