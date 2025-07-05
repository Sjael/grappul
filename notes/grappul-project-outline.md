# Grappul - Smite Build Companion App

## Core Concept
Web-based companion app for Smite focused on character builds and god viability

## Key Features

### 1. Build Management
- **Popular Builds** - Community-favored build configurations
- **Curated Builds** - Creator's expert recommendations
- **Custom Build Creator** - User-created and saved builds

#### Build Details (Right Panel Content)
- **Role Selection** - Selectable roles for each god (one default selected)
- **Final Build** - Complete 6-item build display with hover tooltips
- **Build Timeline** - Item purchase order/progression
- **Tips & Tricks** - Gameplay advice with inline icons
- **Pros & Cons** - Build strengths and weaknesses with inline icons
- **Skill Order** - Ability leveling sequence (1-20)

### 2. God Tier Lists
- **Community Tier List** - Aggregated community sentiment on god viability
- **User Tier Lists** - Players can create and share their own tier lists
- **Curated Tier Lists** - Creator's expert tier rankings
- **Tier Format** - Traditional S/A/B/C/D tiers or similar ranking system

### 3. Cheatsheet Page
- **Quick Reference View** - Synopsis of builds for all gods simultaneously
- **Condensed Format** - Essential build info without full explanations
- **For Advanced Users** - Quick lookup for experienced players
- **Information Dense Design** - Intentionally packed for quick scanning
- **Different Design Rules** - Can be "noisier" than main pages

## UI/UX Design Philosophy

### Core Principles
- **Clean** - Uncluttered interface with generous whitespace
- **Natural** - Intuitive flow that feels effortless to navigate
- **Subtle** - Refined interactions without flashy elements

### Visual Style
- Minimalistic design with purposeful restraint
- Flat, muted color palette as foundation
- Subtle gradients/saturation only for critical actions
- Typography-focused hierarchy
- Soft shadows and gentle transitions
- No harsh lines or aggressive contrasts

### Color System
- **Stat-Specific Colors** - Consistent with in-game representation
  - Magical Power: Purple/Blue
  - Physical Power: Orange/Red
  - Cooldown Reduction: Blue
  - Attack Speed: Yellow
  - Health: Green
  - Mana: Blue
  - [etc. matching Smite's color scheme]

### Text Enhancement
- **Dynamic Inline Icons** - Auto-detection and placement system
  - Scans text for item/ability names
  - Inserts small icons inline next to mentions
  - Breaks up text walls for better readability
  - Icons are subtle, matching text size
  - Hoverable for quick info

### Interactive Elements
- **Gentle Hover States** - Subtle color shifts, not dramatic changes
- **Smooth Transitions** - Ease-in-out animations, nothing jarring
- **Understated Tooltips** - Appear softly on hover with essential info
- **Quiet Feedback** - Confirmations that don't interrupt flow

### Layout
- **Side-by-side split screen** (Main pages)
  - **Left Panel (Navigation)**
    - Searchable god list
    - Filters:
      - By Role (ADC, Support, Mid, Jungle, Solo)
      - By Class (Mage, Hunter, Guardian, Warrior, Assassin)
  - **Right Panel (Content)**
    - Large dynamic content area
    - God-specific content with role tabs
    - Build information sections

## Competitive Differentiation (vs. Mobalytics, U.GG, OP.GG)

### What Grappul Does Differently
- **Information Hierarchy** - Show only what's needed, when it's needed
- **Progressive Disclosure** - Start simple, reveal complexity on demand
- **Breathing Room** - Generous whitespace, no cramped layouts
- **Focus on Essentials** - Core build info first, advanced stats hidden
- **No Ads/Clutter** - Pure focus on the user's needs

### Avoiding Common Pitfalls
- ❌ Multiple win rates/pick rates/ban rates everywhere
- ❌ Overwhelming stat tables
- ❌ Competing visual elements fighting for attention
- ❌ Auto-playing videos or intrusive ads
- ✅ One clear path through the information
- ✅ Stats available but not forced
- ✅ Visual calm with purposeful emphasis

### Build Section Layout Strategy

#### Primary View (What Users See First)
1. **Role Tabs** - Clean, obvious selection
2. **Final Build** - 6 items, clear and prominent
3. **Skill Order** - Simple 1-20 progression (Smite's level cap)
4. **Key Tips** - Most important gameplay points (can be multiple)

#### Secondary View (Expandable/On-Demand)
- Build timeline details
- Extended tips and tricks
- Full pros/cons list
- Matchup-specific advice
- Statistical data (if desired)

## Technical Implementation

### Data Management

#### Wiki Scraping Targets
- **Base Wiki**: https://smite.fandom.com/wiki/Smite_Wiki
- **Gods List Page**: Main page containing all god names and portrait images
- **Individual God Pages**: https://smite.fandom.com/wiki/[GodName]
  - Ability names, descriptions, and icons
  - Base stats and scaling
  - Lore and voice lines (optional)
- **Items Page**: https://smite.fandom.com/wiki/Items
  - All item stats, costs, and icons
  - Item categorization:
    - Tier 1, 2, 3 items
    - Starter items
    - Evolved items
    - Consumables
    - Relics

#### Data Structure Requirements
- **Gods**:
  - Name, title, pantheon, class, roles
  - Portrait and ability icons
  - Base stats (health, mana, protections, etc.)
  - Ability details (damage, scaling, cooldowns)
- **Items**:
  - Name, cost, stats
  - Icon image
  - Tags: tier level, item tree, stats provided
  - Build path (what it builds into/from)
- **Version Control**: Track patch version for data updates

#### Scraping Implementation
- **Wiki Scraping Scripts** (Rust-based)
  - Parse wiki HTML for structured data
  - Download and optimize images
  - Handle wiki formatting inconsistencies
  - Rate limiting to respect wiki servers
- **Local Storage** - Cached data stored locally
- **Update System** - Manual/scheduled scraping on patch releases

### Core Technologies (TBD)
- User authentication system
- Build & tier list database
- Voting/rating system
- Mobile responsiveness
- Tooltip system
- **Text Parser** - Dynamic icon insertion system

## Future Considerations

### For Software Engineers
- Consider scalable architecture for user-generated content
- Plan for efficient data caching and updates
- Design flexible API for build/tier list CRUD operations
- Implement robust text parsing for dynamic icon insertion

### For UX Designers
- Focus on reducing cognitive load
- Create clear visual hierarchies
- Design smooth, subtle interactions
- Ensure mobile experience maintains the clean aesthetic