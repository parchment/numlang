use std::collections::HashMap;

/// Expands a unit abbreviation (e.g., "ml") to its full word equivalent ("milliliters").
/// Returns None if the unit is not recognized.
pub fn expand_unit(unit: &str) -> Option<&'static str> {
    let map = unit_map();
    map.get(&unit.to_lowercase()[..]).copied()
}

fn unit_map() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    // Volume
    m.insert("ml", "milliliters");
    m.insert("cc", "cubic centimeters");
    m.insert("l", "liters");
    m.insert("dl", "deciliters");
    m.insert("ul", "microliters");
    m.insert("fl oz", "fluid ounces");
    m.insert("gal", "gallons");
    m.insert("qt", "quarts");
    m.insert("pt", "pints");
    m.insert("cup", "cups");
    m.insert("tbsp", "tablespoons");
    m.insert("tsp", "teaspoons");
    m.insert("drop", "drops");
    m.insert("gtt", "drops");
    m.insert("spray", "sprays");
    m.insert("puff", "puffs");

    // Mass/weight
    m.insert("mg", "milligrams");
    m.insert("g", "grams");
    m.insert("kg", "kilograms");
    m.insert("mcg", "micrograms");
    m.insert("μg", "micrograms");
    m.insert("lb", "pounds");
    m.insert("oz", "ounces");
    m.insert("iu", "international units");
    m.insert("u", "units");
    m.insert("meq", "milliequivalents");

    // Length
    m.insert("mm", "millimeters");
    m.insert("cm", "centimeters");
    m.insert("m", "meters");
    m.insert("km", "kilometers");
    m.insert("in", "inches");
    m.insert("ft", "feet");
    m.insert("yd", "yards");
    m.insert("mi", "miles");

    // Time
    m.insert("s", "seconds");
    m.insert("sec", "seconds");
    m.insert("min", "minutes");
    m.insert("h", "hours");
    m.insert("hr", "hours");
    m.insert("d", "days");
    m.insert("day", "days");
    m.insert("wk", "weeks");
    m.insert("mo", "months");
    m.insert("yr", "years");

    // Dosage forms
    m.insert("tab", "tablets");
    m.insert("tabs", "tablets");
    m.insert("cap", "capsules");
    m.insert("caps", "capsules");
    m.insert("supp", "suppositories");
    m.insert("amp", "ampules");
    m.insert("vial", "vials");
    m.insert("patch", "patches");
    m.insert("chew", "chews");
    m.insert("chews", "chews");
    m.insert("dose", "doses");
    m.insert("doses", "doses");

    m
}
