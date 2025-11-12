use numlang::{to_plural, to_singular};

#[test]
fn test_to_plural_regular() {
    assert_eq!(to_plural("tablet"), "tablets");
    assert_eq!(to_plural("patch"), "patches");
    assert_eq!(to_plural("capsule"), "capsules");
    assert_eq!(to_plural("dose"), "doses");
    assert_eq!(to_plural("chew"), "chews");
    assert_eq!(to_plural("vial"), "vials");
    assert_eq!(to_plural("ampule"), "ampules");
    assert_eq!(to_plural("drop"), "drops");
    assert_eq!(to_plural("spray"), "sprays");
    assert_eq!(to_plural("puff"), "puffs");
}

#[test]
fn test_to_plural_irregular() {
    assert_eq!(to_plural("foot"), "feet");
    assert_eq!(to_plural("inch"), "inches");
    assert_eq!(to_plural("mouse"), "mice");
    assert_eq!(to_plural("tooth"), "teeth");
}

#[test]
fn test_to_plural_abbreviation() {
    assert_eq!(to_plural("mg"), "mg");
    assert_eq!(to_plural("ml"), "ml");
    assert_eq!(to_plural("IU"), "IU");
    assert_eq!(to_plural("cc"), "cc");
}

#[test]
fn test_to_singular_regular() {
    assert_eq!(to_singular("tablets"), "tablet");
    assert_eq!(to_singular("patches"), "patch");
    assert_eq!(to_singular("capsules"), "capsule");
    assert_eq!(to_singular("doses"), "dose");
    assert_eq!(to_singular("chews"), "chew");
    assert_eq!(to_singular("vials"), "vial");
    assert_eq!(to_singular("ampules"), "ampule");
    assert_eq!(to_singular("drops"), "drop");
    assert_eq!(to_singular("sprays"), "spray");
    assert_eq!(to_singular("puffs"), "puff");
}

#[test]
fn test_to_singular_irregular() {
    assert_eq!(to_singular("feet"), "foot");
    assert_eq!(to_singular("inches"), "inch");
    assert_eq!(to_singular("mice"), "mouse");
    assert_eq!(to_singular("teeth"), "tooth");
}

#[test]
fn test_to_singular_abbreviation() {
    assert_eq!(to_singular("mg"), "mg");
    assert_eq!(to_singular("ml"), "ml");
    assert_eq!(to_singular("IU"), "IU");
    assert_eq!(to_singular("cc"), "cc");
}

#[test]
fn test_to_singular_pluralized_abbreviation() {
    assert_eq!(to_singular("mls"), "ml");
    assert_eq!(to_singular("kgs"), "kg");
    assert_eq!(to_singular("mgs"), "mg");
    assert_eq!(to_singular("ccs"), "cc");
    assert_eq!(to_singular("IUs"), "IU");
}
