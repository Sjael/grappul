import json
import requests
from bs4 import BeautifulSoup, Tag
from typing import Dict, Optional, List
import time
import os
import urllib.parse

class ItemScraper:
    def __init__(self):
        self.base_url = "https://smite.fandom.com/wiki"
        self.image_dir = "output/items"
        # Create images directory if it doesn't exist
        os.makedirs(self.image_dir, exist_ok=True)
        
    def clean_text(self, text: str) -> str:
        """Clean text by removing extra whitespace and newlines."""
        return ' '.join(text.strip().split())
    
    def get_soup(self, url: str) -> BeautifulSoup:
        """Get BeautifulSoup object from URL with error handling."""
        response = requests.get(url)
        response.raise_for_status()
        return BeautifulSoup(response.text, 'html.parser')
    
    def format_item_url(self, item_name: str) -> str:
        """Format item name for URL."""
        return f"{self.base_url}/{item_name.replace(' ', '_')}"

    def scrape_item_names(self) -> Optional[Dict[str, List[str]]]:
        """Scrape all item names from the main items page."""
        try:
            url = f"{self.base_url}/Items"
            soup = self.get_soup(url)
            
            items = {
                'starter_items': [],
                'consumable_items': [],
                'relics': [],
                'tier1_items': [],
                'tier2_items': [],
                'tier3_items': [],
                'glyph_items': [],
                'evolved_items': []
            }
            
            # Find all item grids
            item_grids = soup.find_all('div', class_='items-overview-grid')
            
            for grid in item_grids:
                if not isinstance(grid, Tag):
                    continue
                
                # Find the section header
                header = grid.find_previous(['h3', 'h4'])
                if not header:
                    continue
                    
                section_title = header.get_text(strip=True)
                
                # Determine which section this belongs to
                current_section = None
                if "Starter items" in section_title:
                    current_section = 'starter_items'
                elif "Consumable items" in section_title:
                    current_section = 'consumable_items'
                elif "Relics" in section_title:
                    current_section = 'relics'
                elif "Tier 1" in section_title:
                    current_section = 'tier1_items'
                elif "Tier 2" in section_title:
                    current_section = 'tier2_items'
                elif "Tier 3" in section_title:
                    current_section = 'tier3_items'
                elif "Glyph" in section_title:
                    current_section = 'glyph_items'
                elif "Evolved" in section_title:
                    current_section = 'evolved_items'
                
                if current_section:
                    # Get all item links in the grid
                    item_links = grid.find_all('a')
                    for link in item_links:
                        if isinstance(link, Tag):
                            item_name = link.get_text(strip=True)
                            if item_name:
                                items[current_section].append(item_name)
            
            # Clean up item names
            def is_valid_item_name(name: str) -> bool:
                # Skip navigation items and other non-item text
                skip_words = ['Items', 'List', 'Main', 'About', 'Game', 'Modes', 'System', 'Cards', 
                            'Chest', 'Pass', 'DLC', 'Wiki', 'General', 'information', 'Gameplay',
                            'Voice', 'Guided', 'Patch', 'notes', 'Achievements', 'Accolades', 'Quest',
                            'Voices', 'Music', 'Lore', 'MOBA', 'Terms', 'Clan', 'Controls', 'VGS',
                            'Commands', 'Trading', 'Cosmetic', 'Treasure', 'Coupon', 'Gifting',
                            'Battle', 'Events', 'Crossovers', 'Esports', 'Live', 'Streams']
                return bool(name and 
                        not any(word in name for word in skip_words) and
                        not name.startswith('.') and 
                        not name.startswith('v') and
                        not any(c.isdigit() for c in name))
            
            # Clean up and deduplicate items
            for section in items:
                cleaned_items = []
                seen = set()
                for item in items[section]:
                    item = item.strip()
                    if is_valid_item_name(item) and item not in seen:
                        cleaned_items.append(item)
                        seen.add(item)
                items[section] = cleaned_items
            
            # Save results
            with open('output/full_items.json', 'w', encoding='utf-8') as f:
                json.dump(items, f, indent=2)
            
            return items
            
        except Exception as e:
            print(f"Error scraping item names: {str(e)}")
            return None
    
    def download_image(self, image_url: str, item_name: str) -> str:
        """Download image and return local path."""
        try:
            # Clean filename
            filename = item_name.lower().replace(' ', '_').replace("'", '') + '.png'
            local_path = os.path.join(self.image_dir, filename)
            
            # Download image
            response = requests.get(image_url)
            response.raise_for_status()
            
            with open(local_path, 'wb') as f:
                f.write(response.content)
                
            return filename
        except Exception as e:
            print(f"Error downloading image for {item_name}: {str(e)}")
            return ""
    
    def scrape_item_details(self, item_name: str) -> Optional[Dict]:
        """Scrape details for a specific item."""
        try:
            url = self.format_item_url(item_name)
            soup = self.get_soup(url)
            
            item = {
                'name': item_name,
                'cost': 0,
                'total_cost': 0,
                'stats': [],
                'passive': '',
                'active': '',
                'type': [],
                'image': ''
            }
            
            # Find and download the item image
            image_elem = soup.find('table', {'class': 'infobox'})
            if image_elem and isinstance(image_elem, Tag):
                # Try to find the image in noscript tag first
                noscript = image_elem.find('noscript')
                if noscript and isinstance(noscript, Tag):
                    img = noscript.find('img')
                else:
                    img = image_elem.find('img')
                
                if img and isinstance(img, Tag):
                    # Try different image source attributes
                    image_url = None
                    for attr in ['data-src', 'src']:
                        url = img.get(attr, '')
                        if isinstance(url, str) and url and not url.startswith('data:'):
                            image_url = url
                            break
                    
                    if image_url:
                        if image_url.startswith('//'):
                            image_url = 'https:' + image_url
                        item['image'] = self.download_image(image_url, item_name)
            
            # Find the main item table
            item_table = soup.find('table', {'class': 'infobox'})
            if item_table and isinstance(item_table, Tag):
                rows = item_table.find_all('tr')
                for row in rows:
                    if not isinstance(row, Tag):
                        continue
                        
                    # Get cells
                    cells = row.find_all(['th', 'td'])
                    if len(cells) >= 2:
                        header = cells[0].get_text(strip=True)
                        value = cells[1].get_text(strip=True)
                        
                        if header == "Cost:":
                            try:
                                item['cost'] = int(value)
                            except ValueError:
                                pass
                        elif header == "Total Cost:":
                            try:
                                item['total_cost'] = int(value)
                            except ValueError:
                                pass
                        elif header == "Item Type:":
                            item['type'] = [t.strip() for t in value.split(',')]
                        elif header == "Stats:":
                            stats = value.split('+')
                            item['stats'] = [stat.strip() for stat in stats if stat.strip()]
                        elif header == "Passive Effect:":
                            item['passive'] = self.clean_text(value)
                        elif header == "Active Effect:":
                            item['active'] = self.clean_text(value)
            
            return item
        except Exception as e:
            print(f"Error scraping {item_name}: {str(e)}")
            return None

    def scrape_all_item_details(self):
        """Scrape details for all items in full_items.json."""
        try:
            # Load item names
            with open('output/full_items.json', 'r', encoding='utf-8') as f:
                items_data = json.load(f)
            
            item_details = {}
            
            # Combine all item lists
            all_items = []
            for category in items_data.values():
                all_items.extend(category)
            
            # Remove duplicates while preserving order
            all_items = list(dict.fromkeys(all_items))
            
            # Scrape each item
            for item_name in all_items:
                print(f"Scraping {item_name}...")
                details = self.scrape_item_details(item_name)
                if details:
                    item_details[item_name.lower()] = details
                time.sleep(0.001)  # Be nice to the server
            
            # Save results
            with open('output/item_details.json', 'w', encoding='utf-8') as f:
                json.dump(item_details, f, indent=2)
                
        except Exception as e:
            print(f"Error scraping item details: {str(e)}")

if __name__ == "__main__":
    import sys
    
    scraper = ItemScraper()
    
    # Check command line arguments
    if len(sys.argv) > 1:
        if sys.argv[1] == "names":
            print("Scraping item names...")
            scraper.scrape_item_names()
        elif sys.argv[1] == "details":
            print("Scraping item details...")
            scraper.scrape_all_item_details()
        else:
            print("Invalid argument. Use 'names' to scrape item names or 'details' to scrape item details.")
    else:
        print("Running complete scrape...")
        scraper.scrape_item_names()
        scraper.scrape_all_item_details() 