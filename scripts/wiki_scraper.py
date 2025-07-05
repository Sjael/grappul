import requests
from bs4 import BeautifulSoup, Tag, NavigableString
import json
import os
import time
import re
from typing import Dict, List, Optional, Union, Any
import urllib.parse

class SmiteWikiScraper:
    def __init__(self):
        self.base_url = "https://smite.fandom.com"
        self.session = requests.Session()
        self.session.headers.update({
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36'
        })
        
        # Create output directories
        self.gods_dir = "output/gods"
        self.abilities_dir = "output/abilities"
        os.makedirs(self.gods_dir, exist_ok=True)
        os.makedirs(self.abilities_dir, exist_ok=True)

    def get_soup(self, url: str) -> BeautifulSoup:
        """Get BeautifulSoup object from URL"""
        response = self.session.get(url)
        return BeautifulSoup(response.text, 'html.parser')

    def download_image(self, url: str, save_path: str) -> bool:
        """Download image from URL and save to path"""
        try:
            response = self.session.get(url)
            if response.status_code == 200:
                with open(save_path, 'wb') as f:
                    f.write(response.content)
                return True
        except Exception as e:
            print(f"Error downloading {url}: {e}")
        return False

    def clean_image_url(self, url: Optional[str]) -> Optional[str]:
        """Clean up image URL by removing revision parameters"""
        if not url or not isinstance(url, str):
            return None
        parts = url.split('/revision')
        return parts[0] if parts else None

    def get_tag_attribute(self, tag: Tag, attr: str) -> Optional[str]:
        """Safely get a tag attribute that might be a string or list"""
        value = tag.get(attr)
        if isinstance(value, list):
            return value[0] if value else None
        return value if isinstance(value, str) else None

    def scrape_gods(self):
        """Scrape god names and images from the List of Gods page"""
        print("Fetching god list...")
        soup = self.get_soup(f"{self.base_url}/wiki/Smite_Wiki")
        
        gods = []
        heroes_div = soup.find('div', {'class': 'mp-heroes'})
        if not heroes_div or not isinstance(heroes_div, Tag):
            print("Could not find heroes div")
            return
            
        # Find all god images
        god_images = heroes_div.find_all('img', {'class': 'mw-file-element'})
        if not god_images:
            print("Could not find any god images")
            return
            
        seen_gods = set()  # Track gods we've already processed
        
        for img in god_images:
            if not isinstance(img, Tag):
                continue
                
            # Get image URL and clean it
            img_url = self.get_tag_attribute(img, 'data-src') or self.get_tag_attribute(img, 'src')
            img_url = self.clean_image_url(img_url)
            if not img_url:
                continue
                
            # Extract god name from image name
            img_name = self.get_tag_attribute(img, 'data-image-name')
            if not img_name or 'Transparent' in img_name:  # Skip transparent icons
                continue
                
            # Convert "T Achilles Default Icon.png" to "Achilles"
            god_name = img_name.replace('T ', '').replace(' Default Icon.png', '').strip()
            if not god_name or god_name in seen_gods:  # Skip duplicates
                continue
                
                # text-shadow: -1px 0 0.2em black, 0 1px 0.2em black, 1px 0 0.2em black, 0 -1px 0.2em black;
            seen_gods.add(god_name)
            
            # Download god image
            img_filename = f"{god_name.lower().replace(' ', '_')}.png"
            img_path = os.path.join(self.gods_dir, img_filename)
            
            if self.download_image(img_url, img_path):
                print(f"Downloaded image for {god_name}")
                gods.append(god_name)
            else:
                print(f"Failed to download image for {god_name}")
                
        # Save list of gods
        with open('output/gods_short.json', 'w') as f:
            json.dump(gods, f, indent=2)
            
        print(f"Successfully scraped {len(gods)} gods")

    def scrape_abilities(self):
        """Scrape abilities for all gods"""
        print("Loading god list...")
        try:
            with open('output/gods_short.json', 'r') as f:
                gods = json.load(f)
        except FileNotFoundError:
            print("gods_short.json not found. Run scrape_gods() first.")
            return

        abilities_data = {}
        
        for god in gods:
            print(f"Scraping abilities for {god}...")
            god_url = f"{self.base_url}/wiki/{god.replace(' ', '_')}"
            soup = self.get_soup(god_url)
            
            # Find abilities section
            abilities_h2 = soup.find('span', {'id': 'Abilities'})
            if not abilities_h2 or not isinstance(abilities_h2, Tag):
                print(f"Could not find abilities section for {god}")
                continue
                
            abilities_section = abilities_h2.parent
            if not abilities_section or not isinstance(abilities_section, Tag):
                continue
                
            # Find the main abilities table and then get the detail tables
            next_tables = abilities_section.find_next_siblings('table')
            if not next_tables:
                print(f"Could not find ability tables for {god}")
                continue
                
            ability_tables = next_tables[0]
            if not isinstance(ability_tables, Tag):
                continue
                
            detail_tables = ability_tables.find_all('table', {'class': 'wikitable'})
            if not detail_tables:
                print(f"Could not find ability details for {god}")
                continue
            
            god_abilities = []
            for idx, table in enumerate(detail_tables, 1):
                if not isinstance(table, Tag):
                    continue
                    
                ability_data = {}
                
                # Get ability name and description
                name_cell = table.find('td', style='font-size:110%')
                desc_cell = table.find('td', style='padding:10px')
                
                if not name_cell or not desc_cell or not isinstance(name_cell, Tag) or not isinstance(desc_cell, Tag):
                    continue
                    
                ability_name = name_cell.get_text(strip=True)
                ability_desc = desc_cell.get_text(strip=True)
                
                # Get ability image
                img_tag = table.find('img')
                if img_tag and isinstance(img_tag, Tag):
                    img_url = self.get_tag_attribute(img_tag, 'data-src') or self.get_tag_attribute(img_tag, 'src')
                    img_url = self.clean_image_url(img_url)
                    if img_url:
                        img_filename = f"{god.lower().replace(' ', '_')}_{idx}.png"
                        img_path = os.path.join(self.abilities_dir, img_filename)
                        
                        if self.download_image(img_url, img_path):
                            print(f"Downloaded ability image {idx} for {god}")
                            ability_data['image'] = img_filename
                
                # Get ability details
                details = []
                detail_cells = table.find_all('td', style='padding-left:10px')
                for cell in detail_cells:
                    if isinstance(cell, Tag):
                        detail_text = cell.get_text(strip=True)
                        if detail_text:
                            details.append(detail_text)
                
                ability_data.update({
                    'name': ability_name,
                    'description': ability_desc,
                    'details': details
                })
                
                god_abilities.append(ability_data)
            
            if god_abilities:
                abilities_data[god] = god_abilities
                
        # Save abilities data
        with open('output/abilities.json', 'w') as f:
            json.dump(abilities_data, f, indent=2)
            
        print("Finished scraping abilities")

def main():
    scraper = SmiteWikiScraper()
    
    if len(sys.argv) < 2:
        print("Please specify a command: gods or abilities")
        return
        
    command = sys.argv[1].lower()
    
    if command == "gods":
        scraper.scrape_gods()
    elif command == "abilities":
        scraper.scrape_abilities()
    else:
        print("Invalid command. Use 'gods' or 'abilities'")

if __name__ == "__main__":
    import sys
    main() 
