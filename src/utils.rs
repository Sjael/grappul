const NON_CAP_WORDS: [&str; 4] = ["and", "or", "the", "of"];
pub const ATTR_FLIP: [&str; 2] = ["cooldown", "mana cost"];

/// Converts a slug (underscore/hyphen separated string) into a properly capitalized title
/// Example: "fire_giant" -> "Fire Giant"
pub fn unslug(slug: &str) -> String {
    let words: Vec<String> = slug
        .replace('_', " ")
        .replace('-', " ")
        .split_whitespace()
        .map(|s| {
            if !NON_CAP_WORDS.contains(&s.to_lowercase().as_str()) {
                let mut chars = s.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().chain(chars.as_str().chars()).collect(),
                }
            } else {
                s.to_lowercase()
            }
        })
        .collect();

    words.join(" ")
}

/// Converts a string into a slug (lowercase with underscores)
/// Example: "Fire Giant" -> "fire_giant"
pub fn slug(text: &str) -> String {
    text.replace(' ', "_").to_lowercase()
}

/// Simplifies text by removing apostrophes and converting to lowercase
/// Example: "Warrior's Axe" -> "warriors axe"
pub fn simplify(text: &str) -> String {
    text.replace('\'', "").to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unslug() {
        assert_eq!(unslug("fire_giant"), "Fire Giant");
        assert_eq!(unslug("the_fire_giant"), "the Fire Giant");
        assert_eq!(unslug("warriors-axe"), "Warriors Axe");
    }

    #[test]
    fn test_slug() {
        assert_eq!(slug("Fire Giant"), "fire_giant");
        assert_eq!(slug("The Fire Giant"), "the_fire_giant");
    }

    #[test]
    fn test_simplify() {
        assert_eq!(simplify("Warrior's Axe"), "warriors axe");
        assert_eq!(simplify("Hunter's Bow"), "hunters bow");
    }
} 