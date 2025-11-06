use numlang::{to_words, Options};

#[test]
fn test_zero() {
    assert_eq!(to_words(0, &Options::default()), "zero");
}

#[test]
fn test_positive_integer() {
    assert_eq!(to_words(42, &Options::default()), "forty-two");
    assert_eq!(to_words(100, &Options::default()), "one hundred");
    assert_eq!(
        to_words(1234, &Options::default()),
        "one thousand two hundred thirty-four"
    );
}

#[test]
fn test_negative_integer() {
    assert_eq!(to_words(-7, &Options::default()), "negative seven");
}

#[test]
fn test_float() {
    assert_eq!(
        to_words(12.34, &Options::default()),
        "twelve point three four"
    );
    assert_eq!(
        to_words(-0.56, &Options::default()),
        "negative zero point five six"
    );
    assert_eq!(
        to_words(123.456, &Options::default()),
        "one hundred twenty-three point four five six"
    );
}

#[test]
fn test_formatting_options() {
    let mut opts = Options::default();
    opts.use_and = true;
    assert_eq!(to_words(123, &opts), "one hundred and twenty-three");

    opts = Options::default();
    opts.use_commas = true;
    assert_eq!(
        to_words(1234, &opts),
        "one thousand, two hundred thirty-four"
    );

    opts = Options::default();
    opts.uppercase = true;
    assert_eq!(
        to_words(1234, &opts),
        "ONE THOUSAND TWO HUNDRED THIRTY-FOUR"
    );

    opts = Options::default();
    opts.capitalize = true;
    assert_eq!(
        to_words(1234, &opts),
        "One thousand two hundred thirty-four"
    );

    opts = Options::default();
    opts.append_only = true;
    assert_eq!(
        to_words(1234, &opts),
        "one thousand two hundred thirty-four only"
    );
}
