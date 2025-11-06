use numlang::parse_words;

#[test]
fn test_basic_numbers() {
    assert_eq!(parse_words("zero").unwrap(), 0);
    assert_eq!(parse_words("one").unwrap(), 1);
    assert_eq!(parse_words("forty-two").unwrap(), 42);
    assert_eq!(parse_words("one hundred").unwrap(), 100);
    assert_eq!(parse_words("one hundred twenty-three").unwrap(), 123);
    assert_eq!(parse_words("one thousand").unwrap(), 1000);
    assert_eq!(
        parse_words("one thousand two hundred thirty-four").unwrap(),
        1234
    );
    assert_eq!(parse_words("negative seven").unwrap(), -7);
}

#[test]
fn test_scales() {
    assert_eq!(parse_words("one million").unwrap(), 1_000_000);
    assert_eq!(parse_words("two billion").unwrap(), 2_000_000_000);
    assert_eq!(parse_words("three trillion").unwrap(), 3_000_000_000_000);
}

#[test]
fn test_compound_numbers() {
    assert_eq!(
        parse_words("nine hundred ninety-nine thousand nine hundred ninety-nine").unwrap(),
        999_999
    );
    assert_eq!(parse_words("seven hundred eighty-six").unwrap(), 786);
}

#[test]
fn test_errors() {
    assert!(parse_words("foo").is_err());
    assert!(parse_words("one foo").is_err());
}
