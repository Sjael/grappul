#!/usr/bin/env python3
"""Test scraper to debug wiki structure"""

import requests
from bs4 import BeautifulSoup

def test_gods_page():
    """Test what's on the gods page"""
    url = "https://smite.fandom.com/wiki/List_of_gods"
    response = requests.get(url)
    soup = BeautifulSoup(response.content, 'html.parser')
    
    print("=== TESTING GODS PAGE ===")
    print(f"URL: {url}")
    print(f"Status: {response.status_code}")
    
    # Check for redirects
    if response.history:
        print(f"Redirected to: {response.url}")
    
    # Look for tables
    tables = soup.find_all('table')
    print(f"\nFound {len(tables)} tables")
    
    for i, table in enumerate(tables[:3]):  # First 3 tables
        print(f"\nTable {i}:")
        print(f"  Classes: {table.get('class', [])}")
        rows = table.find_all('tr')
        print(f"  Rows: {len(rows)}")
        if rows:
            # Check first row for headers
            headers = rows[0].find_all(['th', 'td'])
            print(f"  Headers: {[h.text.strip() for h in headers[:5]]}")
            
            # Check second row for data
            if len(rows) > 1:
                cells = rows[1].find_all(['td', 'th'])
                print(f"  First data row cells: {len(cells)}")
                for j, cell in enumerate(cells[:3]):
                    links = cell.find_all('a')
                    if links:
                        print(f"    Cell {j} links: {[l.get('href', '') for l in links[:2]]}")
    
    # Look for main content div
    content = soup.find('div', {'class': 'mw-parser-output'})
    if content:
        print("\nFound mw-parser-output div")
        # Look for any lists of gods
        lists = content.find_all('ul')
        print(f"Found {len(lists)} lists")
    
    # Check page title to ensure we're on the right page
    title = soup.find('h1', {'class': 'page-header__title'})
    if title:
        print(f"\nPage title: {title.text.strip()}")

def test_items_page():
    """Test what's on the items page"""
    url = "https://smite.fandom.com/wiki/Items"
    response = requests.get(url)
    soup = BeautifulSoup(response.content, 'html.parser')
    
    print("\n\n=== TESTING ITEMS PAGE ===")
    print(f"URL: {url}")
    print(f"Status: {response.status_code}")
    
    # Look for section headers
    headers = soup.find_all(['h2', 'h3'])
    print(f"\nFound {len(headers)} section headers")
    for header in headers[:10]:
        print(f"  - {header.text.strip()}")
    
    # Look for specific sections
    for section_name in ['Consumables', 'Relics', 'Starter', 'Tier 1', 'Passive']:
        header = soup.find(['h2', 'h3'], text=lambda t: t and section_name in t)
        if header:
            print(f"\nFound section: {header.text.strip()}")
            # Look for items after this header
            next_elem = header.find_next_sibling()
            if next_elem:
                links = next_elem.find_all('a', href=True)[:5]
                print(f"  First few items: {[l.text.strip() for l in links if l.text.strip()]}")

if __name__ == '__main__':
    test_gods_page()
    test_items_page()