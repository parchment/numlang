use numlang::{to_ordinal, to_words_ordinal, Options};

#[test]
fn test_to_ordinal_basic() {
    assert_eq!(to_ordinal(1), "1st");
    assert_eq!(to_ordinal(2), "2nd");
    assert_eq!(to_ordinal(3), "3rd");
    assert_eq!(to_ordinal(11), "11th");
    assert_eq!(to_ordinal(21), "21st");
    assert_eq!(to_ordinal(42), "42nd");
    assert_eq!(to_ordinal(103), "103rd");
    assert_eq!(to_ordinal(-7), "-7th");
}

#[test]
fn test_to_words_ordinal_basic() {
    assert_eq!(to_words_ordinal(1, &Options::default()), "first");
    assert_eq!(to_words_ordinal(2, &Options::default()), "second");
    assert_eq!(to_words_ordinal(3, &Options::default()), "third");
    assert_eq!(to_words_ordinal(21, &Options::default()), "twenty-first");
    assert_eq!(to_words_ordinal(100, &Options::default()), "one hundredth");
    assert_eq!(
        to_words_ordinal(1000, &Options::default()),
        "one thousandth"
    );
    assert_eq!(
        to_words_ordinal(1234, &Options::default()),
        "one thousand two hundred thirty-fourth"
    );
    assert_eq!(
        to_words_ordinal(-7, &Options::default()),
        "negative seventh"
    );
}

#[test]
fn test_to_words_ordinal_formatting() {
    let mut opts = Options::default();
    opts.use_commas = true;
    opts.use_and = true;
    opts.capitalize = true;
    assert_eq!(
        to_words_ordinal(1234, &opts),
        "One thousand, two hundred and thirty-fourth"
    );
}
