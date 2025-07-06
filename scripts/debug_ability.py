#!/usr/bin/env python3
"""Debug script to analyze the structure of ability tables on Smite wiki."""

import requests
from bs4 import BeautifulSoup
import json

def debug_chronos_accelerate():
    """Fetch Chronos page and analyze the Accelerate ability table structure."""
    
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
        
        print("Found Abilities section")
        
        # Find all wikitables after the Abilities header
        current = abilities_header.parent
        wikitables = []
        
        # Search for wikitables in the following elements
        while current and len(wikitables) < 5:
            current = current.find_next_sibling()
            if current:
                # Check if current element contains wikitables
                tables = current.find_all('table', class_='wikitable', recursive=True)
                wikitables.extend(tables)
                
                # Also check if current element itself is a wikitable
                if current.name == 'table' and 'wikitable' in current.get('class', []):
                    if current not in wikitables:
                        wikitables.append(current)
        
        print(f"\nFound {len(wikitables)} wikitables")
        
        # Print all ability names to verify order
        print("\n=== ALL ABILITIES ===")
        for i, table in enumerate(wikitables):
            header = table.find('th')
            if header:
                ability_name = header.get_text(strip=True)
                print(f"[{i}] {ability_name}")
        
        if len(wikitables) >= 3:
            # Get the third ability (Accelerate is actually the 2nd active ability, which is index 2)
            accelerate_table = wikitables[2]  # Index 2 for Accelerate
            print("\n=== ACCELERATE ABILITY TABLE STRUCTURE ===")
            
            # Print table header info
            caption = accelerate_table.find('caption')
            if caption:
                print(f"Caption: {caption.get_text(strip=True)}")
            
            # Analyze all rows
            rows = accelerate_table.find_all('tr')
            print(f"\nTotal rows: {len(rows)}")
            
            for i, row in enumerate(rows):
                print(f"\n--- Row {i} ---")
                
                # Check for header cells
                ths = row.find_all('th')
                if ths:
                    print(f"Header cells ({len(ths)}):")
                    for j, th in enumerate(ths):
                        text = th.get_text(strip=True)
                        print(f"  [{j}] {text[:100]}...")
                
                # Check for data cells
                tds = row.find_all('td')
                if tds:
                    print(f"Data cells ({len(tds)}):")
                    for j, td in enumerate(tds):
                        text = td.get_text(strip=True)
                        # Check if this might be a description
                        if len(text) > 50:  # Descriptions are usually longer
                            print(f"  [{j}] POSSIBLE DESCRIPTION: {text[:200]}...")
                        else:
                            print(f"  [{j}] {text}")
                        
                        # Also check for any nested divs or paragraphs
                        nested_content = td.find_all(['div', 'p'])
                        if nested_content:
                            print(f"       Has {len(nested_content)} nested elements")
            
            # Look for description patterns
            print("\n=== SEARCHING FOR DESCRIPTION PATTERNS ===")
            
            # Pattern 1: Look for cells with long text
            all_cells = accelerate_table.find_all(['td', 'th'])
            for cell in all_cells:
                text = cell.get_text(strip=True)
                if len(text) > 100 and not any(stat in text.lower() for stat in ['damage', 'cooldown', 'cost', 'range']):
                    print(f"\nLong text found: {text[:200]}...")
                    print(f"Parent tag: {cell.parent.name}")
                    print(f"Cell tag: {cell.name}")
            
            # Pattern 2: Look for specific description indicators
            for cell in all_cells:
                # Check cell attributes
                cell_class = cell.get('class', [])
                cell_style = cell.get('style', '')
                colspan = cell.get('colspan', '')
                
                if colspan or 'description' in str(cell_class).lower():
                    print(f"\nSpecial cell found:")
                    print(f"  Class: {cell_class}")
                    print(f"  Colspan: {colspan}")
                    print(f"  Text: {cell.get_text(strip=True)[:200]}...")
            
            # Pattern 3: Check the raw HTML structure
            print("\n=== RAW HTML STRUCTURE (first 2000 chars) ===")
            table_html = str(accelerate_table)[:2000]
            print(table_html)
            
        else:
            print("ERROR: Could not find enough ability tables!")
            
    except Exception as e:
        print(f"ERROR: {str(e)}")
        import traceback
        traceback.print_exc()

if __name__ == "__main__":
    debug_chronos_accelerate()