use numlang::from_words;

#[test]
fn test_basic_numbers() {
    assert_eq!(from_words("zero").unwrap(), 0.0);
    assert_eq!(from_words("one").unwrap(), 1.0);
    assert_eq!(from_words("forty-two").unwrap(), 42.0);
    assert_eq!(from_words("one hundred").unwrap(), 100.0);
    assert_eq!(from_words("one hundred twenty-three").unwrap(), 123.0);
    assert_eq!(from_words("one thousand").unwrap(), 1000.0);
    assert_eq!(
        from_words("one thousand two hundred thirty-four").unwrap(),
        1234.0
    );
    assert_eq!(from_words("negative seven").unwrap(), -7.0);
}

#[test]
fn test_scales() {
    assert_eq!(from_words("one million").unwrap(), 1_000_000.0);
    assert_eq!(from_words("two billion").unwrap(), 2_000_000_000.0);
    assert_eq!(from_words("three trillion").unwrap(), 3_000_000_000_000.0);
}

#[test]
fn test_compound_numbers() {
    assert_eq!(
        from_words("nine hundred ninety-nine thousand nine hundred ninety-nine").unwrap(),
        999_999.0
    );
    assert_eq!(from_words("seven hundred eighty-six").unwrap(), 786.0);
}

#[test]
fn test_floats() {
    assert!((from_words("twelve point three four").unwrap() - 12.34).abs() < 1e-9);
    assert!((from_words("negative zero point five six").unwrap() + 0.56).abs() < 1e-9);
    assert!(
        (from_words("one hundred twenty-three point four five six").unwrap() - 123.456).abs()
            < 1e-9
    );
}

#[test]
fn test_fractions() {
    assert!((from_words("half").unwrap() - 0.5).abs() < 1e-9);
    assert!((from_words("one half").unwrap() - 0.5).abs() < 1e-9);
    assert!((from_words("quarter").unwrap() - 0.25).abs() < 1e-9);
    assert!((from_words("three quarters").unwrap() - 0.75).abs() < 1e-9);
    assert!((from_words("one and a half").unwrap() - 1.5).abs() < 1e-9);
    assert!((from_words("two and three quarters").unwrap() - 2.75).abs() < 1e-9);
}

#[test]
fn test_frequency_adverbs() {
    assert_eq!(from_words("once").unwrap(), 1.0);
    assert_eq!(from_words("twice").unwrap(), 2.0);
    assert_eq!(from_words("thrice").unwrap(), 3.0);
    assert_eq!(from_words("Twice daily").unwrap(), 2.0);
    assert_eq!(from_words("ONCE PER WEEK").unwrap(), 1.0);
}

#[test]
fn test_errors() {
    assert!(from_words("foo").is_err());
    assert!(from_words("one foo").is_err());
}

#[test]
fn test_number_strings() {
    assert_eq!(from_words("2").unwrap(), 2.0);
    assert_eq!(from_words("12.34").unwrap(), 12.34);
    assert_eq!(from_words("-7").unwrap(), -7.0);
    assert_eq!(from_words("0").unwrap(), 0.0);
}
