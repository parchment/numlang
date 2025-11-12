use numlang::expand_unit;

#[test]
fn test_expand_unit_basic() {
    assert_eq!(expand_unit("ml"), Some("milliliters"));
    assert_eq!(expand_unit("MG"), Some("milligrams"));
    assert_eq!(expand_unit("kg"), Some("kilograms"));
    assert_eq!(expand_unit("oz"), Some("ounces"));
    assert_eq!(expand_unit("IU"), Some("international units"));
    assert_eq!(expand_unit("mcg"), Some("micrograms"));
    assert_eq!(expand_unit("μg"), Some("micrograms"));
    assert_eq!(expand_unit("cc"), Some("cubic centimeters"));
    assert_eq!(expand_unit("gtt"), Some("drops"));
    assert_eq!(expand_unit("tab"), Some("tablets"));
    assert_eq!(expand_unit("caps"), Some("capsules"));
    assert_eq!(expand_unit("patch"), Some("patches"));
    assert_eq!(expand_unit("dose"), Some("doses"));
    assert_eq!(expand_unit("unknown"), None);
}

#[test]
fn test_expand_unit_plural_and_case() {
    assert_eq!(expand_unit("Tabs"), Some("tablets"));
    assert_eq!(expand_unit("CAPS"), Some("capsules"));
    assert_eq!(expand_unit("chews"), Some("chews"));
}
