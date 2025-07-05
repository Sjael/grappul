# Grappul - Rust/Dioxus Technical Brief

## Project Overview
Build a modern, performant web application for the Smite gaming community using Rust and Dioxus. The app provides build guides, tier lists, and a quick-reference cheatsheet - essentially a cleaner, more focused alternative to Mobalytics/OP.GG but for Smite.

## Technology Stack
- **Frontend**: Dioxus (latest stable)
- **Backend**: Axum or Actix-web (if needed)
- **Database**: SurrealDB or PostgreSQL (evaluation needed)
- **Language**: Rust (latest stable) throughout

## Core Technical Architecture

### Dioxus Frontend
- **Component Structure**:
  - Reusable components for gods, items, abilities
  - State management using Dioxus hooks and signals
  - Custom hooks for data fetching and caching
- **Routing**: Dioxus Router for SPA navigation
- **Styling**: 
  - CSS modules or styled components approach
  - Tailwind CSS integration if preferred
  - CSS variables for Smite stat color system

### Data Layer Considerations

#### Option 1: SurrealDB
- **Pros**: 
  - Native Rust integration
  - Built-in real-time capabilities for community features
  - Flexible schema for game data evolution
  - Can run embedded for simpler deployment
- **Cons**: 
  - Newer, less battle-tested
  - Smaller ecosystem

#### Option 2: PostgreSQL with SQLx
- **Pros**: 
  - Mature, proven technology
  - Compile-time checked queries with SQLx
  - Extensive tooling and hosting options
- **Cons**: 
  - More traditional setup
  - Requires separate real-time solution

#### Option 3: Static JSON/Bincode Files
- **Consideration**: Since game data is relatively static, consider if a full database is needed
- Could use serialized Rust structs for ultra-fast loading
- User data could be stored separately if minimal

### Rust-Based Data Pipeline
```rust
// Example scraper architecture
struct SmiteScraper {
    client: reqwest::Client,
    parser: scraper::Html,
}

impl SmiteScraper {
    async fn scrape_gods() -> Result<Vec<God>, Error>
    async fn scrape_items() -> Result<Vec<Item>, Error>
    async fn scrape_abilities() -> Result<Vec<Ability>, Error>
}
```

- **Web Scraping**: `reqwest` + `scraper` crates
- **Data Serialization**: Serde with bincode/JSON
- **Image Processing**: `image` crate for optimization
- **Scheduling**: `tokio-cron-scheduler` for patch updates

### Key Technical Implementation

1. **Dynamic Content Enhancement**
   ```rust
   // Text parser for inline icons
   fn parse_and_enhance_text(text: &str, items: &HashMap<String, Item>) -> VNode {
       // Regex or pest parser to identify item/ability names
       // Return Dioxus VNode with embedded icon components
   }
   ```

2. **Performance Optimizations**
   - WASM bundle splitting with `wasm-pack`
   - Lazy loading with Dioxus suspense boundaries
   - Virtual scrolling using `dioxus-virtual-list`
   - Pre-rendered static pages where possible

3. **State Management**
   ```rust
   #[derive(Clone)]
   struct AppState {
       gods: Signal<Vec<God>>,
       current_build: Signal<Option<Build>>,
       user_preferences: Signal<UserPrefs>,
   }
   ```

### Dioxus-Specific Features
- **Server Components**: Use Dioxus fullstack for SSR/hydration
- **Hot Reloading**: Development with `dioxus-cli`
- **Asset Handling**: Integrate with Dioxus asset system
- **Routing**: Type-safe routes with Dioxus Router

### Deployment Architecture
- **Frontend**: 
  - Static WASM files served via CDN
  - Or fullstack Dioxus with server-side rendering
- **Backend** (if needed):
  - Containerized Rust service
  - Fly.io, Railway, or Shuttle.rs for easy Rust deployment
- **Assets**: Cloudflare R2 or similar for images

### Development Workflow
1. **Phase 1**: Static Dioxus app with hardcoded data
2. **Phase 2**: Add scraper and data pipeline
3. **Phase 3**: Implement user features (if using backend)
4. **Phase 4**: Community features and voting

### Rust-Specific Considerations
- **Error Handling**: Use `thiserror` and `anyhow` appropriately
- **Async Runtime**: Tokio throughout
- **Testing**: Unit tests for parsers, integration tests for scrapers
- **CI/CD**: GitHub Actions with Rust cache
- **Code Quality**: Clippy, rustfmt, and cargo-audit in CI

### Performance Targets
- WASM bundle < 500KB (compressed)
- Initial load < 2s on 3G
- 60fps scrolling on cheatsheet page
- Instant navigation with prefetching

### Why Rust/Dioxus for This Project?
- Type safety for complex game data structures
- Performance for handling large datasets client-side
- Single language across entire stack
- Growing Rust gamedev community alignment
- Excellent WASM support for client-side performance

The combination of Rust's performance and Dioxus's reactive model makes this an ideal stack for a data-heavy gaming companion app that needs to feel native-fast while running in the browser.