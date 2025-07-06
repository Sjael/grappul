use dioxus::prelude::*;
use crate::components::{Header, Ability, MarkdownRenderer};
use crate::data::gods::GODS;
use crate::data::items::ITEMS;
use crate::{SelectedGod, SelectedRole};
use crate::utils::format_god_image_name;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
struct TimelineEntry {
    time: String,
    items: Vec<String>,
    description: String,
    progress: f64, // 0.0 to 100.0 representing position on timeline
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct SavedBuild {
    id: String,
    title: String,
    god: String,
    role: String,
    build: Vec<String>,
    relics: Vec<String>,
    timeline: Vec<SavedTimelineEntry>,
    skill_order: Vec<u8>,
    strategy: String,
    created_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct SavedTimelineEntry {
    time: String,
    items: Vec<String>,
    description: String,
    progress: f64,
}

#[derive(Clone, Debug, PartialEq)]
struct GuideData {
    title: String,
    god: String,
    role: String,
    build: Vec<String>,
    relics: Vec<String>,
    timeline: Vec<TimelineEntry>,
    skill_order: Vec<u8>,
    strategy: String,
}

#[component]
pub fn GuideCreator() -> Element {
    // Get context for selected god and role
    let selected_god = use_context::<Signal<SelectedGod>>();
    let selected_role = use_context::<Signal<SelectedRole>>();
    
    // Initialize form data
    let mut guide_data = use_signal(|| {
        GuideData {
            title: String::new(),
            god: selected_god().0.unwrap_or_default(),
            role: selected_role().0.unwrap_or_default(),
            build: vec![],
            relics: vec![],
            timeline: vec![],
            skill_order: vec![],
            strategy: String::new(),
        }
    });
    
    // UI state
    let search_query = use_signal(String::new);
    
    // Get god info
    let god_info = if guide_data().god.is_empty() { 
        None 
    } else { 
        GODS.get(&guide_data().god).cloned()
    };
    
    rsx! {
        div {
            class: "guide-creator-container",
            style: "display: flex; flex-direction: column; height: 100vh; background: var(--color-bg-primary);",
            
            // Header - simplified like cheatsheet
            div {
                class: "guide-creator-header",
                style: "background: var(--color-bg-secondary);",
                
                // Navigation row
                div {
                    style: "padding: 12px 24px; border-bottom: 1px solid var(--color-border);",
                    Header {}
                }
                
                // Sticky control bar with title, god, role, and save button
                div {
                    style: "position: sticky; top: 0; z-index: 100; padding: 16px 24px; display: flex; align-items: center; gap: 16px; background: var(--color-bg-tertiary); border-bottom: 1px solid var(--color-border);",
                    
                    // Title input
                    input {
                        r#type: "text",
                        placeholder: "Guide title...",
                        style: "font-size: 16px; font-weight: 600; background: transparent; border: 1px solid var(--color-border); border-radius: 4px; color: var(--color-text-primary); outline: none; padding: 8px 12px; min-width: 200px;",
                        value: "{guide_data().title}",
                        oninput: move |evt| {
                            let mut data = guide_data();
                            data.title = evt.value();
                            guide_data.set(data);
                        },
                    }
                    
                    // God selector
                    GodSelector { guide_data: guide_data }
                    
                    // Role selector  
                    RoleSelector { guide_data: guide_data }
                    
                    // Spacer
                    div { style: "flex: 1;" }
                    
                    // Save button
                    button {
                        style: "padding: 8px 16px; background: var(--color-accent); border: none; border-radius: 6px; color: white; font-weight: 600; cursor: pointer;",
                        onclick: move |_| {
                            let data = guide_data();
                            let timestamp = js_sys::Date::now() as u64;
                            let id = format!("build_{}", timestamp);
                            
                            let saved_build = SavedBuild {
                                id: id.clone(),
                                title: if data.title.is_empty() { "Untitled Guide".to_string() } else { data.title },
                                god: data.god,
                                role: data.role,
                                build: data.build,
                                relics: data.relics,
                                timeline: data.timeline.into_iter().map(|entry| SavedTimelineEntry {
                                    time: entry.time,
                                    items: entry.items,
                                    description: entry.description,
                                    progress: entry.progress,
                                }).collect(),
                                skill_order: data.skill_order,
                                strategy: data.strategy,
                                created_at: format!("{}", timestamp),
                            };
                            
                            // Save to browser's localStorage as a JSON file equivalent
                            if let Ok(json) = serde_json::to_string_pretty(&saved_build) {
                                // Save to localStorage with the unique ID as key
                                if let Some(window) = web_sys::window() {
                                    if let Ok(Some(storage)) = window.local_storage() {
                                        let _ = storage.set_item(&format!("grappul_build_{}", id), &json);
                                        web_sys::console::log_1(&format!("Build saved with ID: {}", id).into());
                                        
                                        // Also log the JSON for debugging
                                        log::info!("Saved build JSON: {}", json);
                                    }
                                }
                            }
                        },
                        "Save Guide"
                    }
                }
            }
            
            // Main content area - full width without sidebar
            div {
                style: "flex: 1; overflow-y: auto; padding: 24px 24px 400px 24px; scroll-behavior: smooth;",
                
                // Build and Items section
                div { id: "build", BuildAndItemsSection { guide_data: guide_data, search_query: search_query } }
                
                div { style: "margin-top: 48px;", id: "skills", SkillOrderSection { guide_data: guide_data, god_info: god_info } }
                
                // Timeline section - full width
                div { style: "margin-top: 48px;", id: "timeline", TimelineSection { guide_data: guide_data } }
                
                // Strategy Guide section - full width
                div { style: "margin-top: 48px;", id: "strategy", StrategyGuideSection { guide_data: guide_data } }
            }
        }
    }
}


#[component]
fn GodSelector(guide_data: Signal<GuideData>) -> Element {
    let mut available_gods: Vec<_> = GODS.keys().cloned().collect();
    available_gods.sort();
    let mut is_god_hovered = use_signal(|| false);
    let mut search_god = use_signal(String::new);
    
    // Filter gods based on search
    let filtered_gods: Vec<_> = available_gods.iter()
        .filter(|god| {
            search_god().is_empty() || god.to_lowercase().contains(&search_god().to_lowercase())
        })
        .cloned()
        .collect();
    
    rsx! {
        div {
            style: "position: relative;",
            onmouseenter: move |_| is_god_hovered.set(true),
            onmouseleave: move |_| is_god_hovered.set(false),
            
            // Compact god selector
            div {
                style: format!(
                    "width: 40px; height: 40px; border: 2px solid {}; border-radius: 6px; overflow: hidden; cursor: pointer; transition: all 0.2s ease; background: var(--color-bg-tertiary);",
                    if !guide_data().god.is_empty() { "var(--color-accent)" } else { "var(--color-border)" }
                ),
                
                if !guide_data().god.is_empty() {
                    img {
                        src: format!("/assets/gods/{}.png", format_god_image_name(&guide_data().god)),
                        style: "width: 100%; height: 100%; object-fit: cover;",
                    }
                } else {
                    div {
                        style: "width: 100%; height: 100%; display: flex; align-items: center; justify-content: center; color: var(--color-text-secondary); font-size: 10px;",
                        "God"
                    }
                }
            }
            
            // Dropdown panel on hover
            if is_god_hovered() {
                div {
                    style: "position: absolute; top: 100%; left: 0; z-index: 1000; background: var(--color-bg-secondary); border: 1px solid var(--color-border); border-radius: 8px; padding: 16px; box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15); width: 500px; max-height: 400px; margin-top: 4px;",
                    
                    // Search input
                    input {
                        r#type: "text",
                        placeholder: "Search gods...",
                        value: "{search_god}",
                        oninput: move |evt| search_god.set(evt.value()),
                        style: "width: 100%; padding: 8px 12px; border: 1px solid var(--color-border); border-radius: 6px; background: var(--color-bg-primary); margin-bottom: 12px;",
                    }
                    
                    // Gods grid
                    div {
                        style: "display: grid; grid-template-columns: repeat(8, 1fr); gap: 8px; max-height: 300px; overflow-y: auto;",
                        
                        for god in filtered_gods {
                            div {
                                key: "{god}",
                                style: format!(
                                    "aspect-ratio: 1; border-radius: 6px; overflow: hidden; cursor: pointer; border: 2px solid {}; transition: all 0.2s ease;",
                                    if guide_data().god == god { "var(--color-accent)" } else { "transparent" }
                                ),
                                onclick: {
                                    let god_clone = god.clone();
                                    move |_| {
                                        let mut data = guide_data();
                                        data.god = god_clone.clone();
                                        guide_data.set(data);
                                        is_god_hovered.set(false);
                                        search_god.set(String::new());
                                    }
                                },
                                
                                img {
                                    src: format!("/assets/gods/{}.png", format_god_image_name(&god)),
                                    style: "width: 100%; height: 100%; object-fit: cover;",
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RoleSelector(guide_data: Signal<GuideData>) -> Element {
    let roles = vec!["Solo", "Jungle", "Mid", "Support", "Carry"];
    
    rsx! {
        div {
            style: "display: flex; gap: 8px;",
            
            for role in roles {
                button {
                    style: format!(
                        "padding: 6px 12px; border: 1px solid {}; background: {}; color: {}; border-radius: 4px; cursor: pointer; transition: all 0.2s ease; font-size: 14px;",
                        if guide_data().role == role { "var(--color-accent)" } else { "var(--color-border)" },
                        if guide_data().role == role { "var(--color-accent)" } else { "transparent" },
                        if guide_data().role == role { "white" } else { "var(--color-text-primary)" }
                    ),
                    onclick: move |_| {
                        let mut data = guide_data();
                        data.role = role.to_string();
                        guide_data.set(data);
                    },
                    "{role}"
                }
            }
        }
    }
}

#[component]
fn BuildSection(guide_data: Signal<GuideData>, search_query: Signal<String>) -> Element {
    let mut tier1_enabled = use_signal(|| false); // Tier 1 filtered out by default
    let mut tier2_enabled = use_signal(|| false); // Tier 2 filtered out by default
    let mut tier3_enabled = use_signal(|| true);
    let mut dragged_item_index = use_signal(|| None::<usize>);
    
    // Helper function to get the effective price for sorting (evolved items use base price)
    let get_effective_price = |item_name: &str| -> u32 {
        if let Some(item) = ITEMS.get(item_name) {
            // If price is 0, check if it's an evolved item with a base item
            if item.price == 0 {
                // Check if this item has a glyph effect
                for effect in &item.effects {
                    if effect.starts_with("Glyph:") {
                        // For evolved items, use the base item's price
                        // This is a simplification - in the full implementation,
                        // we might extract the base item name from the effect string
                        return 3000; // Default glyph item price
                    }
                }
            }
            item.price
        } else {
            0
        }
    };
    
    // Helper function to determine item tier
    let get_item_tier = |item_name: &str| -> u8 {
        let price = get_effective_price(item_name);
        
        // Starter items (s_ prefix) are always shown regardless of tier filters
        if item_name.starts_with("s_") {
            return 0; // Special tier for starter items
        }
        
        match price {
            0..=999 => 1,      // Tier 1: Under 1000 gold
            1000..=2499 => 2,  // Tier 2: 1000-2499 gold  
            _ => 3,            // Tier 3: 2500+ gold
        }
    };
    
    // List of relics to exclude from build items
    let relics = ["aegis", "beads", "blink", "shell", "ankh", "med", "thorns", "sprint", "teleport", "frenzy"];
    
    // Helper function to check if an item is a glyph
    let is_glyph_item = |item_name: &str| -> bool {
        if let Some(item) = ITEMS.get(item_name) {
            item.effects.iter().any(|effect| effect.starts_with("Glyph:"))
        } else {
            false
        }
    };
    
    // Separate starter items, glyph items, and regular items
    let mut starter_items: Vec<_> = ITEMS.iter()
        .filter(|(item_name, _item_data)| {
            // Apply search filter
            let search_matches = search_query().is_empty() || item_name.contains(&search_query().to_lowercase());
            if !search_matches {
                return false;
            }
            
            // Skip relics
            if relics.contains(&item_name.as_str()) {
                return false;
            }
            
            // Only starter items
            item_name.starts_with("s_")
        })
        .map(|(item_name, _item_data)| (item_name.clone(), get_effective_price(item_name)))
        .collect();
    
    let mut glyph_items: Vec<_> = ITEMS.iter()
        .filter(|(item_name, _item_data)| {
            // Apply search filter
            let search_matches = search_query().is_empty() || item_name.contains(&search_query().to_lowercase());
            if !search_matches {
                return false;
            }
            
            // Skip relics
            if relics.contains(&item_name.as_str()) {
                return false;
            }
            
            // Skip starter items
            if item_name.starts_with("s_") {
                return false;
            }
            
            // Only glyph items
            is_glyph_item(item_name)
        })
        .map(|(item_name, _item_data)| (item_name.clone(), get_effective_price(item_name)))
        .collect();
    
    let mut regular_items: Vec<_> = ITEMS.iter()
        .filter(|(item_name, _item_data)| {
            // Apply search filter
            let search_matches = search_query().is_empty() || item_name.contains(&search_query().to_lowercase());
            if !search_matches {
                return false;
            }
            
            // Skip relics
            if relics.contains(&item_name.as_str()) {
                return false;
            }
            
            // Skip starter items
            if item_name.starts_with("s_") {
                return false;
            }
            
            // Skip glyph items
            if is_glyph_item(item_name) {
                return false;
            }
            
            // Apply tier filter
            let tier = get_item_tier(item_name);
            match tier {
                1 => tier1_enabled(),
                2 => tier2_enabled(),
                3 => tier3_enabled(),
                _ => true,
            }
        })
        .map(|(item_name, _item_data)| (item_name.clone(), get_effective_price(item_name)))
        .collect();
    
    // Sort all by price ascending
    starter_items.sort_by(|a, b| a.1.cmp(&b.1));
    glyph_items.sort_by(|a, b| a.1.cmp(&b.1));
    regular_items.sort_by(|a, b| a.1.cmp(&b.1));
    
    let starter_items: Vec<String> = starter_items.into_iter().map(|(name, _price)| name).collect();
    let glyph_items: Vec<String> = glyph_items.into_iter().map(|(name, _price)| name).collect();
    let regular_items: Vec<String> = regular_items.into_iter().map(|(name, _price)| name).collect();
    
    rsx! {
        div {
            // Section header
            h3 {
                style: "margin: 0 0 16px 0; font-size: 18px; font-weight: 600; color: var(--color-text-primary); border-bottom: 2px solid var(--color-accent); padding-bottom: 8px;",
                "Build Items"
            }
            
            // Selected items
            div {
                style: "margin-bottom: 24px;",
                
                div {
                    style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 12px;",
                    
                    h3 {
                        style: "font-size: 16px; margin: 0;",
                        { format!("Selected Items ({}/6)", guide_data().build.len()) }
                    }
                    
                    button {
                        style: format!(
                            "padding: 4px 8px; background: #dc2626; border: none; border-radius: 4px; color: white; cursor: {}; font-size: 12px; transition: all 0.2s ease; font-weight: 500; opacity: {};",
                            if guide_data().build.is_empty() { "not-allowed" } else { "pointer" },
                            if guide_data().build.is_empty() { "0.3" } else { "1" }
                        ),
                        onclick: move |_| {
                            if !guide_data().build.is_empty() {
                                let mut data = guide_data();
                                data.build.clear();
                                guide_data.set(data);
                            }
                        },
                        "Clear"
                    }
                }
                
                div {
                    style: "display: grid; grid-template-columns: repeat(6, 1fr); gap: 6px; margin-bottom: 16px; max-width: 320px;",
                    
                    for i in 0..6 {
                        if let Some(item) = guide_data().build.get(i) {
                            div {
                                style: "aspect-ratio: 1; position: relative;",
                                draggable: "true",
                                ondragstart: move |_evt: DragEvent| {
                                    dragged_item_index.set(Some(i));
                                },
                                ondragover: move |evt: DragEvent| {
                                    evt.prevent_default(); // Allow drop
                                },
                                ondrop: move |evt: DragEvent| {
                                    evt.prevent_default();
                                    if let Some(dragged_index) = dragged_item_index() {
                                        if dragged_index != i && dragged_index < guide_data().build.len() {
                                            let mut data = guide_data();
                                            let item_to_move = data.build.remove(dragged_index);
                                            
                                            // Adjust target index if we removed an item before it
                                            let target_index = if dragged_index < i { i - 1 } else { i };
                                            
                                            // Insert at the target position, but cap at current length
                                            let insert_index = target_index.min(data.build.len());
                                            data.build.insert(insert_index, item_to_move);
                                            
                                            guide_data.set(data);
                                        }
                                    }
                                    dragged_item_index.set(None);
                                },
                                
                                img {
                                    src: format!("/assets/items/{}.png", item),
                                    style: "width: 100%; height: 100%; border-radius: 4px; background: var(--color-bg-tertiary); cursor: move;",
                                }
                                
                                button {
                                    style: "position: absolute; top: -4px; right: -4px; width: 20px; height: 20px; border-radius: 50%; background: var(--color-accent); border: none; color: white; cursor: pointer; display: flex; align-items: center; justify-content: center;",
                                    onclick: move |_| {
                                        let mut data = guide_data();
                                        data.build.remove(i);
                                        guide_data.set(data);
                                    },
                                    "×"
                                }
                            }
                        } else {
                            div {
                                style: "aspect-ratio: 1; border: 2px dashed var(--color-border); border-radius: 4px; display: flex; align-items: center; justify-content: center; color: var(--color-text-secondary);",
                                ondragover: move |evt: DragEvent| {
                                    evt.prevent_default(); // Allow drop on empty slots
                                },
                                ondrop: move |evt: DragEvent| {
                                    evt.prevent_default();
                                    if let Some(dragged_index) = dragged_item_index() {
                                        if dragged_index < guide_data().build.len() {
                                            let mut data = guide_data();
                                            let item_to_move = data.build.remove(dragged_index);
                                            
                                            // Insert at this empty position
                                            if i <= data.build.len() {
                                                data.build.insert(i, item_to_move);
                                            } else {
                                                data.build.push(item_to_move);
                                            }
                                            
                                            guide_data.set(data);
                                        }
                                    }
                                    dragged_item_index.set(None);
                                },
                                { format!("{}", i + 1) }
                            }
                        }
                    }
                }
            }
            
            // Tier filters
            div {
                style: "margin-bottom: 16px;",
                
                h4 {
                    style: "margin: 0 0 8px 0; font-size: 14px; font-weight: 600;",
                    "Item Tiers"
                }
                
                div {
                    style: "display: flex; gap: 12px; align-items: center;",
                    
                    label {
                        style: "display: flex; align-items: center; gap: 6px; cursor: pointer;",
                        
                        input {
                            r#type: "checkbox",
                            checked: tier1_enabled(),
                            onchange: move |evt| tier1_enabled.set(evt.checked()),
                            style: "margin: 0;",
                        }
                        
                        span {
                            style: "font-size: 13px; color: var(--color-text-primary);",
                            "Tier 1"
                        }
                    }
                    
                    label {
                        style: "display: flex; align-items: center; gap: 6px; cursor: pointer;",
                        
                        input {
                            r#type: "checkbox",
                            checked: tier2_enabled(),
                            onchange: move |evt| tier2_enabled.set(evt.checked()),
                            style: "margin: 0;",
                        }
                        
                        span {
                            style: "font-size: 13px; color: var(--color-text-primary);",
                            "Tier 2"
                        }
                    }
                    
                    label {
                        style: "display: flex; align-items: center; gap: 6px; cursor: pointer;",
                        
                        input {
                            r#type: "checkbox",
                            checked: tier3_enabled(),
                            onchange: move |evt| tier3_enabled.set(evt.checked()),
                            style: "margin: 0;",
                        }
                        
                        span {
                            style: "font-size: 13px; color: var(--color-text-primary);",
                            "Tier 3"
                        }
                    }
                }
            }
            
            // Search
            input {
                r#type: "text",
                placeholder: "Search items...",
                value: "{search_query}",
                oninput: move |evt| search_query.set(evt.value()),
                style: "width: 100%; padding: 8px 12px; border: 1px solid var(--color-border); border-radius: 6px; background: var(--color-bg-primary); margin-bottom: 16px;",
            }
            
            // Starter items section
            if !starter_items.is_empty() {
                div {
                    style: "margin-bottom: 16px;",
                    
                    h4 {
                        style: "margin: 0 0 8px 0; font-size: 14px; font-weight: 600;",
                        "Starter Items"
                    }
                    
                    div {
                        style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(64px, 1fr)); gap: 8px;",
                        
                        for item in starter_items {
                            button {
                                key: "{item}",
                                style: "aspect-ratio: 1; padding: 0; border: 1px solid var(--color-border); border-radius: 4px; cursor: pointer; background: var(--color-bg-secondary); transition: all 0.2s ease;",
                                disabled: {
                                    let build = guide_data().build;
                                    let has_starter = build.iter().any(|i| i.starts_with("s_"));
                                    build.len() >= 6 || build.contains(&item) || (has_starter && item.starts_with("s_"))
                                },
                                onclick: {
                                    let item_clone = item.clone();
                                    move |_| {
                                        if guide_data().build.len() < 6 {
                                            let mut data = guide_data();
                                            // Check if there's already a starter item
                                            let has_starter = data.build.iter().any(|i| i.starts_with("s_"));
                                            
                                            // Allow adding if no starter exists, or if this is a starter item (replacement)
                                            if !has_starter || item_clone.starts_with("s_") {
                                                if !data.build.contains(&item_clone) {
                                                    // If this is a starter item and one already exists, replace it
                                                    if item_clone.starts_with("s_") && has_starter {
                                                        data.build.retain(|i| !i.starts_with("s_"));
                                                    }
                                                    data.build.push(item_clone.clone());
                                                }
                                                guide_data.set(data);
                                            }
                                        }
                                    }
                                },
                                
                                img {
                                    src: format!("/assets/items/{}.png", item),
                                    style: format!("width: 100%; height: 100%; opacity: {};", 
                                        if guide_data().build.contains(&item) { "0.4" } else { "1" }
                                    )
                                }
                            }
                        }
                    }
                }
            }
            
            // Glyph items section
            if !glyph_items.is_empty() {
                div {
                    style: "margin-bottom: 16px;",
                    
                    h4 {
                        style: "margin: 0 0 8px 0; font-size: 14px; font-weight: 600;",
                        "Glyphs"
                    }
                    
                    div {
                        style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(64px, 1fr)); gap: 8px;",
                        
                        for item in glyph_items {
                            button {
                                key: "{item}",
                                style: "aspect-ratio: 1; padding: 0; border: 1px solid var(--color-border); border-radius: 4px; cursor: pointer; background: var(--color-bg-secondary); transition: all 0.2s ease;",
                                disabled: guide_data().build.len() >= 6 || guide_data().build.contains(&item),
                                onclick: {
                                    let item_clone = item.clone();
                                    move |_| {
                                        if guide_data().build.len() < 6 {
                                            let mut data = guide_data();
                                            data.build.push(item_clone.clone());
                                            guide_data.set(data);
                                        }
                                    }
                                },
                                
                                img {
                                    src: format!("/assets/items/{}.png", item),
                                    style: format!("width: 100%; height: 100%; opacity: {};", 
                                        if guide_data().build.contains(&item) { "0.4" } else { "1" }
                                    )
                                }
                            }
                        }
                    }
                }
            }
            
            // Regular items section
            if !regular_items.is_empty() {
                div {
                    h4 {
                        style: "margin: 0 0 8px 0; font-size: 14px; font-weight: 600;",
                        "Items"
                    }
                    
                    div {
                        style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(64px, 1fr)); gap: 8px; max-height: 400px; overflow-y: auto;",
                        
                        for item in regular_items {
                            button {
                                key: "{item}",
                                style: "aspect-ratio: 1; padding: 0; border: 1px solid var(--color-border); border-radius: 4px; cursor: pointer; background: var(--color-bg-secondary); transition: all 0.2s ease;",
                                disabled: guide_data().build.len() >= 6 || guide_data().build.contains(&item),
                                onclick: {
                                    let item_clone = item.clone();
                                    move |_| {
                                        if guide_data().build.len() < 6 {
                                            let mut data = guide_data();
                                            data.build.push(item_clone.clone());
                                            guide_data.set(data);
                                        }
                                    }
                                },
                                
                                img {
                                    src: format!("/assets/items/{}.png", item),
                                    style: format!("width: 100%; height: 100%; opacity: {};", 
                                        if guide_data().build.contains(&item) { "0.4" } else { "1" }
                                    )
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RelicsSection(guide_data: Signal<GuideData>) -> Element {
    let relics = vec!["aegis", "beads", "blink", "shell", "aegis", "ankh"];
    
    rsx! {
        div {
            // Section header
            h3 {
                style: "margin: 0 0 16px 0; font-size: 18px; font-weight: 600; color: var(--color-text-primary); border-bottom: 2px solid var(--color-accent); padding-bottom: 8px;",
                "Relics"
            }
            
            div {
                style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 12px; max-width: 140px; margin-bottom: 24px;",
                
                for i in 0..2 {
                    if let Some(relic) = guide_data().relics.get(i) {
                        div {
                            style: "aspect-ratio: 1; position: relative;",
                            
                            img {
                                src: format!("/assets/items/{}.png", relic),
                                style: "width: 100%; height: 100%; border-radius: 8px; background: var(--color-bg-tertiary);",
                            }
                            
                            button {
                                style: "position: absolute; top: -4px; right: -4px; width: 24px; height: 24px; border-radius: 50%; background: var(--color-accent); border: none; color: white; cursor: pointer;",
                                onclick: move |_| {
                                    let mut data = guide_data();
                                    data.relics.remove(i);
                                    guide_data.set(data);
                                },
                                "×"
                            }
                        }
                    } else {
                        div {
                            style: "aspect-ratio: 1; border: 2px dashed var(--color-border); border-radius: 8px; display: flex; align-items: center; justify-content: center; color: var(--color-text-secondary);",
                            { format!("Relic {}", i + 1) }
                        }
                    }
                }
            }
            
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(60px, 1fr)); gap: 8px; max-width: 400px;",
                
                for relic in relics {
                    button {
                        style: "aspect-ratio: 1; padding: 0; border: 1px solid var(--color-border); border-radius: 8px; cursor: pointer; background: var(--color-bg-secondary);",
                        disabled: guide_data().relics.len() >= 2 || guide_data().relics.contains(&relic.to_string()),
                        onclick: move |_| {
                            if guide_data().relics.len() < 2 {
                                let mut data = guide_data();
                                data.relics.push(relic.to_string());
                                guide_data.set(data);
                            }
                        },
                        
                        img {
                            src: format!("/assets/items/{}.png", relic),
                            style: "width: 100%; height: 100%;",
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn BuildAndItemsSection(guide_data: Signal<GuideData>, search_query: Signal<String>) -> Element {
    let mut tier1_enabled = use_signal(|| false);
    let mut tier2_enabled = use_signal(|| false);
    let mut tier3_enabled = use_signal(|| true);
    let mut dragged_item_index = use_signal(|| None::<usize>);
    
    // Helper function to get the effective price for sorting
    let get_effective_price = |item_name: &str| -> u32 {
        if let Some(item) = ITEMS.get(item_name) {
            if item.price == 0 {
                for effect in &item.effects {
                    if effect.starts_with("Glyph:") {
                        // For glyph items, use a default price since we don't have base item info
                        return 3000;
                    }
                }
            }
            item.price
        } else {
            0
        }
    };
    
    // Helper function to determine item tier
    let get_item_tier = |item_name: &str| -> u8 {
        let price = get_effective_price(item_name);
        if item_name.starts_with("s_") {
            return 0;
        }
        match price {
            0..=999 => 1,
            1000..=2499 => 2,
            _ => 3,
        }
    };
    
    // Helper function to check if an item is a glyph
    let is_glyph_item = |item_name: &str| -> bool {
        if let Some(item) = ITEMS.get(item_name) {
            item.effects.iter().any(|effect| effect.starts_with("Glyph:"))
        } else {
            false
        }
    };
    
    // List of relics (define first so it can be used in helpers)
    let relic_items = vec!["aegis", "beads", "blink", "shell", "ankh", "med", "thorns", "sprint", "teleport", "frenzy"];
    
    // Helper function to check if an item should be hidden (filtered out)
    let should_hide_item = |item_name: &str| -> bool {
        // Apply tier filter for regular items - hide if tier is disabled
        if !item_name.starts_with("s_") && !is_glyph_item(item_name) && !relic_items.iter().any(|relic| relic == &item_name) {
            let tier = get_item_tier(item_name);
            match tier {
                1 => !tier1_enabled(),
                2 => !tier2_enabled(), 
                3 => !tier3_enabled(),
                _ => false,
            }
        } else {
            false
        }
    };
    
    // Helper function to check if an item should be faded (search filtered but still visible)
    let should_fade_item = |item_name: &str| -> bool {
        // Only fade if search doesn't match
        !search_query().is_empty() && !item_name.contains(&search_query().to_lowercase())
    };
    
    // Get all items categorized
    let starter_items: Vec<_> = ITEMS.keys()
        .filter(|item_name| item_name.starts_with("s_"))
        .cloned()
        .collect();
    
    let glyph_items: Vec<_> = ITEMS.keys()
        .filter(|item_name| !item_name.starts_with("s_") && is_glyph_item(item_name))
        .cloned()
        .collect();
    
    let regular_items: Vec<_> = ITEMS.keys()
        .filter(|item_name| {
            !item_name.starts_with("s_") && 
            !is_glyph_item(item_name) &&
            !relic_items.contains(&item_name.as_str())
        })
        .cloned()
        .collect();
    
    let available_relics: Vec<_> = ITEMS.keys()
        .filter(|item_name| relic_items.iter().any(|relic| relic == item_name))
        .cloned()
        .collect();
    
    rsx! {
        div {
            // Section header
            h3 {
                style: "margin: 0 0 16px 0; font-size: 18px; font-weight: 600; color: var(--color-text-primary); border-bottom: 2px solid var(--color-accent); padding-bottom: 8px;",
                "Build & Items"
            }
            
            // Selected build, relics, and filters in same row
            div {
                style: "display: flex; align-items: flex-start; gap: 24px; margin-bottom: 24px; padding: 16px; background: var(--color-bg-secondary); border-radius: 8px;",
                
                // Selected build items
                div {
                    div {
                        style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 12px;",
                        
                        h4 {
                            style: "font-size: 14px; font-weight: 600; margin: 0;",
                            { format!("Build ({}/6)", guide_data().build.len()) }
                        }
                        
                        button {
                            style: format!(
                                "padding: 4px 8px; background: #dc2626; border: none; border-radius: 4px; color: white; cursor: {}; font-size: 12px; transition: all 0.2s ease; font-weight: 500; opacity: {};",
                                if guide_data().build.is_empty() { "not-allowed" } else { "pointer" },
                                if guide_data().build.is_empty() { "0.3" } else { "1" }
                            ),
                            onclick: move |_| {
                                if !guide_data().build.is_empty() {
                                    let mut data = guide_data();
                                    data.build.clear();
                                    guide_data.set(data);
                                }
                            },
                            "Clear"
                        }
                    }
                    
                    // Build slots
                    div {
                        style: "display: grid; grid-template-columns: repeat(6, 64px); gap: 12px;",
                        
                        for i in 0..6 {
                            if let Some(item) = guide_data().build.get(i) {
                                div {
                                    style: "width: 64px; height: 64px; position: relative; box-sizing: border-box;",
                                    draggable: "true",
                                    ondragstart: move |_| dragged_item_index.set(Some(i)),
                                    ondragover: move |evt: DragEvent| evt.prevent_default(),
                                    ondrop: move |evt: DragEvent| {
                                        evt.prevent_default();
                                        if let Some(dragged_index) = dragged_item_index() {
                                            if dragged_index != i && dragged_index < guide_data().build.len() {
                                                let mut data = guide_data();
                                                let item_to_move = data.build.remove(dragged_index);
                                                let target_index = if dragged_index < i { i - 1 } else { i };
                                                let insert_index = target_index.min(data.build.len());
                                                data.build.insert(insert_index, item_to_move);
                                                guide_data.set(data);
                                            }
                                        }
                                        dragged_item_index.set(None);
                                    },
                                    
                                    img {
                                        src: format!("/assets/items/{}.png", item),
                                        style: "width: 100%; height: 100%; border-radius: 4px; background: var(--color-bg-tertiary); cursor: move;",
                                    }
                                    
                                    button {
                                        style: "position: absolute; top: -6px; right: -6px; width: 24px; height: 24px; border-radius: 50%; background: var(--color-accent); border: none; color: white; cursor: pointer; display: flex; align-items: center; justify-content: center; font-size: 14px;",
                                        onclick: move |_| {
                                            let mut data = guide_data();
                                            data.build.remove(i);
                                            guide_data.set(data);
                                        },
                                        "×"
                                    }
                                }
                            } else {
                                div {
                                    style: "width: 64px; height: 64px; border: 2px dashed var(--color-border); border-radius: 4px; display: flex; align-items: center; justify-content: center; color: var(--color-text-secondary); background: var(--color-bg-tertiary); box-sizing: border-box;",
                                    ondragover: move |evt: DragEvent| evt.prevent_default(),
                                    ondrop: move |evt: DragEvent| {
                                        evt.prevent_default();
                                        if let Some(dragged_index) = dragged_item_index() {
                                            if dragged_index < guide_data().build.len() {
                                                let mut data = guide_data();
                                                let item_to_move = data.build.remove(dragged_index);
                                                if i <= data.build.len() {
                                                    data.build.insert(i, item_to_move);
                                                } else {
                                                    data.build.push(item_to_move);
                                                }
                                                guide_data.set(data);
                                            }
                                        }
                                        dragged_item_index.set(None);
                                    },
                                    { format!("{}", i + 1) }
                                }
                            }
                        }
                    }
                }
                
                // Selected relics
                div {
                    div {
                        style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 12px;",
                        
                        h4 {
                            style: "font-size: 14px; font-weight: 600; margin: 0;",
                            { format!("Relics ({}/2)", guide_data().relics.len()) }
                        }
                        
                        button {
                            style: format!(
                                "padding: 4px 8px; background: #dc2626; border: none; border-radius: 4px; color: white; cursor: {}; font-size: 12px; transition: all 0.2s ease; font-weight: 500; opacity: {};",
                                if guide_data().relics.is_empty() { "not-allowed" } else { "pointer" },
                                if guide_data().relics.is_empty() { "0.3" } else { "1" }
                            ),
                            onclick: move |_| {
                                if !guide_data().relics.is_empty() {
                                    let mut data = guide_data();
                                    data.relics.clear();
                                    guide_data.set(data);
                                }
                            },
                            "Clear"
                        }
                    }
                    
                    // Relic slots
                    div {
                        style: "display: grid; grid-template-columns: repeat(2, 64px); gap: 12px;",
                        
                        for i in 0..2 {
                            if let Some(relic) = guide_data().relics.get(i) {
                                div {
                                    style: "width: 64px; height: 64px; position: relative; box-sizing: border-box;",
                                    
                                    img {
                                        src: format!("/assets/items/{}.png", relic),
                                        style: "width: 100%; height: 100%; border-radius: 4px; background: var(--color-bg-tertiary);",
                                    }
                                    
                                    button {
                                        style: "position: absolute; top: -6px; right: -6px; width: 24px; height: 24px; border-radius: 50%; background: var(--color-accent); border: none; color: white; cursor: pointer; display: flex; align-items: center; justify-content: center; font-size: 14px;",
                                        onclick: move |_| {
                                            let mut data = guide_data();
                                            data.relics.remove(i);
                                            guide_data.set(data);
                                        },
                                        "×"
                                    }
                                }
                            } else {
                                div {
                                    style: "width: 64px; height: 64px; border: 2px dashed var(--color-border); border-radius: 4px; display: flex; align-items: center; justify-content: center; color: var(--color-text-secondary); font-size: 12px; background: var(--color-bg-tertiary); box-sizing: border-box;",
                                    { format!("R{}", i + 1) }
                                }
                            }
                        }
                    }
                    
                    // Total cost display
                    if !guide_data().build.is_empty() {
                        div {
                            style: "margin-top: 12px; padding-top: 12px; border-top: 1px solid var(--color-border);",
                            
                            div {
                                style: "display: flex; align-items: center; justify-content: space-between;",
                                
                                span {
                                    style: "font-size: 14px; font-weight: 600; color: var(--color-text-secondary);",
                                    "Total Cost:"
                                }
                                
                                span {
                                    style: "font-size: 16px; font-weight: 700; color: var(--color-accent);",
                                    {
                                        let total_cost: u32 = guide_data().build.iter()
                                            .filter_map(|item_name| ITEMS.get(item_name))
                                            .map(|item| {
                                                // For evolved items with 0 price, get base item price
                                                if item.price == 0 {
                                                    for effect in &item.effects {
                                                        if effect.starts_with("Glyph:") {
                                                            // For glyph items, use a default price
                                                            return 3000;
                                                        }
                                                    }
                                                }
                                                item.price
                                            })
                                            .sum();
                                        {
                                            let cost_str = total_cost.to_string();
                                            let chars: Vec<char> = cost_str.chars().collect();
                                            let mut result = String::new();
                                            let len = chars.len();
                                            for (i, ch) in chars.iter().enumerate() {
                                                if i > 0 && (len - i) % 3 == 0 {
                                                    result.push(',');
                                                }
                                                result.push(*ch);
                                            }
                                            format!("{}g", result)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                    
                // Tier filters
                div {
                    h4 {
                        style: "margin: 0 0 8px 0; font-size: 14px; font-weight: 600;",
                        "Tiers"
                    }
                    
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        
                        label {
                            style: "display: flex; align-items: center; gap: 6px; cursor: pointer;",
                            input {
                                r#type: "checkbox",
                                checked: tier1_enabled(),
                                onchange: move |evt| tier1_enabled.set(evt.checked()),
                            }
                            span { style: "font-size: 13px;", "Tier 1" }
                        }
                        
                        label {
                            style: "display: flex; align-items: center; gap: 6px; cursor: pointer;",
                            input {
                                r#type: "checkbox",
                                checked: tier2_enabled(),
                                onchange: move |evt| tier2_enabled.set(evt.checked()),
                            }
                            span { style: "font-size: 13px;", "Tier 2" }
                        }
                        
                        label {
                            style: "display: flex; align-items: center; gap: 6px; cursor: pointer;",
                            input {
                                r#type: "checkbox",
                                checked: tier3_enabled(),
                                onchange: move |evt| tier3_enabled.set(evt.checked()),
                            }
                            span { style: "font-size: 13px;", "Tier 3" }
                        }
                    }
                }
                
                // Search
                div {
                    h4 {
                        style: "margin: 0 0 8px 0; font-size: 14px; font-weight: 600;",
                        "Search"
                    }
                    
                    input {
                        r#type: "text",
                        placeholder: "Search items...",
                        value: "{search_query}",
                        oninput: move |evt| search_query.set(evt.value()),
                        style: "width: 100%; padding: 6px 8px; border: 1px solid var(--color-border); border-radius: 4px; background: var(--color-bg-primary); font-size: 13px;",
                    }
                }
            }
            
            // All items in separate rows
            div {
                style: "display: flex; flex-direction: column; gap: 16px; margin-top: 24px;",
                
                // Row 1: Regular items
                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(48px, 1fr)); gap: 6px;",
                    
                    for item in regular_items {
                        if !should_hide_item(&item) {
                            button {
                                key: "{item}",
                                style: format!(
                                    "aspect-ratio: 1; padding: 0; border: 2px solid {}; border-radius: 4px; cursor: pointer; background: var(--color-bg-secondary); transition: all 0.2s ease; opacity: {};",
                                    if guide_data().build.contains(&item) { "var(--color-accent)" } else { "var(--color-border)" },
                                    if should_fade_item(&item) { "0.3" } else { "1" }
                                ),
                                disabled: guide_data().build.len() >= 6 && !guide_data().build.contains(&item),
                                onclick: {
                                    let item_clone = item.clone();
                                    move |_| {
                                        let mut data = guide_data();
                                        if let Some(pos) = data.build.iter().position(|x| x == &item_clone) {
                                            // Item is in build, remove it
                                            data.build.remove(pos);
                                        } else if data.build.len() < 6 {
                                            // Item not in build, add it
                                            data.build.push(item_clone.clone());
                                        }
                                        guide_data.set(data);
                                    }
                                },
                                
                                img {
                                    src: format!("/assets/items/{}.png", item),
                                    style: "width: 100%; height: 100%;",
                                }
                            }
                        }
                    }
                }
                
                // Row 2: Glyph items
                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(48px, 1fr)); gap: 6px;",
                    
                    for item in glyph_items {
                        if !should_hide_item(&item) {
                            button {
                                key: "{item}",
                                style: format!(
                                    "aspect-ratio: 1; padding: 0; border: 2px solid {}; border-radius: 4px; cursor: pointer; background: var(--color-bg-secondary); transition: all 0.2s ease; opacity: {};",
                                    if guide_data().build.contains(&item) { "var(--color-accent)" } else { "var(--color-border)" },
                                    if should_fade_item(&item) { "0.3" } else { "1" }
                                ),
                                disabled: guide_data().build.len() >= 6 && !guide_data().build.contains(&item),
                                onclick: {
                                    let item_clone = item.clone();
                                    move |_| {
                                        let mut data = guide_data();
                                        if let Some(pos) = data.build.iter().position(|x| x == &item_clone) {
                                            // Item is in build, remove it
                                            data.build.remove(pos);
                                        } else if data.build.len() < 6 {
                                            // Check if this is a glyph and if its base item is in the build
                                            if let Some(item_data) = ITEMS.get(&item_clone) {
                                                for effect in &item_data.effects {
                                                    if effect.starts_with("Glyph:") {
                                                        // For glyph items, just add them normally
                                                        // In a full implementation, we might parse the base item from the effect string
                                                        break;
                                                    }
                                                }
                                            }
                                            // No base item found, add normally
                                            data.build.push(item_clone.clone());
                                        }
                                        guide_data.set(data);
                                    }
                                },
                                
                                img {
                                    src: format!("/assets/items/{}.png", item),
                                    style: "width: 100%; height: 100%;",
                                }
                            }
                        }
                    }
                }
                
                // Row 3: Starter items and relics side by side
                div {
                    style: "display: grid; grid-template-columns: 2fr 1fr; gap: 24px;",
                    
                    // Starter items
                    div {
                        style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(48px, 1fr)); gap: 6px;",
                        
                        for item in starter_items {
                            if !should_hide_item(&item) {
                                button {
                                    key: "{item}",
                                    style: format!(
                                        "aspect-ratio: 1; padding: 0; border: 2px solid {}; border-radius: 4px; cursor: pointer; background: var(--color-bg-secondary); transition: all 0.2s ease; opacity: {};",
                                        if guide_data().build.contains(&item) { "var(--color-accent)" } else { "var(--color-border)" },
                                        if should_fade_item(&item) { "0.3" } else { "1" }
                                    ),
                                    disabled: {
                                        let build = guide_data().build;
                                        let has_starter = build.iter().any(|i| i.starts_with("s_"));
                                        build.len() >= 6 && !build.contains(&item) && (has_starter && item.starts_with("s_"))
                                    },
                                    onclick: {
                                        let item_clone = item.clone();
                                        move |_| {
                                            let mut data = guide_data();
                                            if let Some(pos) = data.build.iter().position(|x| x == &item_clone) {
                                                // Item is in build, remove it
                                                data.build.remove(pos);
                                            } else if data.build.len() < 6 {
                                                let has_starter = data.build.iter().any(|i| i.starts_with("s_"));
                                                
                                                if !has_starter || item_clone.starts_with("s_") {
                                                    if item_clone.starts_with("s_") && has_starter {
                                                        data.build.retain(|i| !i.starts_with("s_"));
                                                    }
                                                    data.build.push(item_clone.clone());
                                                }
                                            }
                                            guide_data.set(data);
                                        }
                                    },
                                    
                                    img {
                                        src: format!("/assets/items/{}.png", item),
                                        style: "width: 100%; height: 100%;",
                                    }
                                }
                            }
                        }
                    }
                    
                    // Available relics
                    div {
                        style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(48px, 1fr)); gap: 6px;",
                        
                        for relic in available_relics {
                            if !should_hide_item(&relic) {
                                button {
                                    key: "{relic}",
                                    style: format!(
                                        "aspect-ratio: 1; padding: 0; border: 2px solid {}; border-radius: 4px; cursor: pointer; background: var(--color-bg-secondary); transition: all 0.2s ease; opacity: {};",
                                        if guide_data().relics.contains(&relic) { "var(--color-accent)" } else { "var(--color-border)" },
                                        if should_fade_item(&relic) { "0.3" } else { "1" }
                                    ),
                                    disabled: guide_data().relics.len() >= 2 && !guide_data().relics.contains(&relic),
                                    onclick: {
                                        let relic_clone = relic.clone();
                                        move |_| {
                                            let mut data = guide_data();
                                            if let Some(pos) = data.relics.iter().position(|x| x == &relic_clone) {
                                                // Relic is selected, remove it
                                                data.relics.remove(pos);
                                            } else if data.relics.len() < 2 {
                                                // Relic not selected, add it
                                                data.relics.push(relic_clone.clone());
                                            }
                                            guide_data.set(data);
                                        }
                                    },
                                    
                                    img {
                                        src: format!("/assets/items/{}.png", relic),
                                        style: "width: 100%; height: 100%;",
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TimelineSection(guide_data: Signal<GuideData>) -> Element {
    // Helper to get mouse position relative to timeline
    let get_timeline_progress = |client_x: f64, timeline_rect: web_sys::DomRect| -> f64 {
        let relative_x = client_x - timeline_rect.left();
        let progress = (relative_x / timeline_rect.width()) * 100.0;
        progress.max(0.0).min(100.0)
    };
    
    rsx! {
        div {
            // Section header
            h3 {
                style: "margin: 0 0 16px 0; font-size: 18px; font-weight: 600; color: var(--color-text-primary); border-bottom: 2px solid var(--color-accent); padding-bottom: 8px;",
                "Timeline"
            }
            
            // Interactive timeline
            div {
                style: "margin-bottom: 24px;",
                
                // Timeline bar
                div {
                    id: "timeline-bar",
                    style: "position: relative; height: 80px; background: var(--color-bg-secondary); border-radius: 8px; border: 2px dashed var(--color-border); cursor: pointer; margin-bottom: 16px;",
                    onclick: move |evt| {
                        // Get timeline element
                        if let Some(win) = web_sys::window() {
                            if let Some(doc) = win.document() {
                                if let Some(timeline_element) = doc.get_element_by_id("timeline-bar") {
                                    let rect = timeline_element.get_bounding_client_rect();
                                    let progress = get_timeline_progress(evt.data().client_coordinates().x as f64, rect);
                                    
                                    // Create new checkpoint
                                    let mut data = guide_data();
                                    data.timeline.push(TimelineEntry {
                                        time: format!("{}%", progress as u32),
                                        items: vec![],
                                        description: "New checkpoint".to_string(),
                                        progress,
                                    });
                                    
                                    // Sort by progress
                                    data.timeline.sort_by(|a, b| a.progress.partial_cmp(&b.progress).unwrap());
                                    guide_data.set(data);
                                }
                            }
                        }
                    },
                    
                    // Timeline line
                    div {
                        style: "position: absolute; top: 50%; left: 10%; right: 10%; height: 4px; background: var(--color-border); border-radius: 2px; transform: translateY(-50%);",
                    }
                    
                    // Timeline checkpoints
                    for (idx, entry) in guide_data().timeline.iter().enumerate() {
                        div {
                            key: "{idx}",
                            style: format!(
                                "position: absolute; left: {}%; top: 50%; transform: translate(-50%, -50%); width: 20px; height: 20px; background: var(--color-accent); border: 3px solid white; border-radius: 50%; cursor: pointer; z-index: 10;",
                                entry.progress.max(10.0).min(90.0) // Keep checkpoints within the visible timeline
                            ),
                            onclick: move |evt: Event<MouseData>| {
                                evt.stop_propagation();
                            },
                            title: "{entry.time}: {entry.description}",
                        }
                    }
                    
                    // Instructions overlay when empty
                    if guide_data().timeline.is_empty() {
                        div {
                            style: "position: absolute; inset: 0; display: flex; align-items: center; justify-content: center; color: var(--color-text-secondary); font-size: 14px; pointer-events: none;",
                            "Click anywhere on the timeline to add a checkpoint"
                        }
                    }
                }
                
                // Timeline legend
                div {
                    style: "display: flex; justify-content: space-between; color: var(--color-text-secondary); font-size: 12px; margin-bottom: 16px;",
                    span { "Early Game" }
                    span { "Mid Game" }
                    span { "Late Game" }
                }
            }
            
            // Checkpoint details list
            if !guide_data().timeline.is_empty() {
                div {
                    style: "margin-bottom: 24px;",
                    
                    h4 {
                        style: "margin: 0 0 12px 0; font-size: 16px; font-weight: 600;",
                        "Checkpoints"
                    }
                    
                    for (idx, entry) in guide_data().timeline.iter().enumerate() {
                        div {
                            key: "{idx}",
                            style: "padding: 12px; background: var(--color-bg-secondary); border-radius: 6px; margin-bottom: 8px; position: relative;",
                            
                            // Always editable mode
                            div {
                                style: "display: grid; gap: 12px;",
                                
                                input {
                                    r#type: "text",
                                    value: "{entry.time}",
                                    oninput: move |evt| {
                                        let mut data = guide_data();
                                        if let Some(e) = data.timeline.get_mut(idx) {
                                            e.time = evt.value();
                                        }
                                        guide_data.set(data);
                                    },
                                    style: "padding: 6px 8px; border: 1px solid var(--color-border); border-radius: 4px; background: var(--color-bg-primary);",
                                    placeholder: "Time/phase (e.g., '5 mins', 'Early Game')",
                                }
                                
                                textarea {
                                    value: "{entry.description}",
                                    oninput: move |evt| {
                                        let mut data = guide_data();
                                        if let Some(e) = data.timeline.get_mut(idx) {
                                            e.description = evt.value();
                                        }
                                        guide_data.set(data);
                                    },
                                    style: "padding: 6px 8px; border: 1px solid var(--color-border); border-radius: 4px; background: var(--color-bg-primary); min-height: 60px; resize: vertical;",
                                    placeholder: "Description of what to do at this checkpoint",
                                }
                                
                                // Remove button in the corner
                                button {
                                    style: "position: absolute; top: 8px; right: 8px; width: 24px; height: 24px; background: #dc2626; border: none; border-radius: 50%; color: white; cursor: pointer; font-size: 16px; display: flex; align-items: center; justify-content: center;",
                                    onclick: move |_| {
                                        let mut data = guide_data();
                                        data.timeline.remove(idx);
                                        guide_data.set(data);
                                    },
                                    "×"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SkillOrderSection(guide_data: Signal<GuideData>, god_info: Option<crate::data::gods::God>) -> Element {
    // Helper function to check if a skill can be leveled at a specific level
    let can_level_skill = |skill_idx: usize, level: usize, current_order: &[u8]| -> bool {
        let level_1based = level + 1; // Convert to 1-based level
        
        // Count current ranks in each skill
        let mut skill_ranks = [0; 4];
        for &skill in current_order.iter() {
            if skill > 0 && skill <= 4 {
                skill_ranks[skill as usize - 1] += 1;
            }
        }
        
        // Total points allocated so far
        let total_points: u32 = skill_ranks.iter().sum();
        
        // Can't allocate more points than levels
        if total_points >= level_1based as u32 {
            return false;
        }
        
        let current_rank = skill_ranks[skill_idx];
        
        // All skills max at rank 5
        if current_rank >= 5 {
            return false;
        }
        
        // Check rank caps based on level
        let max_rank_for_level = match level_1based {
            1..=2 => 1,   // Can only have 1 point in abilities at levels 1-2
            3..=4 => 2,   // Can have 2 points in abilities at levels 3-4
            5..=8 => 3,   // Can have 3 points in abilities at levels 5-8
            9..=14 => 4,  // Can have 4 points in abilities at levels 9-14
            15..=20 => 5, // Can have 5 points in abilities at levels 15+
            _ => 0,
        };
        
        // Special case for ultimate: additional restrictions on when it can be leveled
        if skill_idx == 3 { // Ultimate (skill 4)
            let max_ult_rank_for_level = match level_1based {
                1..=4 => 0,   // Can't level ultimate until level 5
                5..=8 => 1,   // Max 1 rank until level 9
                9..=12 => 2,  // Max 2 ranks until level 13
                13..=16 => 3, // Max 3 ranks until level 17
                17..=19 => 4, // Max 4 ranks until level 20
                20 => 5,      // Max 5 ranks at level 20
                _ => 0,
            };
            
            // Ultimate has its own stricter cap
            if current_rank >= max_ult_rank_for_level {
                return false;
            }
        }
        
        // Check if this skill would exceed the rank cap for this level
        if current_rank >= max_rank_for_level {
            return false;
        }
        
        true
    };

    rsx! {
        div {
            // Section header with clear button
            div {
                style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 16px;",
                
                h3 {
                    style: "margin: 0; font-size: 18px; font-weight: 600; color: var(--color-text-primary); border-bottom: 2px solid var(--color-accent); padding-bottom: 8px; flex: 1;",
                    "Skill Order"
                }
                
                button {
                    style: format!(
                        "padding: 4px 8px; background: #dc2626; border: none; border-radius: 4px; color: white; cursor: {}; font-size: 12px; margin-left: 16px; transition: all 0.2s ease; font-weight: 500; opacity: {};",
                        if guide_data().skill_order.is_empty() { "not-allowed" } else { "pointer" },
                        if guide_data().skill_order.is_empty() { "0.3" } else { "1" }
                    ),
                    onclick: move |_| {
                        if !guide_data().skill_order.is_empty() {
                            let mut data = guide_data();
                            data.skill_order.clear();
                            guide_data.set(data);
                        }
                    },
                    "Clear"
                }
            }
            
            if let Some(god) = god_info {
                div {
                    
                    div {
                        style: "display: grid; gap: 8px;",
                        
                        // Header row
                        div {
                            style: "display: grid; grid-template-columns: 40px repeat(20, 1fr); gap: 4px;",
                            
                            div {} // Empty corner
                            
                            for i in 1..=20 {
                                div {
                                    style: "text-align: center; font-size: 12px; color: var(--color-text-secondary);",
                                    "{i}"
                                }
                            }
                        }
                        
                        // Skill rows
                        for (skill_idx, ability) in god.abilities.iter().enumerate() {
                            div {
                                style: "display: grid; grid-template-columns: 40px repeat(20, 1fr); gap: 4px;",
                                
                                Ability { ab: ability.clone(), size: 32 }
                                
                                for level in 0..20 {
                                    button {
                                        style: {
                                            // Count how many times this skill appears at or before this level
                                            let skill_count_at_level = guide_data().skill_order.iter()
                                                .take(level + 1)
                                                .filter(|&&s| s == (skill_idx as u8 + 1))
                                                .count();
                                            // Count how many times this skill appears before this level
                                            let skill_count_before = if level > 0 {
                                                guide_data().skill_order.iter()
                                                    .take(level)
                                                    .filter(|&&s| s == (skill_idx as u8 + 1))
                                                    .count()
                                            } else { 0 };
                                            // This level is selected if the count increased
                                            let is_selected = skill_count_at_level > skill_count_before;
                                            let can_select = can_level_skill(skill_idx, level, &guide_data().skill_order);
                                            
                                            format!(
                                                "aspect-ratio: 1; border: 1px solid var(--color-border); border-radius: 4px; background: {}; color: {}; cursor: {}; font-size: 12px; opacity: {};",
                                                if is_selected {
                                                    "var(--color-accent)"
                                                } else if can_select {
                                                    "var(--color-bg-tertiary)"
                                                } else {
                                                    "var(--color-bg-secondary)"
                                                },
                                                if is_selected {
                                                    "white"
                                                } else if can_select {
                                                    "var(--color-text-secondary)"
                                                } else {
                                                    "var(--color-text-disabled)"
                                                },
                                                if can_select || is_selected { "pointer" } else { "not-allowed" },
                                                if can_select || is_selected { "1" } else { "0.5" }
                                            )
                                        },
                                        disabled: {
                                            // Count how many times this skill appears at or before this level
                                            let skill_count_at_level = guide_data().skill_order.iter()
                                                .take(level + 1)
                                                .filter(|&&s| s == (skill_idx as u8 + 1))
                                                .count();
                                            // Count how many times this skill appears before this level
                                            let skill_count_before = if level > 0 {
                                                guide_data().skill_order.iter()
                                                    .take(level)
                                                    .filter(|&&s| s == (skill_idx as u8 + 1))
                                                    .count()
                                            } else { 0 };
                                            let is_selected = skill_count_at_level > skill_count_before;
                                            let can_select = can_level_skill(skill_idx, level, &guide_data().skill_order);
                                            !can_select && !is_selected
                                        },
                                        onclick: move |_| {
                                            let can_select = can_level_skill(skill_idx, level, &guide_data().skill_order);
                                            
                                            let mut data = guide_data();
                                            
                                            // Count points up to this level to see if this skill is selected here
                                            let skill_count_at_level = data.skill_order.iter()
                                                .take(level + 1)
                                                .filter(|&&s| s == (skill_idx as u8 + 1))
                                                .count();
                                            let skill_count_before = if level > 0 {
                                                data.skill_order.iter()
                                                    .take(level)
                                                    .filter(|&&s| s == (skill_idx as u8 + 1))
                                                    .count()
                                            } else { 0 };
                                            let is_selected_here = skill_count_at_level > skill_count_before;
                                            
                                            if is_selected_here {
                                                // Find and remove the first occurrence of this skill after position 'level-1'
                                                for i in level..data.skill_order.len() {
                                                    if data.skill_order[i] == (skill_idx as u8 + 1) {
                                                        data.skill_order.remove(i);
                                                        break;
                                                    }
                                                }
                                            } else if can_select {
                                                // Add the skill point at this level
                                                data.skill_order.push(skill_idx as u8 + 1);
                                            }
                                            
                                            guide_data.set(data);
                                        },
                                        
                                        {
                                            // Count how many times this skill appears at or before this level
                                            let skill_count_at_level = guide_data().skill_order.iter()
                                                .take(level + 1)
                                                .filter(|&&s| s == (skill_idx as u8 + 1))
                                                .count();
                                            // Count how many times this skill appears before this level
                                            let skill_count_before = if level > 0 {
                                                guide_data().skill_order.iter()
                                                    .take(level)
                                                    .filter(|&&s| s == (skill_idx as u8 + 1))
                                                    .count()
                                            } else { 0 };
                                            // This level is selected if the count increased
                                            if skill_count_at_level > skill_count_before {
                                                "●"
                                            } else {
                                                ""
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                p {
                    style: "color: var(--color-text-secondary);",
                    "Please select a god first"
                }
            }
        }
    }
}
#[component]
fn StrategyGuideSection(guide_data: Signal<GuideData>) -> Element {
    let mut preview_mode = use_signal(|| false);
    
    rsx! {
        div {
            // Section header with toggle
            div {
                style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 16px;",
                
                h3 {
                    style: "margin: 0; font-size: 18px; font-weight: 600; color: var(--color-text-primary); border-bottom: 2px solid var(--color-accent); padding-bottom: 8px; flex: 1;",
                    "Strategy Guide"
                }
                
                // Toggle buttons
                div {
                    style: "display: flex; gap: 4px; margin-left: 16px;",
                    
                    button {
                        style: format!(
                            "padding: 6px 12px; background: {}; border: 1px solid var(--color-border); border-radius: 4px 0 0 4px; color: {}; cursor: pointer; font-size: 13px; font-weight: {};",
                            if !preview_mode() { "var(--color-accent)" } else { "transparent" },
                            if !preview_mode() { "white" } else { "var(--color-text-secondary)" },
                            if !preview_mode() { "600" } else { "400" }
                        ),
                        onclick: move |_| preview_mode.set(false),
                        "✏️ Edit"
                    }
                    
                    button {
                        style: format!(
                            "padding: 6px 12px; background: {}; border: 1px solid var(--color-border); border-radius: 0 4px 4px 0; border-left: none; color: {}; cursor: pointer; font-size: 13px; font-weight: {};",
                            if preview_mode() { "var(--color-accent)" } else { "transparent" },
                            if preview_mode() { "white" } else { "var(--color-text-secondary)" },
                            if preview_mode() { "600" } else { "400" }
                        ),
                        onclick: move |_| preview_mode.set(true),
                        "👁️ Preview"
                    }
                }
            }
            
            if preview_mode() {
                // Preview mode - render markdown
                div {
                    style: "padding: 16px; background: var(--color-bg-secondary); border-radius: 8px; min-height: 200px;",
                    
                    if guide_data().strategy.is_empty() {
                        p {
                            style: "color: var(--color-text-secondary); font-style: italic;",
                            "No strategy guide written yet."
                        }
                    } else {
                        MarkdownRenderer { content: guide_data().strategy.clone() }
                    }
                }
            } else {
                // Edit mode
                div {
                    // Formatting toolbar
                    div {
                        style: "display: flex; gap: 4px; margin-bottom: 8px; padding: 8px; background: var(--color-bg-secondary); border-radius: 4px;",
                        
                        button {
                            style: "padding: 4px 8px; background: var(--color-bg-tertiary); border: 1px solid var(--color-border); border-radius: 3px; color: var(--color-text-primary); cursor: pointer; font-weight: bold; font-size: 12px;",
                            onclick: move |_| {
                                let mut data = guide_data();
                                data.strategy.push_str("**bold text**");
                                guide_data.set(data);
                            },
                            "B"
                        }
                        
                        button {
                            style: "padding: 4px 8px; background: var(--color-bg-tertiary); border: 1px solid var(--color-border); border-radius: 3px; color: var(--color-text-primary); cursor: pointer; font-style: italic; font-size: 12px;",
                            onclick: move |_| {
                                let mut data = guide_data();
                                data.strategy.push_str("*italic text*");
                                guide_data.set(data);
                            },
                            "I"
                        }
                        
                        div { style: "width: 1px; background: var(--color-border); margin: 0 4px;" }
                        
                        button {
                            style: "padding: 4px 8px; background: var(--color-bg-tertiary); border: 1px solid var(--color-border); border-radius: 3px; color: var(--color-text-primary); cursor: pointer; font-size: 12px;",
                            onclick: move |_| {
                                let mut data = guide_data();
                                data.strategy.push_str("\n# Header");
                                guide_data.set(data);
                            },
                            "H1"
                        }
                        
                        button {
                            style: "padding: 4px 8px; background: var(--color-bg-tertiary); border: 1px solid var(--color-border); border-radius: 3px; color: var(--color-text-primary); cursor: pointer; font-size: 12px;",
                            onclick: move |_| {
                                let mut data = guide_data();
                                data.strategy.push_str("\n## Header");
                                guide_data.set(data);
                            },
                            "H2"
                        }
                        
                        button {
                            style: "padding: 4px 8px; background: var(--color-bg-tertiary); border: 1px solid var(--color-border); border-radius: 3px; color: var(--color-text-primary); cursor: pointer; font-size: 12px;",
                            onclick: move |_| {
                                let mut data = guide_data();
                                data.strategy.push_str("\n### Header");
                                guide_data.set(data);
                            },
                            "H3"
                        }
                        
                        div { style: "width: 1px; background: var(--color-border); margin: 0 4px;" }
                        
                        button {
                            style: "padding: 4px 8px; background: var(--color-bg-tertiary); border: 1px solid var(--color-border); border-radius: 3px; color: var(--color-text-primary); cursor: pointer; font-size: 12px;",
                            onclick: move |_| {
                                let mut data = guide_data();
                                data.strategy.push_str("\n- List item");
                                guide_data.set(data);
                            },
                            "• List"
                        }
                        
                        button {
                            style: "padding: 4px 8px; background: var(--color-bg-tertiary); border: 1px solid var(--color-border); border-radius: 3px; color: var(--color-text-primary); cursor: pointer; font-size: 12px;",
                            onclick: move |_| {
                                let mut data = guide_data();
                                data.strategy.push_str("\n1. Numbered item");
                                guide_data.set(data);
                            },
                            "1. List"
                        }
                        
                        div { style: "width: 1px; background: var(--color-border); margin: 0 4px;" }
                        
                        button {
                            style: "padding: 4px 8px; background: var(--color-bg-tertiary); border: 1px solid var(--color-border); border-radius: 3px; color: var(--color-text-primary); cursor: pointer; font-family: monospace; font-size: 12px;",
                            onclick: move |_| {
                                let mut data = guide_data();
                                data.strategy.push_str("`code`");
                                guide_data.set(data);
                            },
                            "<>"
                        }
                    }
                    
                    // Markdown editor
                    textarea {
                        value: "{guide_data().strategy}",
                        oninput: move |evt| {
                            let mut data = guide_data();
                            data.strategy = evt.value();
                            guide_data.set(data);
                        },
                        style: "width: 100%; min-height: 400px; padding: 12px; border: 1px solid var(--color-border); border-radius: 4px; background: var(--color-bg-primary); font-family: monospace; font-size: 14px; line-height: 1.6; resize: vertical;",
                        placeholder: "Write your strategy guide here...\n\nYou can use markdown formatting:\n- **Bold** text with **text**\n- *Italic* text with *text*\n- Headers with #, ##, ###\n- Bullet lists with - or *\n- Numbered lists with 1., 2., etc.\n- Code with `code`\n\nMention items like transcendence or abilities like crushing_wave and they'll automatically show icons!"
                    }
                    
                    // Help text
                    p {
                        style: "margin-top: 8px; font-size: 12px; color: var(--color-text-secondary);",
                        "💡 Tip: Mention any item or ability name and it will automatically display with its icon when previewed!"
                    }
                }
            }
        }
    }
}