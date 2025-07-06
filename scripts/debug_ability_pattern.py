#!/usr/bin/env python3
"""Show the exact pattern for extracting ability descriptions from Smite wiki."""

import requests
from bs4 import BeautifulSoup

def extract_ability_info(ability_table):
    """Extract all information from a single ability table."""
    
    ability_info = {}
    
    # Get ability name from header
    header = ability_table.find('th')
    if header:
        header_text = header.get_text(strip=True)
        # Parse "Passive - Time Lord" or "1st Ability - Time Rift"
        if ' - ' in header_text:
            ability_type, ability_name = header_text.split(' - ', 1)
            ability_info['type'] = ability_type.strip()
            ability_info['name'] = ability_name.strip()
        else:
            ability_info['name'] = header_text.strip()
    
    # Get all rows
    rows = ability_table.find_all('tr')
    
    # Extract description (typically in row 2, colspan=2 cell)
    if len(rows) > 2:
        desc_row = rows[2]
        desc_cells = desc_row.find_all('td')
        
        for cell in desc_cells:
            if cell.get('colspan') == '2':
                # This is the description cell
                ability_info['description'] = cell.get_text(separator=' ', strip=True)
                break
    
    # Extract stats (from remaining rows)
    stats = {}
    for row in rows[3:]:  # Start from row 3
        cells = row.find_all('td')
        
        # Stats are usually in pairs (label: value)
        for i in range(0, len(cells), 2):
            if i + 1 < len(cells):
                label = cells[i].get_text(strip=True)
                value = cells[i + 1].get_text(strip=True)
                
                if label and value and ':' in label:
                    # Clean up the label
                    label = label.rstrip(':').strip()
                    stats[label] = value
    
    ability_info['stats'] = stats
    
    return ability_info

def test_chronos_abilities():
    """Test extraction on Chronos abilities."""
    
    url = "https://smite.fandom.com/wiki/Chronos"
    print(f"Testing ability extraction on: {url}\n")
    
    try:
        response = requests.get(url)
        response.raise_for_status()
        soup = BeautifulSoup(response.text, 'html.parser')
        
        # Find abilities section
        abilities_header = soup.find('span', {'id': 'Abilities'})
        if not abilities_header:
            print("ERROR: Could not find Abilities section!")
            return
        
        # Find all wikitables
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
        
        # Extract info from each ability
        for i, table in enumerate(wikitables):
            print(f"=== Ability {i} ===")
            info = extract_ability_info(table)
            
            print(f"Name: {info.get('name', 'Unknown')}")
            print(f"Type: {info.get('type', 'Unknown')}")
            print(f"Description: {info.get('description', 'No description found')[:200]}...")
            
            if info.get('stats'):
                print("Stats:")
                for stat, value in list(info['stats'].items())[:5]:  # Show first 5 stats
                    print(f"  - {stat}: {value}")
            
            print("\n")
            
    except Exception as e:
        print(f"ERROR: {str(e)}")
        import traceback
        traceback.print_exc()

if __name__ == "__main__":
    test_chronos_abilities()