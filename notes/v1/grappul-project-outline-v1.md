# Grappul - Smite Build Companion App (v1 Implementation)

## Core Concept
Web-based companion app for Smite focused on character builds and god viability - WASM-powered Rust application using Dioxus framework.

## Current Implementation Status

### 1. Build Management ✅
- **Curated Builds** - Pre-configured expert builds for gods
- **Build Display** - Complete 6-item build with hover tooltips
- **Build Timeline** - Item purchase order/progression visualization
- **Custom Build Creator** - Partial implementation (UI complete, saving in progress)

#### Build Details (Currently Implemented)
- **Role Selection** - Selectable roles for each god (ADC, Support, Mid, Jungle, Solo)
- **Final Build** - Complete 6-item build display with hover tooltips
- **Build Timeline** - Item purchase order with timeline visualization
- **Skill Order** - Ability leveling sequence (1-20) display
- **Strategy Guide** - Markdown-rendered gameplay advice

### 2. God Browser ✅
- **God Grid** - Visual grid with portraits for all 124 gods
- **Class Filters** - Filter by Mage, Hunter, Guardian, Warrior, Assassin
- **Role Filters** - Filter by ADC, Support, Mid, Jungle, Solo
- **Search** - Quick search functionality with keyboard shortcuts
- **Persistent Selection** - Remembers selected god across sessions

### 3. Cheatsheet Page ✅
- **Quick Reference View** - Synopsis of builds for all gods simultaneously
- **Condensed Format** - Essential build info without full explanations
- **Keyboard Navigation** - Press any letter to focus search
- **Filter Support** - Works with class and role filters
- **Information Dense Design** - Optimized for quick scanning

### 4. Data Management ✅
- **Static Data** - Pre-processed god, item, and ability data
- **Local Storage** - User preferences and selections persistence
- **Python Scrapers** - Wiki data extraction scripts
- **Asset Pipeline** - Organized images for gods, items, and abilities

## UI/UX Design Implementation

### Core Principles (Achieved)
- **Clean** - Uncluttered interface with generous whitespace
- **Natural** - Intuitive flow with familiar navigation patterns
- **Subtle** - Refined interactions without flashy elements

### Visual Style (Implemented)
- Minimalistic design with purposeful restraint
- Flat, muted color palette as foundation
- CSS variable-based theming system
- Typography-focused hierarchy
- Soft shadows and gentle transitions
- Dark mode support

### Color System ✅
- **Stat-Specific Colors** - Consistent with in-game representation
  - Magical Power: Purple (#9B59B6)
  - Physical Power: Orange (#E67E22)
  - Cooldown Reduction: Blue (#3498DB)
  - Attack Speed: Yellow (#F1C40F)
  - Health: Green (#27AE60)
  - Mana: Blue (#2980B9)
  - Penetration: Red (#E74C3C)

### Interactive Elements ✅
- **Gentle Hover States** - Subtle color shifts on interactive elements
- **Smooth Transitions** - CSS transitions for all state changes
- **Smart Tooltips** - Context-aware positioning that avoids viewport edges
- **Visual Feedback** - Immediate response to user interactions

### Layout (Current)
- **Header Navigation** - Logo, page links, dark mode toggle
- **Main Content Area** 
  - **Home Page**: Side-by-side god selection and build display
  - **Cheatsheet**: Full-width grid layout
  - **Guide Creator**: Form-based interface with preview
- **Responsive Considerations** - Basic mobile compatibility

## Technical Implementation

### Data Structure
- **Gods**: 124 gods with complete ability data
- **Items**: 162 items with stats and effects
- **Guides**: Pre-configured builds with timelines and strategies
- **Static Storage**: All data compiled into the binary

### Core Technologies
- **Frontend**: Dioxus 0.6.3 (React-like framework for Rust)
- **Language**: Rust 2021 edition
- **Deployment**: WebAssembly (WASM) via wasm-bindgen
- **Styling**: Custom CSS with CSS variables
- **State Management**: Dioxus signals and context providers
- **Storage**: localStorage for user preferences
- **Build Tool**: Trunk for WASM bundling

### Performance Characteristics
- **Bundle Size**: ~500KB WASM (uncompressed)
- **Load Time**: < 2s on modern connections
- **Runtime Performance**: 60fps scrolling and interactions
- **Memory Usage**: Efficient due to Rust's memory management

## Features Not Yet Implemented

### User Features
- User authentication system
- User-created builds persistence
- Build sharing functionality
- Community voting/rating system

### Content Features
- Tier lists (community/user/curated)
- Extended matchup advice
- Patch notes integration
- Build statistics/analytics

### Technical Features
- Backend API
- Real-time updates
- Mobile-specific optimizations
- PWA capabilities
- Social sharing

## Current User Flow

1. **Landing** - User arrives at god selection grid
2. **Browse** - Filter/search for desired god
3. **Select** - Click god to view build details
4. **Explore** - View build, timeline, skill order, and strategy
5. **Reference** - Use cheatsheet for quick lookups
6. **Create** - Optionally create custom builds (limited functionality)

## Deployment Status
- **Development**: Local development with Dioxus CLI
- **Build**: Trunk builds optimized WASM bundle
- **Hosting**: Ready for static site deployment
- **Assets**: All images embedded in binary or served statically