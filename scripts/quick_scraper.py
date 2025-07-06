#!/usr/bin/env python3
"""
Quick scraper to get basic god and item data from Smite Wiki
Works with the current wiki structure
"""

import json
import os
import re
import time
from pathlib import Path
from typing import Dict, List, Any

import requests
from bs4 import BeautifulSoup


class QuickScraper:
    def __init__(self):
        self.session = requests.Session()
        self.session.headers.update({
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36'
        })
        self.base_url = "https://smite.fandom.com"
        self.output_dir = Path("output")
        self.output_dir.mkdir(exist_ok=True)
    
    def scrape_gods_from_table(self):
        """Scrape basic god info from the list page"""
        url = f"{self.base_url}/wiki/List_of_gods"
        response = self.session.get(url)
        soup = BeautifulSoup(response.content, 'html.parser')
        
        # Find the main table
        table = soup.find('table', {'class': 'blue-window sortable'})
        if not table:
            print("Could not find gods table")
            return {}
        
        gods = {}
        rows = table.find_all('tr')[1:]  # Skip header
        
        for row in rows:
            cells = row.find_all(['td', 'th'])
            if len(cells) >= 5:  # We need at least 5 columns
                # Extract data from columns
                # Column 0: Icon
                # Column 1: God name
                # Column 2: Pantheon
                # Column 3: Attack Type
                # Column 4: Power Type
                # Column 5: Class
                
                god_link = cells[1].find('a')
                if not god_link:
                    continue
                
                name = god_link.text.strip()
                normalized_name = self._normalize_name(name)
                
                # Get pantheon
                pantheon = cells[2].text.strip()
                
                # Get class (column 5)
                class_name = cells[5].text.strip() if len(cells) > 5 else ""
                
                # Map to standard roles based on class
                role_map = {
                    'Warrior': ['Solo'],
                    'Assassin': ['Jungle'],
                    'Hunter': ['ADC', 'Carry'],
                    'Mage': ['Mid'],
                    'Guardian': ['Support']
                }
                roles = role_map.get(class_name, [])
                
                # Download icon if available
                icon_img = cells[0].find('img')
                image_path = ""
                if icon_img and icon_img.get('src'):
                    image_url = icon_img['src']
                    if not image_url.startswith('http'):
                        image_url = 'https:' + image_url
                    image_path = self._download_image(image_url, f"{normalized_name}.png", "gods")
                
                gods[normalized_name] = {
                    "name": normalized_name,
                    "display_name": name,
                    "class_name": class_name,
                    "pantheon": pantheon,
                    "title": "",  # Not available in table
                    "roles": roles,
                    "abilities": [],  # Would need to scrape individual pages
                    "image_path": image_path
                }
                
                print(f"Scraped {name} - {class_name} ({pantheon})")
        
        return gods
    
    def scrape_basic_items(self):
        """Scrape basic item info from the items page"""
        url = f"{self.base_url}/wiki/Items"
        response = self.session.get(url)
        soup = BeautifulSoup(response.content, 'html.parser')
        
        items = {}
        
        # Find all item links in the main content
        content = soup.find('div', {'class': 'mw-parser-output'})
        if not content:
            print("Could not find content div")
            return items
        
        # Look for item sections
        sections = [
            ('Consumables', 'consumable', 0),
            ('Relics', 'relic', 0),
            ('Starter items', 'starter', 0),
            ('Tier 1', 'passive', 1),
            ('Tier 2', 'passive', 2),
            ('Tier 3', 'passive', 3),
        ]
        
        for section_name, category, tier in sections:
            # Find header with this text
            header = None
            for h in content.find_all(['h2', 'h3', 'h4']):
                if section_name.lower() in h.text.lower():
                    header = h
                    break
            
            if not header:
                print(f"Could not find section: {section_name}")
                continue
            
            print(f"\nProcessing section: {section_name}")
            
            # Find items after this header
            current = header.find_next_sibling()
            items_found = 0
            
            while current and current.name not in ['h2', 'h3']:
                # Look for item links
                links = current.find_all('a', href=True)
                for link in links:
                    href = link['href']
                    if '/wiki/' in href and not any(skip in href for skip in ['Category:', 'Template:', 'File:', '#']):
                        item_name = link.text.strip() or link.get('title', '').strip()
                        if item_name and item_name not in ['Smite Wiki', 'Items']:
                            normalized_name = self._normalize_name(item_name)
                            
                            # Try to get image
                            img = link.find_previous('img') or link.find_next('img')
                            image_path = ""
                            if img and img.get('src'):
                                image_url = img['src']
                                if not image_url.startswith('http'):
                                    image_url = 'https:' + image_url
                                image_path = self._download_image(image_url, f"{normalized_name}.png", "items")
                            
                            items[normalized_name] = {
                                "name": normalized_name,
                                "display_name": item_name,
                                "category": category,
                                "tier": tier,
                                "cost": 0,
                                "total_cost": 0,
                                "stats": {},
                                "passive": None,
                                "active": None,
                                "image_path": image_path
                            }
                            items_found += 1
                            print(f"  Found item: {item_name}")
                
                current = current.find_next_sibling() if current else None
            
            print(f"  Total items in section: {items_found}")
        
        return items
    
    def _normalize_name(self, name: str) -> str:
        """Convert name to lowercase with underscores"""
        # Remove special characters and convert spaces to underscores
        name = re.sub(r'[^\w\s-]', '', name)
        name = re.sub(r'[-\s]+', '_', name)
        return name.lower()
    
    def _download_image(self, url: str, filename: str, subdir: str) -> str:
        """Download image and return relative path"""
        try:
            # Clean URL
            url = url.split('/revision/')[0]
            
            # Create subdirectory
            subdir_path = self.output_dir / subdir
            subdir_path.mkdir(exist_ok=True)
            
            # Download
            response = self.session.get(url, timeout=10)
            if response.status_code == 200:
                filepath = subdir_path / filename
                with open(filepath, 'wb') as f:
                    f.write(response.content)
                return f"{subdir}/{filename}"
        except Exception as e:
            print(f"Failed to download image from {url}: {e}")
        
        return ""
    
    def save_json(self, data: Any, filename: str):
        """Save data as JSON"""
        filepath = self.output_dir / filename
        with open(filepath, 'w', encoding='utf-8') as f:
            json.dump(data, f, indent=2, ensure_ascii=False)
        print(f"\nSaved {filename} with {len(data)} entries")
    
    def run(self):
        """Run the scraper"""
        print("Starting quick scrape...")
        
        # Scrape gods
        print("\n=== Scraping Gods ===")
        gods = self.scrape_gods_from_table()
        self.save_json(gods, "gods.json")
        
        # Scrape items
        print("\n=== Scraping Items ===")
        items = self.scrape_basic_items()
        self.save_json(items, "items.json")
        
        # Create empty abilities file
        self.save_json({}, "abilities.json")
        
        # Create metadata
        metadata = {
            "scrape_date": time.strftime("%Y-%m-%d %H:%M:%S"),
            "source": "https://smite.fandom.com",
            "note": "Basic scrape - wiki structure has changed"
        }
        self.save_json(metadata, "metadata.json")
        
        print("\n=== Scraping Complete ===")
        print(f"Gods: {len(gods)}")
        print(f"Items: {len(items)}")


if __name__ == '__main__':
    scraper = QuickScraper()
    scraper.run()