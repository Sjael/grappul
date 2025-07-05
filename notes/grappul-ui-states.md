# Grappul - UI State Examples

## Main Build Page

### State 1: Initial Load (No God Selected)
```
+------------------+--------------------------------+
| GODS & FILTERS   | Welcome to Grappul            |
|                  |                                |
| [Search...]      | Select a god from the left     |
|                  | to view builds                 |
| Filters:         |                                |
| Role: [All ▼]    | [Subtle gradient background    |
| Class: [All ▼]   |  with Smite-themed imagery]    |
|                  |                                |
| A                |                                |
| • Achilles       |                                |
| • Agni           |                                |
| • Ah Muzen Cab   |                                |
| • Ah Puch        |                                |
| ...              |                                |
+------------------+--------------------------------+
```

### State 2: Loading Build Data
```
+------------------+--------------------------------+
| GODS & FILTERS   | Agni                          |
|                  | The God of Fire               |
| [Search: Agni]   |                               |
|                  | [Subtle loading animation]     |
| Filters:         | Loading builds...              |
| Role: [Mid ▼]    |                               |
| Class: [Mage ▼]  |                               |
|                  |                               |
| A                |                               |
| • Achilles       |                               |
| • Agni [selected]|                               |
| • Ah Muzen Cab   |                               |
+------------------+--------------------------------+
```

### State 3: Build Loaded Successfully
```
+------------------+--------------------------------+
| GODS & FILTERS   | Agni                          |
|                  | [Mid] [Jungle] [Support]      |
| [Search: Agni]   |                               |
|                  | FINAL BUILD                    |
| Filters:         | [Book of] [Spear of] [Soul]   |
| Role: [Mid ▼]    | [Thoth  ] [Deso   ] [Reaver]  |
| Class: [Mage ▼]  |                               |
|                  | [Obsi  ] [Tahuti ] [Staff ]   |
| • Agni [selected]| [Shard ] [      ] [of Myr]   |
|                  |                               |
|                  | SKILL ORDER                    |
|                  | 1→4→2→3 (Max 4→2→3→1)         |
|                  |                               |
|                  | KEY TIPS                       |
|                  | • Use [icon] Path of Flames   |
|                  |   for wave clear              |
|                  | • Combo [icon] Noxious Fumes  |
|                  |   with stun for burst         |
|                  |                               |
|                  | [▼ Show Timeline & Details]   |
+------------------+--------------------------------+
```

### State 4: Build Expanded View
```
+------------------+--------------------------------+
| GODS & FILTERS   | Agni                          |
|                  | [Mid] [Jungle] [Support]      |
|                  |                               |
|                  | BUILD TIMELINE                 |
|                  | Start: [Conduit] → [Pots]     |
|                  | Early: [Lost Artifact] →      |
|                  |        [Book of Thoth]        |
|                  | Mid:   [Spear of Deso]        |
|                  | Late:  Complete build above   |
|                  |                               |
|                  | PROS & CONS                    |
|                  | ✓ High burst damage           |
|                  | ✓ Excellent poke              |
|                  | ✗ No escape                   |
|                  | ✗ Mana hungry early           |
|                  |                               |
|                  | EXTENDED TIPS                  |
|                  | Early game focus on...        |
|                  | [Extended text with inline    |
|                  |  icons for items/abilities]   |
+------------------+--------------------------------+
```

### State 5: No Builds Available
```
+------------------+--------------------------------+
| GODS & FILTERS   | Cthulhu                       |
|                  | [Support] [Solo] [Jungle]     |
|                  |                               |
|                  | No builds available yet       |
|                  |                               |
|                  | [Create Build] button         |
|                  |                               |
|                  | Check back after next patch   |
|                  | or create your own build      |
+------------------+--------------------------------+
```

## Tier List Page

### State 1: Community Tier List View
```
+------------------+--------------------------------+
| TIER LISTS       | Community Tier List           |
|                  | Updated: 2 hours ago          |
| • Community      | Based on 1,247 votes          |
| • [User] Jake's  |                               |
| • [User] Pro1    | S TIER                        |
| • Create New     | [Agni] [Athena] [Bastet]     |
|                  |                               |
| Filter:          | A TIER                        |
| Role: [All ▼]    | [Achilles] [Apollo] [Ares]   |
| Patch: [5.1 ▼]   | [Artemis] [Bacchus]          |
|                  |                               |
|                  | B TIER                        |
|                  | [Cabrakan] [Chaac] [Chang'e] |
|                  |                               |
|                  | [View More Tiers ▼]          |
+------------------+--------------------------------+
```

### State 2: Creating/Editing Tier List
```
+------------------+--------------------------------+
| TIER LISTS       | My Tier List (Draft)          |
|                  | [Publish] [Save Draft]        |
| • Community      |                               |
| • [User] Jake's  | S TIER                        |
| • Create New     | [Drop gods here]              |
|   [editing]      | ┌─────────────────────────┐  |
|                  | │ [Agni] → (dragging)     │  |
|                  | └─────────────────────────┘  |
|                  | A TIER                        |
|                  | [Achilles] [Apollo]          |
|                  |                               |
|                  | UNRANKED GODS                 |
|                  | [All remaining gods in grid]  |
+------------------+--------------------------------+
```

## Cheatsheet Page

### State 1: Full View Loaded
```
+--------------------------------+
| CHEATSHEET - Quick Reference   |
| [Search...] [Role ▼] [Class ▼] |
|                                |
| ┌─────────┬─────────┬────────┐|
| │ AGNI    │ ACHILLES│ APOLLO ││
| │ Mid/Jng │ Solo    │ ADC    ││
| │ 4→2→3→1 │ 1→3→2→4 │ 1→3→2→4││
| │ [items] │ [items] │ [items]││
| ├─────────┼─────────┼────────┤|
| │ ARES    │ ARTEMIS │ ATHENA ││
| │ Support │ ADC     │ Support││
| │ 4→3→2→1 │ 3→1→2→4 │ 3→1→2→4││
| │ [items] │ [items] │ [items]││
| └─────────┴─────────┴────────┘|
| [Load More...]                 |
+--------------------------------+
```

### State 2: Filtered View
```
+--------------------------------+
| CHEATSHEET - Quick Reference   |
| [Search: "war"] [Role: Solo ▼] |
|                                |
| Showing 3 results              |
|                                |
| ┌─────────┬─────────┬────────┐|
| │ WARRIOR │ CHAAC   │ HERC   ││
| │ Solo    │ Solo    │ Solo   ││
| │ 1→3→2→4 │ 1→3→2→4 │ 3→1→2→4││
| │ [items] │ [items] │ [items]││
| └─────────┴─────────┴────────┘|
+--------------------------------+
```

## Common UI Patterns

### Tooltip States
```
[Book of Thoth] ←─┐
                  │
┌─────────────────▼────────────┐
│ Book of Thoth                │
│ +80 Magical Power            │
│ +250 Mana                    │
│ +20 MP5                      │
│                              │
│ PASSIVE: Gain power equal    │
│ to 7% of your mana          │
└──────────────────────────────┘
```

### Error States
```
+------------------+--------------------------------+
| GODS & FILTERS   | Oops!                         |
|                  |                               |
| [Search...]      | Unable to load build data     |
|                  |                               |
|                  | [Retry] [Report Issue]        |
|                  |                               |
|                  | Error: Network timeout        |
+------------------+--------------------------------+
```

### User Authentication States

#### Logged Out
```
Header: [Grappul] [Builds] [Tiers] [Cheatsheet] [Login]
```

#### Logged In
```
Header: [Grappul] [Builds] [Tiers] [Cheatsheet] [My Builds] [Profile ▼]
```

### Mobile Responsive States

#### Mobile Navigation (Collapsed)
```
[☰] Grappul     [Search]

[Main content area]
```

#### Mobile Navigation (Expanded)
```
[×] Grappul     [Search]
├─ Gods
├─ Builds
├─ Tier Lists
├─ Cheatsheet
└─ My Account
```

## Design Notes

- **Loading states**: Subtle skeleton screens matching content structure
- **Transitions**: 200ms ease-out for all state changes
- **Empty states**: Always provide actionable next steps
- **Error states**: Clear, friendly language with recovery options
- **Hover states**: +5% brightness on interactive elements
- **Focus states**: Subtle outline for accessibility
- **Active states**: Slightly inset appearance with shadow