use std::collections::HashMap;

/// Returns the plural form of a unit or word.
/// Abbreviations (e.g., "mg", "ml") are returned unchanged.
pub fn to_plural(singular: &str) -> String {
    if is_abbreviation(singular) {
        return singular.to_string();
    }
    let irregular = irregular_plurals();
    if let Some(&plural) = irregular.get(singular) {
        return plural.to_string();
    }
    // Rules-based
    if singular.ends_with("y")
        && singular.len() > 1
        && !matches!(
            singular.chars().nth(singular.len() - 2),
            Some('a' | 'e' | 'i' | 'o' | 'u')
        )
    {
        let base = &singular[..singular.len() - 1];
        return format!("{}ies", base);
    }
    if singular.ends_with("s")
        || singular.ends_with("x")
        || singular.ends_with("z")
        || singular.ends_with("ch")
        || singular.ends_with("sh")
    {
        return format!("{}es", singular);
    }
    format!("{}s", singular)
}

/// Returns the singular form of a unit or word.
/// Abbreviations (e.g., "mg", "ml") are returned unchanged.
/// Handles pluralized abbreviations like "mls" -> "ml", "kgs" -> "kg".
pub fn to_singular(unit: &str) -> String {
    // Handle pluralized abbreviations like "mls" -> "ml", "kgs" -> "kg"
    if unit.len() > 2 && unit.ends_with('s') {
        let candidate = &unit[..unit.len() - 1];
        if is_abbreviation(candidate) {
            return candidate.to_string();
        }
    }
    if is_abbreviation(unit) {
        return unit.to_string();
    }
    let irregular = irregular_singulars();
    if let Some(&singular) = irregular.get(unit) {
        return singular.to_string();
    }
    // Rules-based
    if unit.ends_with("ies") && unit.len() > 3 {
        let base = &unit[..unit.len() - 3];
        return format!("{}y", base);
    }
    if unit.ends_with("es")
        && (unit.ends_with("ses")
            || unit.ends_with("xes")
            || unit.ends_with("zes")
            || unit.ends_with("ches")
            || unit.ends_with("shes"))
    {
        return unit[..unit.len() - 2].to_string();
    }
    if unit.ends_with('s') && unit.len() > 1 {
        return unit[..unit.len() - 1].to_string();
    }
    unit.to_string()
}

/// Returns true if the string is likely an abbreviation (all lowercase or uppercase, <=4 chars, contains no vowels).
fn is_abbreviation(s: &str) -> bool {
    let s = s.trim();
    let len = s.len();
    if len == 0 || len > 4 {
        return false;
    }
    let is_alpha = s.chars().all(|c| c.is_ascii_alphabetic());
    let is_upper_or_lower =
        s.chars().all(|c| c.is_ascii_uppercase()) || s.chars().all(|c| c.is_ascii_lowercase());
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let has_vowels = s.chars().any(|c| vowels.contains(&c));
    is_alpha && is_upper_or_lower && !has_vowels
}

/// Irregular singular → plural map.
fn irregular_plurals() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("foot", "feet");
    m.insert("inch", "inches");
    m.insert("mouse", "mice");
    m.insert("tooth", "teeth");
    m.insert("tablet", "tablets");
    m.insert("capsule", "capsules");
    m.insert("patch", "patches");
    m.insert("suppository", "suppositories");
    m.insert("dose", "doses");
    m.insert("chew", "chews");
    m.insert("vial", "vials");
    m.insert("ampule", "ampules");
    m.insert("drop", "drops");
    m.insert("spray", "sprays");
    m.insert("puff", "puffs");
    // Add more as needed
    m
}

/// Irregular plural → singular map, generated from irregular_plurals().
fn irregular_singulars() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    for (sing, plur) in irregular_plurals() {
        m.insert(plur, sing);
    }
    m
}
