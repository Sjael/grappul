#!/usr/bin/env python3
"""Debug script to show how to properly extract ability descriptions."""

import requests
from bs4 import BeautifulSoup

def extract_ability_description():
    """Show the exact method to extract ability descriptions."""
    
    url = "https://smite.fandom.com/wiki/Chronos"
    print(f"Fetching: {url}")
    
    try:
        response = requests.get(url)
        response.raise_for_status()
        soup = BeautifulSoup(response.text, 'html.parser')
        
        # Find the Abilities section
        abilities_header = soup.find('span', {'id': 'Abilities'})
        if not abilities_header:
            print("ERROR: Could not find Abilities section!")
            return
        
        # Find all wikitables after the Abilities header
        current = abilities_header.parent
        wikitables = []
        
        while current and len(wikitables) < 5:
            current = current.find_next_sibling()
            if current:
                tables = current.find_all('table', class_='wikitable', recursive=True)
                wikitables.extend(tables)
                
                if current.name == 'table' and 'wikitable' in current.get('class', []):
                    if current not in wikitables:
                        wikitables.append(current)
        
        # Extract descriptions for all abilities
        print("\n=== ABILITY DESCRIPTIONS ===\n")
        
        for i, table in enumerate(wikitables):
            # Get ability name from header
            header = table.find('th')
            if header:
                ability_name = header.get_text(strip=True)
                print(f"[{i}] {ability_name}")
                
                # Find the description cell
                # It's typically in the 3rd row (index 2), in a cell with colspan="2"
                rows = table.find_all('tr')
                if len(rows) > 2:
                    # Check the third row
                    row = rows[2]
                    cells = row.find_all('td')
                    
                    for cell in cells:
                        # Look for cells with colspan="2" that contain the description
                        if cell.get('colspan') == '2':
                            # Extract the main description text
                            description_parts = []
                            
                            # Get the direct text content (before any nested tags)
                            for content in cell.children:
                                if isinstance(content, str):
                                    text = content.strip()
                                    if text:
                                        description_parts.append(text)
                                elif content.name == 'p':
                                    # Get text from nested paragraphs
                                    p_text = content.get_text(strip=True)
                                    if p_text:
                                        description_parts.append(p_text)
                            
                            # Join all parts
                            full_description = ' '.join(description_parts)
                            
                            if full_description:
                                print(f"   Description: {full_description}\n")
                                
                                # Also show the raw cell content for debugging
                                print(f"   Raw cell text: {cell.get_text(strip=True)}\n")
                                break
                
                print("-" * 80 + "\n")
                
    except Exception as e:
        print(f"ERROR: {str(e)}")
        import traceback
        traceback.print_exc()

if __name__ == "__main__":
    extract_ability_description()