# Grappul - UI State Examples (v1 Implementation)

## Main Build Page (Home Route)

### State 1: Initial Load
```
+------------------+--------------------------------+
| GRAPPUL          | [Builds] [Cheatsheet] [🌙]    |
+------------------+--------------------------------+
| GODS & FILTERS   | Welcome to Grappul            |
|                  |                                |
| [Search...]      | Select a god from the left     |
|                  | to view builds                 |
| Class: [All ▼]   |                                |
| Role: [All ▼]    | [Clean gradient background]    |
|                  |                                |
| [Grid of god     |                                |
|  portraits in    |                                |
|  alphabetical    |                                |
|  order]          |                                |
+------------------+--------------------------------+
```

### State 2: God Selected with Build
```
+------------------+--------------------------------+
| GRAPPUL          | [Builds] [Cheatsheet] [🌙]    |
+------------------+--------------------------------+
| GODS & FILTERS   | AGNI - The God of Fire        |
|                  | Mid                           |
| [Search: Agni]   |                               |
|                  | ABILITIES                      |
| Class: [Mage ▼]  | [Q][W][E][R] (hover for info) |
| Role: [Mid ▼]    |                               |
|                  | BUILD                          |
| [Agni portrait   | [Conduit] [Book of] [Spear]   |
|  highlighted]    | [Gem    ] [Thoth  ] [of Des]  |
|                  |                               |
|                  | [Soul  ] [Tahuti] [Ob Shard]  |
|                  | [Reaver] [      ] [        ]  |
|                  |                               |
|                  | RELICS                         |
|                  | [Purification] [Aegis]         |
|                  |                               |
|                  | SKILL ORDER                    |
|                  | 4 > 2 > 3 > 1                 |
|                  |                               |
|                  | TIMELINE                       |
|                  | [Visual timeline bars showing  |
|                  |  item purchase progression]    |
|                  |                               |
|                  | STRATEGY                       |
|                  | [Markdown rendered guide text] |
+------------------+--------------------------------+
```

### State 3: Filtered View
```
+------------------+--------------------------------+
| GRAPPUL          | [Builds] [Cheatsheet] [🌙]    |
+------------------+--------------------------------+
| GODS & FILTERS   | Select a god to view builds   |
|                  |                                |
| [Search: war]    | Showing: Warriors             |
|                  |                                |
| Class: Warrior ✓ |                                |
| Role: [Solo ▼]   |                                |
|                  |                                |
| [Filtered grid   |                                |
|  showing only    |                                |
|  Warrior gods]   |                                |
+------------------+--------------------------------+
```

## Cheatsheet Page

### State 1: Full Cheatsheet View
```
+--------------------------------------------------------+
| GRAPPUL          | [Builds] [Cheatsheet] [🌙]         |
+--------------------------------------------------------+
| CHEATSHEET                                             |
| [Search...] Class: [All ▼] Role: [All ▼]              |
|                                                        |
| Press any letter key to focus search                   |
|                                                        |
| ┌─────────────┬─────────────┬─────────────┐          |
| │ ACHILLES    │ AGNI        │ AH MUZEN CAB│          |
| │ Solo        │ Mid         │ ADC         │          |
| │ [Bluestone] │ [Conduit]   │ [Leather]   │          |
| │ Build: ...  │ Build: ...  │ Build: ...  │          |
| │ 1>3>2>4     │ 4>2>3>1     │ 2>1>3>4     │          |
| ├─────────────┼─────────────┼─────────────┤          |
| │ AH PUCH     │ AMATERASU   │ ANHUR       │          |
| │ Mid         │ Solo        │ ADC         │          |
| │ [Conduit]   │ [Bluestone] │ [Leather]   │          |
| │ Build: ...  │ Build: ...  │ Build: ...  │          |
| │ 1>3>4>2     │ 2>3>1>4     │ 1>3>2>4     │          |
| └─────────────┴─────────────┴─────────────┘          |
| [... more god cards in grid layout ...]                |
+--------------------------------------------------------+
```

### State 2: Filtered Cheatsheet
```
+--------------------------------------------------------+
| GRAPPUL          | [Builds] [Cheatsheet] [🌙]         |
+--------------------------------------------------------+
| CHEATSHEET                                             |
| [Search: hunter] Class: Hunter ✓ Role: ADC ✓          |
|                                                        |
| Showing 8 results                                      |
|                                                        |
| ┌─────────────┬─────────────┬─────────────┐          |
| │ AH MUZEN CAB│ ANHUR       │ APOLLO      │          |
| │ ADC         │ ADC         │ ADC         │          |
| │ [Leather]   │ [Leather]   │ [Leather]   │          |
| │ Build: ...  │ Build: ...  │ Build: ...  │          |
| │ 2>1>3>4     │ 1>3>2>4     │ 1>3>2>4     │          |
| └─────────────┴─────────────┴─────────────┘          |
+--------------------------------------------------------+
```

## Guide Creator Page

### State 1: Initial Guide Creation
```
+--------------------------------------------------------+
| GRAPPUL          | [Builds] [Cheatsheet] [🌙]         |
+--------------------------------------------------------+
| CREATE GUIDE                                           |
|                                                        |
| Select God: [Dropdown - Select a god ▼]               |
| Select Role: [Dropdown - Select a role ▼]             |
|                                                        |
| [Continue button - disabled until selections made]     |
+--------------------------------------------------------+
```

### State 2: Guide Editor
```
+--------------------------------------------------------+
| GRAPPUL          | [Builds] [Cheatsheet] [🌙]         |
+--------------------------------------------------------+
| CREATE GUIDE - AGNI (MID)                             |
|                                                        |
| SKILL ORDER                                            |
| [1][2][3][4] - Drag to reorder                        |
|                                                        |
| BUILD TIMELINE                                         |
| [+ Add Timeline Entry]                                 |
| ┌────────────────────────────────────┐               |
| │ Entry 1: Start                     │               |
| │ Items: [Select items...]           │               |
| │ Notes: [Text input]                │               |
| └────────────────────────────────────┘               |
|                                                        |
| ITEMS (Click to add to build)                         |
| [Grid of all available items with search/filter]       |
|                                                        |
| SELECTED BUILD                                         |
| [Empty slots for 6 items]                             |
|                                                        |
| RELICS                                                 |
| [Select Relic 1 ▼] [Select Relic 2 ▼]                |
|                                                        |
| STRATEGY GUIDE                                         |
| [Markdown editor with preview]                         |
|                                                        |
| [Save Guide] [Preview]                                 |
+--------------------------------------------------------+
```

## Common UI Patterns

### Item Tooltip (Hover State)
```
Mouse hovering over [Book of Thoth] item

┌─────────────────────────────────┐
│ Book of Thoth                   │
│                                 │
│ +80 Magical Power               │
│ +250 Mana                       │
│ +20 MP5                         │
│                                 │
│ EVOLVED PASSIVE - You           │
│ permanently gain 10 Mana per    │
│ Stack, and receive 5 Stacks     │
│ for a god kill...              │
│                                 │
│ Cost: 2800                      │
└─────────────────────────────────┘
```

### Dark Mode Toggle States
```
Light Mode: [🌙] (moon icon)
Dark Mode: [☀️] (sun icon)

Theme changes affect:
- Background colors
- Text colors  
- Border colors
- Maintains contrast ratios
```

### Loading States
```
+------------------+--------------------------------+
| GODS & FILTERS   | Loading...                    |
|                  | [Subtle loading spinner]       |
| [Grid loading    |                                |
|  with skeleton   |                                |
|  placeholders]   |                                |
+------------------+--------------------------------+
```

### Search Interaction
```
User types in search box:
[Search: ama|] <- cursor

Results update in real-time:
- Amaterasu (highlighted match)
- Rama (contains 'ama')
```

### Mobile Responsive Behavior
```
Mobile View (<768px):
+------------------------+
| ☰ GRAPPUL         [🌙] |
+------------------------+
| [Search...]            |
| Class: All ▼ Role: All ▼|
+------------------------+
| [2-column god grid]    |
|                        |
+------------------------+

Tap on god -> Full screen build view
```

## Interaction Feedback

### Click States
- **God Portrait**: Subtle scale transform (0.95) on click
- **Buttons**: Slight darken on active
- **Links**: Underline on hover

### Transitions
- **Page Routes**: Instant navigation (SPA)
- **Filter Changes**: 200ms fade transition
- **Tooltip Appearance**: 100ms fade-in
- **Theme Switch**: 300ms color transitions

### Keyboard Shortcuts
- **Letter Keys**: Focus search on cheatsheet
- **Escape**: Clear search/close modals
- **Tab**: Navigate through interactive elements

## Error Handling

### No Build Available
```
+------------------+--------------------------------+
| GODS & FILTERS   | CTHULHU - The Great Dreamer   |
|                  | Support                        |
| [Cthulhu         |                                |
|  selected]       | No build available yet         |
|                  |                                |
|                  | Check back soon or create      |
|                  | your own build!                |
|                  |                                |
|                  | [Create Build]                 |
+------------------+--------------------------------+
```

### Failed to Load Data
```
+------------------+--------------------------------+
| GODS & FILTERS   | Error Loading Data            |
|                  |                                |
|                  | Unable to load game data.      |
|                  | Please refresh the page.       |
|                  |                                |
|                  | [Refresh Page]                 |
+------------------+--------------------------------+
```

## Design Implementation Notes

- **Consistent Spacing**: 8px grid system throughout
- **Color Variables**: CSS custom properties for theming
- **Responsive Images**: Optimized sizes for different viewports
- **Accessibility**: ARIA labels, keyboard navigation, focus indicators
- **Performance**: Virtual scrolling considered for large lists
- **State Persistence**: Selected god, filters, and theme saved to localStorage