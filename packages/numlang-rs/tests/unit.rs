use numlang::{abbreviate_unit, expand_unit};

#[test]
fn test_expand_unit_basic() {
    assert_eq!(expand_unit("ml"), Some("milliliter"));
    assert_eq!(expand_unit("MG"), Some("milligram"));
    assert_eq!(expand_unit("kg"), Some("kilogram"));
    assert_eq!(expand_unit("oz"), Some("ounce"));
    assert_eq!(expand_unit("IU"), Some("international unit"));
    assert_eq!(expand_unit("mcg"), Some("microgram"));
    assert_eq!(expand_unit("μg"), Some("microgram"));
    assert_eq!(expand_unit("cc"), Some("cubic centimeter"));
    assert_eq!(expand_unit("gtt"), Some("drop"));
    assert_eq!(expand_unit("tab"), Some("tablet"));
    assert_eq!(expand_unit("cap"), Some("capsule"));
    assert_eq!(expand_unit("patch"), Some("patch"));
    assert_eq!(expand_unit("dose"), Some("dose"));
    assert_eq!(expand_unit("unknown"), None);
}

#[test]
fn test_expand_unit_plural_and_case() {
    assert_eq!(expand_unit("tab"), Some("tablet"));
    assert_eq!(expand_unit("cap"), Some("capsule"));
    assert_eq!(expand_unit("chew"), Some("chew"));
}

#[test]
fn test_abbreviate_unit_singular() {
    assert_eq!(abbreviate_unit("milliliter"), Some("ml"));
    assert_eq!(abbreviate_unit("tablet"), Some("tab"));
    assert_eq!(abbreviate_unit("drop"), Some("gtt"));
    let abbr = abbreviate_unit("microgram");
    assert!(abbr == Some("mcg") || abbr == Some("μg"));
}

#[test]
fn test_abbreviate_unit_plural() {
    assert_eq!(abbreviate_unit("milliliters"), Some("ml"));
    assert_eq!(abbreviate_unit("tablets"), Some("tab"));
    assert_eq!(abbreviate_unit("drops"), Some("gtt"));
    let abbr = abbreviate_unit("micrograms");
    assert!(abbr == Some("mcg") || abbr == Some("μg"));
}

#[test]
fn test_abbreviate_unit_case_insensitive() {
    assert_eq!(abbreviate_unit("Milliliters"), Some("ml"));
    assert_eq!(abbreviate_unit("TABLETS"), Some("tab"));
}

#[test]
fn test_abbreviate_unit_unknown() {
    assert_eq!(abbreviate_unit("unknownunit"), None);
    assert_eq!(abbreviate_unit(""), None);
}
