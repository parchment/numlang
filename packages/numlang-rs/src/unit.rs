use std::collections::HashMap;

/// Expands a unit abbreviation (e.g., "ml") to its full word equivalent ("milliliter").
/// Returns None if the unit is not recognized.
pub fn expand_unit(unit: &str) -> Option<&'static str> {
    let map = unit_map();
    map.get(&unit.to_lowercase()[..]).copied()
}

/// Returns the canonical abbreviation for a full unit name (singular or plural).
/// E.g., "milliliters" or "milliliter" -> "ml"
pub fn abbreviate_unit(expanded: &str) -> Option<&'static str> {
    let map = reverse_unit_map();
    let key = expanded.trim().to_lowercase();
    map.get(key.as_str()).copied()
}

fn unit_map() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    // Volume
    m.insert("ml", "milliliter");
    m.insert("cc", "cubic centimeter");
    m.insert("l", "liter");
    m.insert("dl", "deciliter");
    m.insert("ul", "microliter");
    m.insert("fl oz", "fluid ounce");
    m.insert("gal", "gallon");
    m.insert("qt", "quart");
    m.insert("pt", "pint");
    m.insert("tbsp", "tablespoon");
    m.insert("tsp", "teaspoon");
    m.insert("gtt", "drop");
    m.insert("spray", "spray");
    m.insert("puff", "puff");

    // Mass/weight
    m.insert("mg", "milligram");
    m.insert("g", "gram");
    m.insert("kg", "kilogram");
    m.insert("mcg", "microgram");
    m.insert("μg", "microgram");
    m.insert("lb", "pound");
    m.insert("oz", "ounce");
    m.insert("iu", "international unit");
    m.insert("u", "unit");
    m.insert("meq", "milliequivalent");

    // Length
    m.insert("mm", "millimeter");
    m.insert("cm", "centimeter");
    m.insert("m", "meter");
    m.insert("km", "kilometer");
    m.insert("in", "inch");
    m.insert("ft", "foot");
    m.insert("yd", "yard");
    m.insert("mi", "mile");

    // Time (abbreviations only)
    m.insert("s", "second");
    m.insert("sec", "second");
    m.insert("min", "minute");
    m.insert("h", "hour");
    m.insert("hr", "hour");
    m.insert("wk", "week");
    m.insert("mo", "month");
    m.insert("yr", "year");

    // Dosage forms
    m.insert("tab", "tablet");
    m.insert("cap", "capsule");
    m.insert("supp", "suppository");
    m.insert("amp", "ampule");
    m.insert("vial", "vial");
    m.insert("patch", "patch");
    m.insert("chew", "chew");
    m.insert("dose", "dose");

    m
}

fn reverse_unit_map() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    for (abbr, expanded) in unit_map() {
        // Insert singular
        m.insert(expanded, abbr);
        // Insert plural
        let plural = crate::plural::to_plural(expanded);
        m.insert(Box::leak(plural.into_boxed_str()), abbr);
    }
    m
}
