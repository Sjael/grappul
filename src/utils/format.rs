/// Convert god name from code format to image filename format
/// Since we now use slugified names consistently, this just returns the name as-is
pub fn format_god_image_name(god_name: &str) -> String {
    god_name.to_string()
}