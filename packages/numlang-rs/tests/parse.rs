use numlang::parse_words;

#[test]
fn test_basic_numbers() {
    assert_eq!(parse_words("zero").unwrap(), 0.0);
    assert_eq!(parse_words("one").unwrap(), 1.0);
    assert_eq!(parse_words("forty-two").unwrap(), 42.0);
    assert_eq!(parse_words("one hundred").unwrap(), 100.0);
    assert_eq!(parse_words("one hundred twenty-three").unwrap(), 123.0);
    assert_eq!(parse_words("one thousand").unwrap(), 1000.0);
    assert_eq!(
        parse_words("one thousand two hundred thirty-four").unwrap(),
        1234.0
    );
    assert_eq!(parse_words("negative seven").unwrap(), -7.0);
}

#[test]
fn test_scales() {
    assert_eq!(parse_words("one million").unwrap(), 1_000_000.0);
    assert_eq!(parse_words("two billion").unwrap(), 2_000_000_000.0);
    assert_eq!(parse_words("three trillion").unwrap(), 3_000_000_000_000.0);
}

#[test]
fn test_compound_numbers() {
    assert_eq!(
        parse_words("nine hundred ninety-nine thousand nine hundred ninety-nine").unwrap(),
        999_999.0
    );
    assert_eq!(parse_words("seven hundred eighty-six").unwrap(), 786.0);
}

#[test]
fn test_floats() {
    assert!((parse_words("twelve point three four").unwrap() - 12.34).abs() < 1e-9);
    assert!((parse_words("negative zero point five six").unwrap() + 0.56).abs() < 1e-9);
    assert!(
        (parse_words("one hundred twenty-three point four five six").unwrap() - 123.456).abs()
            < 1e-9
    );
}

#[test]
fn test_fractions() {
    assert!((parse_words("half").unwrap() - 0.5).abs() < 1e-9);
    assert!((parse_words("one half").unwrap() - 0.5).abs() < 1e-9);
    assert!((parse_words("quarter").unwrap() - 0.25).abs() < 1e-9);
    assert!((parse_words("three quarters").unwrap() - 0.75).abs() < 1e-9);
    assert!((parse_words("one and a half").unwrap() - 1.5).abs() < 1e-9);
    assert!((parse_words("two and three quarters").unwrap() - 2.75).abs() < 1e-9);
}

#[test]
fn test_errors() {
    assert!(parse_words("foo").is_err());
    assert!(parse_words("one foo").is_err());
}
