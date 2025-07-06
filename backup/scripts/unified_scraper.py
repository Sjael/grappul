#!/usr/bin/env python3
"""
Unified Smite Wiki Scraper
Scrapes gods, abilities, and items data from the Smite Fandom Wiki
"""

import argparse
import json
import logging
import os
import re
import sys
import time
from abc import ABC, abstractmethod
from dataclasses import dataclass, asdict
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Optional, Any
from urllib.parse import urljoin, unquote

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


# Data classes matching Rust structures
@dataclass
class God:
    """Matches the God struct in src/data/gods.rs"""
    name: str
    display_name: str
    class_name: str
    pantheon: str
    title: str
    roles: List[str]
    abilities: List[str]
    image_path: str


@dataclass
class Ability:
    """Represents an ability with its details"""
    name: str
    display_name: str
    god: str
    type: str  # passive, ability1, ability2, ability3, ultimate
    description: str
    details: Dict[str, Any]
    image_path: str


@dataclass
class Item:
    """Matches the Item structure expected by the app"""
    name: str
    display_name: str
    price: int
    stats: List[str]
    effects: List[str]
    tags: List[str]
    image_path: str


class BaseScraper(ABC):
    """Base scraper with common functionality"""
    
    BASE_URL = "https://smite.fandom.com"
    DATA_DIR = Path("../src/data/json")
    ASSETS_DIR = Path("../assets")
    
    def __init__(self, delay: float = 0.1):
        self.delay = delay
        self.session = self._create_session()
        self._ensure_output_dirs()
    
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
    
    def _ensure_output_dirs(self):
        """Create output directories if they don't exist"""
        # Ensure data directory exists
        self.DATA_DIR.mkdir(parents=True, exist_ok=True)
        # Ensure assets subdirectories exist
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
        """Convert name to lowercase with underscores, removing special characters
        This matches the Rust slugify function in utils/storage.rs"""
        # Remove apostrophes and quotes
        name = name.replace("'", "").replace('"', '')
        # Remove special characters except alphanumeric, spaces and hyphens
        name = re.sub(r'[^\w\s-]', '', name)
        # Replace spaces and hyphens with underscores
        name = re.sub(r'[-\s]+', '_', name)
        return name.lower()
    
    def _save_json(self, data: Any, filename: str):
        """Save data as JSON"""
        filepath = self.DATA_DIR / filename
        with open(filepath, 'w', encoding='utf-8') as f:
            json.dump(data, f, indent=2, ensure_ascii=False)
        logger.info(f"Saved {filename}")
    
    @abstractmethod
    def scrape(self) -> Dict[str, Any]:
        """Main scraping method to be implemented by subclasses"""
        pass


class GodsScraper(BaseScraper):
    """Scraper for gods data"""
    
    def scrape(self) -> Dict[str, God]:
        """Scrape all gods data"""
        gods_url = urljoin(self.BASE_URL, "/wiki/List_of_gods")
        soup = self._get_soup(gods_url)
        if not soup:
            return {}
        
        gods = {}
        god_links = self._extract_god_links(soup)
        
        for i, (name, link) in enumerate(god_links.items()):
            logger.info(f"Scraping god {i+1}/{len(god_links)}: {name}")
            god_data = self._scrape_god(name, link)
            if god_data:
                gods[god_data.name] = god_data
        
        return gods
    
    def _extract_god_links(self, soup: BeautifulSoup) -> Dict[str, str]:
        """Extract all god names and their wiki links"""
        god_links = {}
        
        # Try multiple table selectors
        table_selectors = [
            {'class': 'blue-window sortable'},
            {'class': re.compile('sortable')},
            {'class': 'wikitable'},
            {'class': 'article-table'},
            {'class': 'mw-datatable'}
        ]
        
        table = None
        for selector in table_selectors:
            table = soup.find('table', selector)
            if table:
                logger.info(f"Found table with selector: {selector}")
                break
        
        if not table:
            # Try finding any table in the main content
            content = soup.find('div', {'class': 'mw-parser-output'})
            if content:
                table = content.find('table')
        
        if not table:
            logger.error("Could not find gods table")
            return god_links
        
        # Extract gods from table rows
        for row in table.find_all('tr')[1:]:  # Skip header
            cells = row.find_all(['td', 'th'])
            
            # Look for god link in any cell
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
                        break  # Found god in this row, move to next
        
        logger.info(f"Found {len(god_links)} gods")
        return god_links
    
    def _scrape_god(self, display_name: str, url: str) -> Optional[God]:
        """Scrape individual god data"""
        soup = self._get_soup(url)
        if not soup:
            return None
        
        try:
            # Extract god info from infobox
            infobox = soup.find('aside', {'class': 'portable-infobox'})
            if not infobox:
                logger.warning(f"No infobox found for {display_name}")
                return None
            
            # Extract basic info
            title = self._extract_infobox_value(infobox, 'Title')
            pantheon = self._extract_infobox_value(infobox, 'Pantheon')
            class_name = self._extract_infobox_value(infobox, 'Class') or self._extract_infobox_value(infobox, 'Type')
            
            # Map class names to standard ones used in the app
            class_map = {
                'warrior': 'Warrior',
                'assassin': 'Assassin', 
                'hunter': 'Hunter',
                'mage': 'Mage',
                'guardian': 'Guardian'
            }
            class_name = class_map.get(class_name.lower(), class_name) if class_name else ''
            
            # Extract roles (may be under "Role" or "Roles")
            roles_text = self._extract_infobox_value(infobox, 'Role') or self._extract_infobox_value(infobox, 'Roles')
            if roles_text:
                # Clean up role text and split
                roles_text = roles_text.replace(' and ', ', ').replace(' or ', ', ')
                roles = [r.strip() for r in roles_text.split(',') if r.strip()]
            else:
                # Default roles based on class
                default_roles = {
                    'Warrior': ['Solo'],
                    'Assassin': ['Jungle'],
                    'Hunter': ['ADC'],
                    'Mage': ['Mid'],
                    'Guardian': ['Support']
                }
                roles = default_roles.get(class_name, [])
            
            # Download god image
            image_url = self._extract_god_image(infobox)
            normalized_name = self._slugify(display_name)
            image_filename = f"{normalized_name}.png"
            image_path = self._download_image(image_url, image_filename, 'gods')
            
            # Extract abilities (just names for now, details handled by AbilitiesScraper)
            abilities = self._extract_ability_names(soup)
            
            return God(
                name=normalized_name,
                display_name=display_name,
                class_name=class_name,
                pantheon=pantheon,
                title=title,
                roles=roles,
                abilities=abilities,
                image_path=image_path
            )
            
        except Exception as e:
            logger.error(f"Error scraping god {display_name}: {e}")
            return None
    
    def _extract_infobox_value(self, infobox: BeautifulSoup, label: str) -> str:
        """Extract a value from the infobox by label"""
        for section in infobox.find_all('div', {'class': 'pi-item'}):
            label_elem = section.find('h3', {'class': 'pi-data-label'})
            if label_elem and label.lower() in label_elem.text.lower():
                value_elem = section.find('div', {'class': 'pi-data-value'})
                if value_elem:
                    return value_elem.text.strip()
        return ""
    
    def _extract_god_image(self, infobox: BeautifulSoup) -> str:
        """Extract god portrait image URL"""
        image_elem = infobox.find('img')
        if image_elem and image_elem.get('src'):
            return image_elem['src']
        return ""
    
    def _extract_ability_names(self, soup: BeautifulSoup) -> List[str]:
        """Extract ability names for the god"""
        abilities = []
        
        # Look for abilities section - try multiple approaches
        abilities_section = None
        
        # Method 1: Look for Abilities header
        for header_id in ['Abilities', 'Skills', 'Ability']:
            header = soup.find('span', {'id': header_id})
            if header:
                abilities_section = header.parent
                break
        
        # Method 2: Look for heading with "Abilities" text
        if not abilities_section:
            for h_tag in ['h2', 'h3']:
                header = soup.find(h_tag, text=re.compile('Abilities', re.IGNORECASE))
                if header:
                    abilities_section = header
                    break
        
        if not abilities_section:
            return abilities
        
        # Look for ability information in various formats
        # Method 1: Look for subsections after abilities header
        current = abilities_section
        ability_count = 0
        max_abilities = 5  # Passive + 4 abilities
        
        while current and ability_count < max_abilities:
            current = current.find_next_sibling()
            if not current:
                break
                
            # Stop at next major section
            if current.name in ['h2'] and current != abilities_section:
                break
            
            # Look for ability subsections (h3, h4, etc)
            if current.name in ['h3', 'h4']:
                ability_text = current.get_text(strip=True)
                # Skip "Notes" or other non-ability headers
                if ability_text and not any(skip in ability_text.lower() for skip in ['notes', 'video', 'trivia', 'skins']):
                    # Extract ability name (remove ability type prefix if present)
                    ability_name = re.sub(r'^(Passive|Ability \d|Ultimate)\s*[-:]?\s*', '', ability_text, flags=re.IGNORECASE)
                    if ability_name:
                        abilities.append(self._slugify(ability_name))
                        ability_count += 1
        
        # Method 2: Look for gallery items with ability names
        if len(abilities) < 3:
            gallery = soup.find('div', {'class': re.compile('wikia-gallery|gallery')})
            if gallery:
                for item in gallery.find_all(['div', 'figure']):
                    caption = item.find(['figcaption', 'div'], {'class': re.compile('caption|lightbox-caption')})
                    if caption:
                        ability_name = caption.get_text(strip=True)
                        if ability_name and self._slugify(ability_name) not in abilities:
                            abilities.append(self._slugify(ability_name))
        
        return abilities[:5]  # Return at most 5 abilities


class AbilitiesScraper(BaseScraper):
    """Scraper for abilities data"""
    
    def __init__(self, gods_data: Dict[str, God], delay: float = 0.1):
        super().__init__(delay)
        self.gods_data = gods_data
    
    def scrape(self) -> Dict[str, List[Ability]]:
        """Scrape all abilities data"""
        abilities_by_god = {}
        
        for god_name, god in self.gods_data.items():
            logger.info(f"Scraping abilities for {god.display_name}")
            god_url = urljoin(self.BASE_URL, f"/wiki/{god.display_name.replace(' ', '_')}")
            abilities = self._scrape_god_abilities(god_name, god_url)
            if abilities:
                abilities_by_god[god_name] = abilities
        
        return abilities_by_god
    
    def _scrape_god_abilities(self, god_name: str, url: str) -> List[Ability]:
        """Scrape abilities for a specific god"""
        soup = self._get_soup(url)
        if not soup:
            return []
        
        abilities = []
        ability_types = ['passive', 'ability1', 'ability2', 'ability3', 'ultimate']
        
        # Find abilities section
        abilities_section = None
        
        # Try different methods to find abilities section
        for header_id in ['Abilities', 'Skills', 'Ability']:
            header = soup.find('span', {'id': header_id})
            if header:
                abilities_section = header.parent
                break
        
        if not abilities_section:
            for h_tag in ['h2', 'h3']:
                header = soup.find(h_tag, text=re.compile('Abilities', re.IGNORECASE))
                if header:
                    abilities_section = header
                    break
        
        if not abilities_section:
            return abilities
        
        # Extract abilities by looking for subsections
        current = abilities_section
        ability_index = 0
        
        while current and ability_index < len(ability_types):
            current = current.find_next_sibling()
            if not current:
                break
            
            # Stop at next major section
            if current.name == 'h2' and current != abilities_section:
                break
            
            # Look for ability headers (h3, h4)
            if current.name in ['h3', 'h4']:
                ability_header_text = current.get_text(strip=True)
                
                # Skip non-ability headers
                if any(skip in ability_header_text.lower() for skip in ['notes', 'video', 'trivia', 'skins', 'lore', 'achievements']):
                    continue
                
                # Extract ability name
                ability_name = re.sub(r'^(Passive|Ability \d|Ultimate)\s*[-:]?\s*', '', ability_header_text, flags=re.IGNORECASE)
                if not ability_name:
                    continue
                
                # Get ability type
                ability_type = ability_types[ability_index] if ability_index < len(ability_types) else 'ability'
                
                # Look for ability details in following elements
                description = ""
                details = {}
                image_url = ""
                
                # Search for description and image in next siblings
                detail_elem = current
                for _ in range(10):  # Look at next 10 siblings max
                    detail_elem = detail_elem.find_next_sibling()
                    if not detail_elem:
                        break
                    
                    # Stop at next ability header
                    if detail_elem.name in ['h3', 'h4']:
                        break
                    
                    # Extract description from paragraphs or divs
                    if detail_elem.name in ['p', 'div'] and not description:
                        desc_text = detail_elem.get_text(strip=True)
                        if desc_text and len(desc_text) > 20:  # Meaningful description
                            description = desc_text
                    
                    # Look for ability image
                    if not image_url:
                        img = detail_elem.find('img')
                        if img and img.get('src'):
                            img_src = img['src']
                            # Check if it's an ability icon (not a general image)
                            if any(pattern in img_src.lower() for pattern in ['icon', 'ability', ability_name.lower()[:4]]):
                                image_url = img_src
                    
                    # Extract stats from lists or tables
                    if detail_elem.name == 'ul':
                        for li in detail_elem.find_all('li'):
                            stat_text = li.get_text(strip=True)
                            # Parse common stat patterns
                            for stat_type in ['Damage', 'Cooldown', 'Cost', 'Range', 'Radius', 'Duration', 'Heal']:
                                if stat_type in stat_text:
                                    details[stat_type.lower()] = stat_text
                
                # If no description found, use the header's title attribute or next text
                if not description and current.get('title'):
                    description = current['title']
                
                # Create ability object
                normalized_name = self._slugify(ability_name)
                image_filename = f"{god_name}_{normalized_name}.png"
                image_path = self._download_image(image_url, image_filename, 'abilities') if image_url else ""
                
                abilities.append(Ability(
                    name=normalized_name,
                    display_name=ability_name,
                    god=god_name,
                    type=ability_type,
                    description=description or f"{ability_name} ability",
                    details=details,
                    image_path=image_path
                ))
                
                ability_index += 1
        
        return abilities
    
    def _extract_ability_details(self, cell: BeautifulSoup) -> Dict[str, Any]:
        """Extract detailed stats from ability description"""
        details = {}
        
        # Look for common patterns like "Damage: X", "Cooldown: Y"
        text = cell.text
        patterns = {
            'damage': r'Damage:\s*([^\n]+)',
            'cooldown': r'Cooldown:\s*([^\n]+)',
            'cost': r'Cost:\s*([^\n]+)',
            'radius': r'Radius:\s*([^\n]+)',
            'range': r'Range:\s*([^\n]+)',
            'duration': r'Duration:\s*([^\n]+)',
        }
        
        for key, pattern in patterns.items():
            match = re.search(pattern, text, re.IGNORECASE)
            if match:
                details[key] = match.group(1).strip()
        
        return details


class ItemsScraper(BaseScraper):
    """Scraper for items data"""
    
    def scrape(self) -> Dict[str, Item]:
        """Scrape all items data"""
        items_url = urljoin(self.BASE_URL, "/wiki/Items")
        soup = self._get_soup(items_url)
        if not soup:
            return {}
        
        items = {}
        
        # Scrape different item categories
        categories = [
            ('Starter_Items', 'starter', 0),
            ('Consumables', 'consumable', 0),
            ('Relics', 'relic', 0),
            ('Tier_1_Items', 'offensive', 1),
            ('Tier_2_Items', 'offensive', 2),
            ('Tier_3_Items', 'offensive', 3),
            ('Glyph_Items', 'glyph', 3),
            ('Evolved_Items', 'evolved', 4)
        ]
        
        # First, scrape relics from the dedicated relics page
        logger.info("Scraping relics from dedicated page")
        relics = self._scrape_relics()
        items.update(relics)
        
        for section_id, category, tier in categories:
            logger.info(f"Scraping {section_id}")
            section_items = self._scrape_item_section(soup, section_id, category, tier)
            items.update(section_items)
        
        return items
    
    def _scrape_item_section(self, soup: BeautifulSoup, section_id: str, category: str, tier: int) -> Dict[str, Item]:
        """Scrape items from a specific section"""
        items = {}
        
        # Try different ways to find the section
        section_patterns = [
            section_id,
            section_id.replace('_', ' '),
            section_id.replace('_', ''),
        ]
        
        header = None
        for pattern in section_patterns:
            # Try span with id
            header = soup.find('span', {'id': pattern})
            if header:
                break
            
            # Try heading with text
            for heading_tag in ['h2', 'h3', 'h4']:
                header = soup.find(heading_tag, text=re.compile(pattern, re.IGNORECASE))
                if header:
                    break
            
            if header:
                break
        
        if not header:
            logger.warning(f"Section {section_id} not found, trying alternative approach")
            
            # Alternative: look for text containing the section name
            text_elements = soup.find_all(text=re.compile(section_id.replace('_', ' '), re.IGNORECASE))
            for elem in text_elements:
                if elem.parent.name in ['h2', 'h3', 'h4', 'span']:
                    header = elem.parent
                    break
        
        if not header:
            return items
        
        # Find the items container after the header
        current = header
        items_found = False
        
        # Look for the next element that contains item links
        for _ in range(10):  # Limit search depth
            current = current.find_next_sibling() if hasattr(current, 'find_next_sibling') else None
            if not current:
                current = header.parent
                if hasattr(current, 'find_next_sibling'):
                    current = current.find_next_sibling()
            
            if current:
                # Check if this element contains item links
                links = current.find_all('a', href=re.compile(r'/wiki/[^:]+$'))
                if links:
                    for link in links:
                        href = link['href']
                        if any(skip in href.lower() for skip in ['list_of', 'category:', 'template:', 'file:', '#']):
                            continue
                        
                        display_name = link.get('title', link.text).strip()
                        display_name = display_name.replace(' (page does not exist)', '')
                        
                        if display_name:
                            url = urljoin(self.BASE_URL, href)
                            logger.info(f"Scraping item: {display_name}")
                            item_data = self._scrape_item(display_name, url, category, tier)
                            if item_data:
                                items[item_data.name] = item_data
                            items_found = True
                
                # Stop if we found items or hit another section
                if items_found or (current.name in ['h2', 'h3'] and current != header):
                    break
        
        return items
    
    def _extract_item_links(self, element: BeautifulSoup) -> Dict[str, str]:
        """Extract item links from a gallery or list"""
        links = {}
        
        # Look for links with item names
        for link in element.find_all('a'):
            if link.get('href') and '/wiki/' in link['href']:
                # Get the title or text
                title = link.get('title', '').replace(' (page does not exist)', '')
                if not title:
                    title = link.text.strip()
                
                if title and not title.startswith('File:'):
                    links[title] = urljoin(self.BASE_URL, link['href'])
        
        return links
    
    def _scrape_item(self, display_name: str, url: str, category: str, tier: int) -> Optional[Item]:
        """Scrape individual item data"""
        soup = self._get_soup(url)
        if not soup:
            return None
        
        try:
            # Find infobox
            infobox = soup.find('aside', {'class': 'portable-infobox'})
            if not infobox:
                logger.warning(f"No infobox found for {display_name}")
                return None
            
            # Extract basic info
            cost = self._extract_cost(infobox, 'Cost')
            
            # Extract stats as list of strings
            stats = self._extract_item_stats(soup)
            
            # Extract effects (passive/active)
            effects = []
            passive = self._extract_passive(soup)
            active = self._extract_active(soup)
            if passive:
                effects.append(passive)
            if active:
                effects.append(active)
            
            # Determine tags based on category and tier
            tags = self._determine_item_tags(display_name, category, tier)
            
            # Download item image
            image_url = self._extract_item_image(infobox)
            normalized_name = self._slugify(display_name)
            image_filename = f"{normalized_name}.png"
            image_path = self._download_image(image_url, image_filename, 'items')
            
            return Item(
                name=normalized_name,
                display_name=display_name,
                price=cost,
                stats=stats,
                effects=effects,
                tags=tags,
                image_path=image_path
            )
            
        except Exception as e:
            logger.error(f"Error scraping item {display_name}: {e}")
            return None
    
    def _extract_cost(self, infobox: BeautifulSoup, label: str) -> int:
        """Extract cost value from infobox"""
        value = self._extract_infobox_value(infobox, label)
        # Extract numeric value
        match = re.search(r'\d+', value)
        return int(match.group()) if match else 0
    
    def _extract_infobox_value(self, infobox: BeautifulSoup, label: str) -> str:
        """Extract a value from the infobox by label"""
        for section in infobox.find_all('div', {'class': 'pi-item'}):
            label_elem = section.find('h3', {'class': 'pi-data-label'})
            if label_elem and label.lower() in label_elem.text.lower():
                value_elem = section.find('div', {'class': 'pi-data-value'})
                if value_elem:
                    return value_elem.text.strip()
        return ""
    
    def _extract_item_image(self, infobox: BeautifulSoup) -> str:
        """Extract item image URL"""
        image_elem = infobox.find('img')
        if image_elem and image_elem.get('src'):
            return image_elem['src']
        return ""
    
    def _extract_item_stats(self, soup: BeautifulSoup) -> Dict[str, Any]:
        """Extract item stats"""
        stats = {}
        
        # Look for stats in the page content
        # This is simplified - you may need to adjust based on actual page structure
        stat_patterns = {
            'physical_power': r'Physical Power[:\s]+([+\-]?\d+)',
            'magical_power': r'Magical Power[:\s]+([+\-]?\d+)',
            'health': r'Health[:\s]+([+\-]?\d+)',
            'mana': r'Mana[:\s]+([+\-]?\d+)',
            'attack_speed': r'Attack Speed[:\s]+([+\-]?\d+%?)',
            'critical_chance': r'Critical Strike Chance[:\s]+([+\-]?\d+%?)',
            'movement_speed': r'Movement Speed[:\s]+([+\-]?\d+%?)',
            'cooldown_reduction': r'Cooldown Reduction[:\s]+([+\-]?\d+%?)',
            'penetration': r'Penetration[:\s]+([+\-]?\d+)',
            'lifesteal': r'Lifesteal[:\s]+([+\-]?\d+%?)',
        }
        
        content_text = soup.get_text()
        for stat_name, pattern in stat_patterns.items():
            match = re.search(pattern, content_text, re.IGNORECASE)
            if match:
                stats[stat_name] = match.group(1)
        
        return stats
    
    def _extract_passive(self, soup: BeautifulSoup) -> Optional[str]:
        """Extract passive description"""
        # Method 1: Look in infobox for passive field
        infobox = soup.find('aside', {'class': 'portable-infobox'})
        if infobox:
            passive_value = self._extract_infobox_value(infobox, 'Passive')
            if passive_value:
                return passive_value
        
        # Method 2: Look for passive section in content
        content = soup.find('div', {'class': 'mw-parser-output'})
        if content:
            # Look for passive header
            for header in content.find_all(['h2', 'h3', 'h4', 'b', 'strong']):
                if 'passive' in header.get_text(strip=True).lower():
                    # Get description from next element
                    next_elem = header.find_next_sibling()
                    if next_elem:
                        # Collect text until next header
                        passive_text = []
                        while next_elem and next_elem.name not in ['h2', 'h3', 'h4']:
                            text = next_elem.get_text(strip=True)
                            if text:
                                passive_text.append(text)
                            next_elem = next_elem.find_next_sibling()
                            if len(passive_text) > 0 and (not next_elem or next_elem.name in ['h2', 'h3', 'h4']):
                                break
                        if passive_text:
                            return ' '.join(passive_text)
        
        return None
    
    def _determine_item_tags(self, display_name: str, category: str, tier: int) -> List[str]:
        """Determine tags for an item based on its name and metadata"""
        tags = []
        display_lower = display_name.lower()
        
        # Relics
        relic_keywords = ['aegis', 'blink', 'beads', 'shell', 'ankh', 'heavenly', 'horrific', 
                          'meditation', 'phantom', 'teleport', 'thorns', 'sundering', 'bracer',
                          'divine barrier', 'cloak']
        if any(keyword in display_lower for keyword in relic_keywords) or category == 'relic':
            tags.append('Relic')
            if 'greater' in display_lower:
                tags.append('Tier2')
            elif any(x in display_lower for x in ['acceleration', 'brilliance', 'ascetic', 'peril', 
                                                  'entangling', 'persistent', 'scorching', 'blast', 
                                                  'temporal', 'judgement']):
                tags.append('Tier3')
            else:
                tags.append('Tier1')
        
        # Consumables
        elif category == 'consumable' or any(word in display_lower for word in ['potion', 'elixir', 'ward', 'chalice', 'brew']):
            tags.append('Consumable')
        
        # Starter items
        elif category == 'starter' or any(word in display_lower for word in ['starter', 'mask', 'eye of the jungle', 
                                                                             'bluestone pendant', "warrior's axe",
                                                                             "bumba's", 'manikin', 'sands of time',
                                                                             'conduit gem', "lono's"]):
            tags.append('Starter')
            if any(word in display_lower for word in ['spear', 'blade', 'pendulum', 'brooch']):
                tags.append('Evolved')
        
        # Evolved items
        elif category == 'evolved' or any(suffix in display_lower for suffix in ['evolved', 'perfected', 'malicious', 'envenomed']):
            tags.append('Evolved')
            tags.append('Tier3')
        
        # Glyph items
        elif 'glyph' in display_lower or category == 'glyph':
            tags.append('Glyph')
            tags.append('Tier4')
        
        # Regular items by tier
        else:
            if tier == 1:
                tags.append('Tier1')
            elif tier == 2:
                tags.append('Tier2')
            elif tier >= 3:
                tags.append('Tier3')
        
        return tags
    
    def _scrape_relics(self) -> Dict[str, Item]:
        """Scrape relics from the dedicated relics page"""
        relics = {}
        relics_url = urljoin(self.BASE_URL, "/wiki/Relics")
        soup = self._get_soup(relics_url)
        if not soup:
            return relics
        
        # Find all relic links
        content = soup.find('div', {'class': 'mw-parser-output'})
        if not content:
            return relics
        
        # Look for relic sections (Tier 1, Tier 2, Tier 3 Relics)
        for tier_num in [1, 2, 3]:
            tier_header = None
            # Find tier header
            for header in content.find_all(['h2', 'h3']):
                header_text = header.get_text(strip=True)
                if f'Tier {tier_num}' in header_text and 'Relic' in header_text:
                    tier_header = header
                    break
            
            if not tier_header:
                continue
            
            # Find relics in this tier section
            current = tier_header
            relics_found = False
            
            for _ in range(20):  # Check next 20 elements
                current = current.find_next_sibling()
                if not current:
                    break
                
                # Stop at next major section
                if current.name in ['h2', 'h3']:
                    break
                
                # Look for relic links
                links = current.find_all('a', href=re.compile(r'/wiki/[^:]+$'))
                if links:
                    for link in links:
                        href = link['href']
                        # Skip non-relic pages
                        if any(skip in href.lower() for skip in ['category:', 'template:', 'file:', '#', 'relic_']):
                            continue
                        
                        display_name = link.get('title', link.text).strip()
                        if display_name and display_name not in ['Relics', 'Relic']:
                            url = urljoin(self.BASE_URL, href)
                            logger.info(f"Scraping relic: {display_name}")
                            
                            # Use special handling for relics
                            item_data = self._scrape_relic_item(display_name, url, tier_num)
                            if item_data:
                                relics[item_data.name] = item_data
                            relics_found = True
                
                if relics_found:
                    break
        
        return relics
    
    def _scrape_relic_item(self, display_name: str, url: str, tier: int) -> Optional[Item]:
        """Scrape individual relic with special handling for active effects"""
        soup = self._get_soup(url)
        if not soup:
            return None
        
        try:
            # For relics, most have 0 cost
            cost = 0
            stats = []  # Relics typically don't have stats
            
            # Extract active effect - this is the main feature of relics
            effects = []
            active_effect = self._extract_active(soup)
            if active_effect:
                effects.append(active_effect)
            
            # Determine tags
            tags = self._determine_item_tags(display_name, 'relic', tier)
            
            # Download item image
            infobox = soup.find('aside', {'class': 'portable-infobox'})
            image_url = self._extract_item_image(infobox) if infobox else ""
            normalized_name = self._slugify(display_name)
            image_filename = f"{normalized_name}.png"
            image_path = self._download_image(image_url, image_filename, 'items')
            
            return Item(
                name=normalized_name,
                display_name=display_name,
                price=cost,
                stats=stats,
                effects=effects,
                tags=tags,
                image_path=image_path
            )
            
        except Exception as e:
            logger.error(f"Error scraping relic {display_name}: {e}")
            return None
    
    def _extract_active(self, soup: BeautifulSoup) -> Optional[str]:
        """Extract active ability description"""
        # For relics, the active effect is usually in the main description
        # Method 1: Look in infobox
        infobox = soup.find('aside', {'class': 'portable-infobox'})
        if infobox:
            # Try to find active effect in infobox
            for section in infobox.find_all('section'):
                section_text = section.get_text(strip=True)
                if 'Active:' in section_text or 'Using this item' in section_text:
                    # Extract the active description
                    active_match = re.search(r'(Active:|Using this item)[^.]+\.(\s+[^.]+\.)*', section_text)
                    if active_match:
                        return active_match.group(0)
        
        # Method 2: Look in main content
        content = soup.find('div', {'class': 'mw-parser-output'})
        if content:
            # Look for active description in paragraphs
            for p in content.find_all('p'):
                p_text = p.get_text(strip=True)
                if 'Active:' in p_text or 'Using this item' in p_text:
                    return p_text
            
            # Look for active header
            for header in content.find_all(['h2', 'h3', 'h4', 'b', 'strong']):
                if 'active' in header.get_text(strip=True).lower():
                    # Get description from next element
                    next_elem = header.find_next_sibling()
                    if next_elem:
                        # Collect text until next header
                        active_text = []
                        while next_elem and next_elem.name not in ['h2', 'h3', 'h4']:
                            text = next_elem.get_text(strip=True)
                            if text:
                                active_text.append(text)
                            next_elem = next_elem.find_next_sibling()
                            if len(active_text) > 0 and (not next_elem or next_elem.name in ['h2', 'h3', 'h4']):
                                break
                        if active_text:
                            return ' '.join(active_text)
        
        return None


class UnifiedScraper:
    """Main scraper orchestrator"""
    
    def __init__(self, delay: float = 0.1):
        self.delay = delay
        self.data_dir = Path("../src/data/json")
        self.data_dir.mkdir(parents=True, exist_ok=True)
    
    def scrape_all(self):
        """Scrape all data"""
        logger.info("Starting unified scraping process")
        
        # Track metadata
        metadata = {
            'scrape_date': datetime.now().isoformat(),
            'source': 'https://smite.fandom.com',
            'version': '1.0.0'
        }
        
        # Scrape gods
        logger.info("Scraping gods...")
        gods_scraper = GodsScraper(self.delay)
        gods_data = gods_scraper.scrape()
        self._save_data(gods_data, 'gods.json')
        
        # Scrape abilities
        logger.info("Scraping abilities...")
        abilities_scraper = AbilitiesScraper(gods_data, self.delay)
        abilities_data = abilities_scraper.scrape()
        self._save_data(abilities_data, 'abilities.json')
        
        # Scrape items
        logger.info("Scraping items...")
        items_scraper = ItemsScraper(self.delay)
        items_data = items_scraper.scrape()
        self._save_data(items_data, 'items.json')
        
        # Save metadata
        self._save_json(metadata, 'metadata.json')
        
        logger.info("Scraping complete!")
        self._print_summary(gods_data, abilities_data, items_data)
    
    def scrape_category(self, category: str):
        """Scrape a specific category"""
        if category == 'gods':
            scraper = GodsScraper(self.delay)
            data = scraper.scrape()
            self._save_data(data, 'gods.json')
        elif category == 'abilities':
            # Need gods data first
            gods_file = self.data_dir / 'gods.json'
            if not gods_file.exists():
                logger.error("Gods data not found. Run 'gods' scraper first.")
                return
            
            with open(gods_file) as f:
                gods_dict = json.load(f)
                gods_data = {k: God(**v) for k, v in gods_dict.items()}
            
            scraper = AbilitiesScraper(gods_data, self.delay)
            data = scraper.scrape()
            self._save_data(data, 'abilities.json')
        elif category == 'items':
            scraper = ItemsScraper(self.delay)
            data = scraper.scrape()
            self._save_data(data, 'items.json')
        else:
            logger.error(f"Unknown category: {category}")
    
    def _save_data(self, data: Dict, filename: str):
        """Save scraped data"""
        # Convert dataclasses to dicts
        if data and isinstance(next(iter(data.values())), (God, Item)):
            data = {k: asdict(v) for k, v in data.items()}
        elif data and isinstance(next(iter(data.values())), list):
            # For abilities (list of dataclasses)
            data = {k: [asdict(ability) for ability in v] for k, v in data.items()}
        
        filepath = self.data_dir / filename
        with open(filepath, 'w', encoding='utf-8') as f:
            json.dump(data, f, indent=2, ensure_ascii=False)
        logger.info(f"Saved {filename}")
    
    def _save_json(self, data: Any, filename: str):
        """Save data as JSON"""
        filepath = self.data_dir / filename
        with open(filepath, 'w', encoding='utf-8') as f:
            json.dump(data, f, indent=2, ensure_ascii=False)
    
    def _print_summary(self, gods_data: Dict, abilities_data: Dict, items_data: Dict):
        """Print scraping summary"""
        print("\n" + "="*50)
        print("SCRAPING SUMMARY")
        print("="*50)
        print(f"Gods scraped: {len(gods_data)}")
        print(f"Gods with abilities: {len(abilities_data)}")
        total_abilities = sum(len(abilities) for abilities in abilities_data.values())
        print(f"Total abilities: {total_abilities}")
        print(f"Items scraped: {len(items_data)}")
        print(f"\nOutput directory: {self.data_dir.absolute()}")
        print("="*50)


def main():
    """Main entry point"""
    parser = argparse.ArgumentParser(
        description="Unified Smite Wiki Scraper",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python unified_scraper.py --all              # Scrape everything
  python unified_scraper.py --category gods    # Scrape only gods
  python unified_scraper.py --category items   # Scrape only items
  python unified_scraper.py --delay 1.0        # Use 1 second delay
        """
    )
    
    parser.add_argument(
        '--category',
        choices=['gods', 'abilities', 'items'],
        help='Specific category to scrape'
    )
    parser.add_argument(
        '--all',
        action='store_true',
        help='Scrape all categories'
    )
    parser.add_argument(
        '--delay',
        type=float,
        default=0.1,
        help='Delay between requests in seconds (default: 0.5)'
    )
    parser.add_argument(
        '--verbose',
        action='store_true',
        help='Enable verbose logging'
    )
    
    args = parser.parse_args()
    
    if args.verbose:
        logging.getLogger().setLevel(logging.DEBUG)
    
    if not args.all and not args.category:
        parser.error('Either --all or --category must be specified')
    
    scraper = UnifiedScraper(args.delay)
    
    if args.all:
        scraper.scrape_all()
    else:
        scraper.scrape_category(args.category)


if __name__ == '__main__':
    main()