# Grappul - Rust/Dioxus Technical Brief (v1 Implementation)

## Project Overview
A modern, performant web application for the Smite gaming community built entirely in Rust using Dioxus and WebAssembly. The app provides build guides, a quick-reference cheatsheet, and a build creation tool - delivering a cleaner, more focused alternative to traditional MOBA companion apps.

## Current Technology Stack
- **Frontend**: Dioxus 0.6.3 (React-like framework for Rust)
- **Runtime**: WebAssembly (WASM) via wasm-bindgen 0.2
- **Language**: Rust 2021 edition
- **Build Tool**: Trunk for WASM bundling
- **Styling**: Custom CSS with CSS variables
- **State Management**: Dioxus signals and context providers
- **Storage**: localStorage via web-sys
- **Data Processing**: Python 3 with BeautifulSoup for wiki scraping

## Core Technical Architecture

### Dioxus Frontend Implementation
- **Component Structure**:
  ```rust
  // Main routes
  - Home: God selection and build display
  - Cheatsheet: Quick reference grid
  - GuideCreator: Build creation interface
  
  // Core components
  - Header: Navigation and theme toggle
  - GodGrid: Filterable god selection
  - ClassFilters/RoleFilters: Filter controls
  - Item/Ability: Display with tooltips
  - Tooltip: Smart positioning system
  - MarkdownRenderer: Strategy content
  ```

- **State Management**:
  ```rust
  // Global state using Dioxus signals
  pub static FILTERED_CLASS: GlobalSignal<Option<String>> = Signal::global();
  pub static FILTERED_ROLE: GlobalSignal<Option<String>> = Signal::global();
  pub static SELECTED_GOD: GlobalSignal<Option<String>> = Signal::global();
  pub static SELECTED_BUILD: GlobalSignal<String> = Signal::global();
  
  // Context providers for tooltip positioning
  pub static HOVERED_ITEM: GlobalSignal<Option<Item>> = Signal::global();
  pub static MOUSE_POS: GlobalSignal<(f32, f32)> = Signal::global();
  pub static TOOLTIP_POS: GlobalSignal<(i32, i32)> = Signal::global();
  ```

- **Routing**:
  ```rust
  #[derive(Routable, Clone)]
  enum Route {
      #[route("/")]
      Home {},
      #[route("/cheatsheet")]
      Cheatsheet {},
      #[route("/guide/create")]
      GuideCreator {},
  }
  ```

### Data Layer Implementation

#### Current Approach: Static Data Compilation
- **Strategy**: All game data is compiled directly into the WASM binary
- **Benefits**: 
  - Zero runtime data fetching
  - Instant access to all information
  - No backend requirements
  - Offline capability
- **Implementation**:
  ```rust
  // Data is stored as static constants
  lazy_static! {
      pub static ref GODS: Vec<God> = serde_json::from_str(GODS_JSON).unwrap();
      pub static ref ITEMS: HashMap<String, Item> = load_items();
      pub static ref GUIDES: HashMap<String, Guide> = load_guides();
  }
  ```

### Current Data Pipeline

1. **Wiki Scraping** (Python)
   ```python
   # scripts/scrape_*.py files
   - scrape_gods.py: Extract god data from Smite wiki
   - scrape_items.py: Extract item stats and descriptions
   - process_images.py: Download and organize assets
   ```

2. **Data Processing**
   - JSON transformation for Rust consumption
   - Image optimization and organization
   - Build guide curation and formatting

3. **Asset Management**
   ```
   /assets/
   ├── abilities/     # 30+ ability icons
   ├── gods/         # 124 god portraits
   ├── items/        # 162 item icons
   └── icons/        # UI icons (classes, roles)
   ```

### Key Technical Implementations

1. **Smart Tooltip System**
   ```rust
   // Viewport-aware positioning
   pub fn calculate_tooltip_position(mouse_x: f32, mouse_y: f32) -> (i32, i32) {
       // Prevents tooltips from going off-screen
       // Adjusts position based on viewport boundaries
   }
   ```

2. **Performance Optimizations**
   - Lazy static initialization for large datasets
   - Efficient filtering algorithms for god/item searches
   - CSS-based animations (no JS overhead)
   - Image lazy loading considerations

3. **Persistence Layer**
   ```rust
   // localStorage integration
   pub fn save_to_storage(key: &str, value: &str) {
       if let Some(storage) = window().local_storage().ok().flatten() {
           storage.set_item(key, value).ok();
       }
   }
   ```

### Dioxus-Specific Features Utilized

1. **Signals & Memoization**
   - Global state management without prop drilling
   - Automatic re-renders on state changes
   - Efficient diff algorithm

2. **Component Lifecycle**
   - use_effect for side effects
   - use_memo for expensive computations
   - Custom hooks for reusable logic

3. **Event Handling**
   ```rust
   // Keyboard shortcuts
   use_effect(move || {
       let handler = Closure::wrap(Box::new(move |e: KeyboardEvent| {
           // Handle letter key -> search focus
       }));
   });
   ```

### Build & Deployment

1. **Development**
   ```bash
   trunk serve --open  # Hot reloading development
   ```

2. **Production Build**
   ```bash
   trunk build --release
   # Outputs optimized WASM + minimal HTML/JS loader
   ```

3. **Output Structure**
   ```
   /dist/
   ├── index.html      # Minimal loader
   ├── grappul_bg.wasm # Main application
   ├── grappul.js      # WASM bindings
   └── /assets/        # Static resources
   ```

### Performance Metrics (Current)
- **WASM Bundle**: ~500KB (uncompressed)
- **Total Assets**: ~15MB (all images)
- **Initial Load**: < 2s on 4G
- **Time to Interactive**: < 1s after WASM load
- **Runtime Performance**: Consistent 60fps

### Technical Challenges & Solutions

1. **Challenge**: Large static data in WASM
   - **Solution**: Lazy static initialization, consider future splitting

2. **Challenge**: Tooltip positioning across browsers
   - **Solution**: web-sys for consistent viewport calculations

3. **Challenge**: Image loading performance
   - **Solution**: Organized asset structure, browser caching

### Future Technical Considerations

1. **Data Management Evolution**
   - Consider external data loading for smaller WASM
   - Implement incremental data updates
   - Add caching strategies

2. **Performance Enhancements**
   - Code splitting for routes
   - Progressive image loading
   - Service worker for offline mode

3. **Feature Additions**
   - WebSocket for real-time features
   - IndexedDB for larger local storage
   - Web Workers for data processing

### Development Tools & Workflow
- **Cargo**: Dependency management
- **rustfmt**: Code formatting
- **Clippy**: Linting
- **Python**: Data pipeline scripts

### Code Quality Metrics
- **Type Safety**: 100% - Rust's type system
- **Memory Safety**: Guaranteed by Rust
- **Bundle Size**: Optimized with release builds
- **Browser Support**: Modern browsers with WASM support

The implementation successfully leverages Rust's performance and safety guarantees while providing a smooth, React-like development experience through Dioxus. The architecture is well-suited for the data-heavy nature of a gaming companion app.