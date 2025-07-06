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

/// Convert text to a slug format by removing special characters and replacing spaces with underscores
/// This is used to create consistent identifiers from display names
/// e.g., "Ah Muzen Cab" -> "ah_muzen_cab", "Chang'e" -> "change"
pub fn slugify(text: &str) -> String {
    // Remove apostrophes and quotes
    let text = text.replace('\'', "").replace('"', "");
    
    // Replace non-alphanumeric characters (except spaces and hyphens) with nothing
    let text: String = text
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-')
        .collect();
    
    // Replace spaces and hyphens with underscores, then lowercase
    text.replace(' ', "_")
        .replace('-', "_")
        .to_lowercase()
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
    fn test_slugify() {
        assert_eq!(slugify("God Name"), "god_name");
        assert_eq!(slugify("Single"), "single");
        assert_eq!(slugify("Ah Muzen Cab"), "ah_muzen_cab");
        assert_eq!(slugify("Chang'e"), "change");
        assert_eq!(slugify("Ao Kuang"), "ao_kuang");
        assert_eq!(slugify("He Bo"), "he_bo");
        assert_eq!(slugify("Nu Wa"), "nu_wa");
        assert_eq!(slugify("Sun Wukong"), "sun_wukong");
        assert_eq!(slugify("The Morrigan"), "the_morrigan");
        assert_eq!(slugify("Baron Samedi"), "baron_samedi");
        assert_eq!(slugify("Cu Chulainn"), "cu_chulainn");
        assert_eq!(slugify("Da Ji"), "da_ji");
        assert_eq!(slugify("Erlang Shen"), "erlang_shen");
        assert_eq!(slugify("Guan Yu"), "guan_yu");
        assert_eq!(slugify("Hun Batz"), "hun_batz");
        assert_eq!(slugify("Ne Zha"), "ne_zha");
        assert_eq!(slugify("Xing Tian"), "xing_tian");
        assert_eq!(slugify("Yu Huang"), "yu_huang");
        assert_eq!(slugify("Zhong Kui"), "zhong_kui");
        assert_eq!(slugify("Item-Name!"), "item_name");
        assert_eq!(slugify("@Special#Item$"), "specialitem");
    }

    #[test]
    fn test_simplify() {
        assert_eq!(simplify(" Test  "), "test");
        assert_eq!(simplify("UPPER"), "upper");
    }
}