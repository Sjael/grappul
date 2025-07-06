use web_sys::window;

pub const ATTR_FLIP: [&str; 2] = ["cooldown", "mana cost"];

/// Convert a slug like "god_name" to "God Name"
pub fn unslug(slug: &str) -> String {
    let words: Vec<String> = slug
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect();
    
    words.join(" ")
}

/// Convert text to a slug format
pub fn slug(text: &str) -> String {
    text.to_lowercase().replace(' ', "_")
}

/// Simplify text by removing special characters and normalizing whitespace
pub fn simplify(text: &str) -> String {
    text.trim().to_lowercase()
}

/// Save a value to browser's local storage
pub fn save_to_storage(key: &str, value: &str) {
    if let Some(storage) = window().and_then(|w| w.local_storage().ok()).flatten() {
        let _ = storage.set_item(key, value);
    }
}

/// Load a value from browser's local storage
pub fn load_from_storage(key: &str) -> Option<String> {
    window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
        .and_then(|storage| storage.get_item(key).ok())
        .flatten()
}

/// Clear a value from browser's local storage
pub fn clear_from_storage(key: &str) {
    if let Some(storage) = window().and_then(|w| w.local_storage().ok()).flatten() {
        let _ = storage.remove_item(key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unslug() {
        assert_eq!(unslug("god_name"), "God Name");
        assert_eq!(unslug("single"), "Single");
        assert_eq!(unslug(""), "");
    }

    #[test]
    fn test_slug() {
        assert_eq!(slug("God Name"), "god_name");
        assert_eq!(slug("Single"), "single");
    }

    #[test]
    fn test_simplify() {
        assert_eq!(simplify(" Test  "), "test");
        assert_eq!(simplify("UPPER"), "upper");
    }
}