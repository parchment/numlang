use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitType {
    Volume,
    Mass,
    Length,
    Time,
    DosageForm,
    Other,
}

/// Expands a unit abbreviation (e.g., "ml") to its full word equivalent ("milliliter").
/// Returns None if the unit is not recognized.
pub fn expand_unit(unit: &str) -> Option<&'static str> {
    let map = unit_map();
    map.get(&unit.to_lowercase()[..])
        .map(|(expanded, _)| *expanded)
}

/// Returns the canonical abbreviation for a full unit name (singular or plural).
/// E.g., "milliliters" or "milliliter" -> "ml"
pub fn abbreviate_unit(expanded: &str) -> Option<&'static str> {
    let map = reverse_unit_map();
    let key = expanded.trim().to_lowercase();
    map.get(key.as_str()).copied()
}

/// Returns the UnitType for a unit abbreviation (e.g., "mg", "ml") or expanded name.
/// Returns None if not recognized.
pub fn unit_type(unit: &str) -> Option<UnitType> {
    let map = unit_map();
    if let Some((_, typ)) = map.get(&unit.to_lowercase()[..]) {
        return Some(*typ);
    }
    let rev_map = reverse_unit_map_with_type();
    let key = unit.trim().to_lowercase();
    rev_map.get(key.as_str()).copied()
}

pub fn unit_map() -> HashMap<&'static str, (&'static str, UnitType)> {
    let mut m = HashMap::new();
    // Volume
    m.insert("ml", ("milliliter", UnitType::Volume));
    m.insert("cc", ("cubic centimeter", UnitType::Volume));
    m.insert("l", ("liter", UnitType::Volume));
    m.insert("dl", ("deciliter", UnitType::Volume));
    m.insert("ul", ("microliter", UnitType::Volume));
    m.insert("fl oz", ("fluid ounce", UnitType::Volume));
    m.insert("gal", ("gallon", UnitType::Volume));
    m.insert("qt", ("quart", UnitType::Volume));
    m.insert("pt", ("pint", UnitType::Volume));
    m.insert("tbsp", ("tablespoon", UnitType::Volume));
    m.insert("tsp", ("teaspoon", UnitType::Volume));
    m.insert("gtt", ("drop", UnitType::DosageForm));
    m.insert("spray", ("spray", UnitType::DosageForm));
    m.insert("puff", ("puff", UnitType::DosageForm));
    m.insert("mg/kg", ("milligram per kilogram", UnitType::Mass));
    m.insert("ml/kg", ("milliliter per kilogram", UnitType::Volume));
    m.insert("mcg/kg", ("microgram per kilogram", UnitType::Mass));
    m.insert("ug/kg", ("microgram per kilogram", UnitType::Mass));

    // Mass/weight
    m.insert("mg", ("milligram", UnitType::Mass));
    m.insert("g", ("gram", UnitType::Mass));
    m.insert("kg", ("kilogram", UnitType::Mass));
    m.insert("mcg", ("microgram", UnitType::Mass));
    m.insert("μg", ("microgram", UnitType::Mass));
    m.insert("lb", ("pound", UnitType::Mass));
    m.insert("oz", ("ounce", UnitType::Mass));
    m.insert("iu", ("international unit", UnitType::Other));
    m.insert("u", ("unit", UnitType::Other));
    m.insert("meq", ("milliequivalent", UnitType::Other));

    // Length
    m.insert("mm", ("millimeter", UnitType::Length));
    m.insert("cm", ("centimeter", UnitType::Length));
    m.insert("m", ("meter", UnitType::Length));
    m.insert("km", ("kilometer", UnitType::Length));
    m.insert("in", ("inch", UnitType::Length));
    m.insert("ft", ("foot", UnitType::Length));
    m.insert("yd", ("yard", UnitType::Length));
    m.insert("mi", ("mile", UnitType::Length));

    // Time (abbreviations only)
    m.insert("s", ("second", UnitType::Time));
    m.insert("sec", ("second", UnitType::Time));
    m.insert("min", ("minute", UnitType::Time));
    m.insert("h", ("hour", UnitType::Time));
    m.insert("hr", ("hour", UnitType::Time));
    m.insert("d", ("day", UnitType::Time));
    m.insert("wk", ("week", UnitType::Time));
    m.insert("mo", ("month", UnitType::Time));
    m.insert("yr", ("year", UnitType::Time));

    // Dosage forms
    m.insert("tablet", ("tablet", UnitType::DosageForm));
    m.insert("cap", ("capsule", UnitType::DosageForm));
    m.insert("capsule", ("capsule", UnitType::DosageForm));
    m.insert("supp", ("suppository", UnitType::DosageForm));
    m.insert("amp", ("ampule", UnitType::DosageForm));
    m.insert("vial", ("vial", UnitType::DosageForm));
    m.insert("patch", ("patch", UnitType::DosageForm));
    m.insert("chew", ("chew", UnitType::DosageForm));
    m.insert("dose", ("dose", UnitType::DosageForm));
    m.insert("sachet", ("sachet", UnitType::DosageForm));
    m.insert("kg dose", ("kilogram dose", UnitType::DosageForm));
    m.insert("pump", ("pump", UnitType::DosageForm));
    m.insert("app", ("application", UnitType::DosageForm));
    m.insert("pipette", ("pipette", UnitType::DosageForm));

    m
}

fn reverse_unit_map() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    for (abbr, (expanded, _)) in unit_map() {
        // Insert singular
        m.insert(expanded, abbr);
        // Insert plural
        let plural = crate::plural::to_plural(expanded);
        m.insert(Box::leak(plural.into_boxed_str()), abbr);
    }
    m
}

fn reverse_unit_map_with_type() -> HashMap<&'static str, UnitType> {
    let mut m = HashMap::new();
    for (_abbr, (expanded, typ)) in unit_map() {
        m.insert(expanded, typ);
        let plural = crate::plural::to_plural(expanded);
        m.insert(Box::leak(plural.into_boxed_str()), typ);
    }
    m
}
