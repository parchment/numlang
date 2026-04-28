use numlang::tokenise::{tokenise, Token};

#[test]
fn test_tokenise_examples() {
    let cases = vec![
        (
            "twenty-one",
            vec![Token::NumberWord("twenty-one".to_string())],
        ),
        (
            "3.5kg",
            vec![
                Token::NumberString("3.5".to_string()),
                Token::Unit("kg".to_string()),
            ],
        ),
        (
            "negative forty-two",
            vec![
                Token::NumberWord("negative".to_string()),
                Token::NumberWord("forty-two".to_string()),
            ],
        ),
        (
            "one hundred and five",
            vec![
                Token::NumberWord("one".to_string()),
                Token::NumberWord("hundred".to_string()),
                Token::NumberWord("and".to_string()),
                Token::NumberWord("five".to_string()),
            ],
        ),
        (
            "foo bar",
            vec![
                Token::Unknown("foo".to_string()),
                Token::Unknown("bar".to_string()),
            ],
        ),
        (
            "12.34.56",
            vec![Token::NumberString("12.34.56".to_string())],
        ),
        (
            "100g of sugar",
            vec![
                Token::NumberString("100".to_string()),
                Token::Unit("g".to_string()),
                Token::Unknown("of".to_string()),
                Token::Unknown("sugar".to_string()),
            ],
        ),
    ];

    for (input, expected) in cases {
        let actual = tokenise(input)
            .into_iter()
            .map(|span| span.token)
            .collect::<Vec<_>>();
        assert_eq!(actual, expected, "Failed for input: {}", input);
    }
}

#[test]
fn test_tokenise_punctuation() {
    let cases = vec![
        (
            "days.",
            vec![
                Token::Unknown("days".to_string()),
                Token::Unknown(".".to_string()),
            ],
        ),
        (
            "\"foo!\"",
            vec![
                Token::Unknown("\"".to_string()),
                Token::Unknown("foo".to_string()),
                Token::Unknown("!\"".to_string()),
            ],
        ),
        (
            "42,",
            vec![
                Token::NumberString("42".to_string()),
                Token::Unknown(",".to_string()),
            ],
        ),
        (
            "(seven)",
            vec![
                Token::Unknown("(".to_string()),
                Token::NumberWord("seven".to_string()),
                Token::Unknown(")".to_string()),
            ],
        ),
    ];

    for (input, expected) in cases {
        let actual = tokenise(input)
            .into_iter()
            .map(|span| span.token)
            .collect::<Vec<_>>();
        assert_eq!(actual, expected, "Failed for input: {}", input);
    }
}

#[test]
fn test_tokenise_positions() {
    let input = "100g of sugar";
    let expected = vec![
        (Token::NumberString("100".to_string()), 0, 3),
        (Token::Unit("g".to_string()), 3, 4),
        (Token::Unknown("of".to_string()), 5, 7),
        (Token::Unknown("sugar".to_string()), 8, 13),
    ];
    let actual = tokenise(input);
    for (i, (token, start, end)) in expected.iter().enumerate() {
        assert_eq!(&actual[i].token, token, "Token mismatch at index {}", i);
        assert_eq!(actual[i].start, *start, "Start mismatch at index {}", i);
        assert_eq!(actual[i].end, *end, "End mismatch at index {}", i);
    }
}

#[test]
fn test_tokenise_complex_positions() {
    let input =
        "Give 1 tablet every 12 hours for 7 days then give 2 tablet every 8 hour for 5 days";
    let expected = vec![
        (Token::Unknown("Give".to_string()), 0, 4),
        (Token::NumberString("1".to_string()), 5, 6),
        (Token::Unit("tablet".to_string()), 7, 13),
        (Token::Unknown("every".to_string()), 14, 19),
        (Token::NumberString("12".to_string()), 20, 22),
        (Token::Unknown("hours".to_string()), 23, 28),
        (Token::Unknown("for".to_string()), 29, 32),
        (Token::NumberString("7".to_string()), 33, 34),
        (Token::Unknown("days".to_string()), 35, 39),
        (Token::Unknown("then".to_string()), 40, 44),
        (Token::Unknown("give".to_string()), 45, 49),
        (Token::NumberString("2".to_string()), 50, 51),
        (Token::Unit("tablet".to_string()), 52, 58),
        (Token::Unknown("every".to_string()), 59, 64),
        (Token::NumberString("8".to_string()), 65, 66),
        (Token::Unknown("hour".to_string()), 67, 71),
        (Token::Unknown("for".to_string()), 72, 75),
        (Token::NumberString("5".to_string()), 76, 77),
        (Token::Unknown("days".to_string()), 78, 82),
    ];
    let actual = tokenise(input);
    for (i, (token, start, end)) in expected.iter().enumerate() {
        assert_eq!(&actual[i].token, token, "Token mismatch at index {}", i);
        assert_eq!(actual[i].start, *start, "Start mismatch at index {}", i);
        assert_eq!(actual[i].end, *end, "End mismatch at index {}", i);
    }
}

#[test]
fn test_tokenise_compound_unit_attached() {
    let input = "20mg/kg";
    let tokens = tokenise(input);
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token, Token::NumberString("20".to_string()));
    assert_eq!(tokens[1].token, Token::Unit("mg/kg".to_string()));
}

#[test]
fn test_tokenise_compound_unit_separated() {
    let input = "20 mg/kg";
    let tokens = tokenise(input);
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token, Token::NumberString("20".to_string()));
    assert_eq!(tokens[1].token, Token::Unit("mg/kg".to_string()));
}

#[test]
fn test_tokenise_simple_attached_unit() {
    let input = "5ml";
    let tokens = tokenise(input);
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token, Token::NumberString("5".to_string()));
    assert_eq!(tokens[1].token, Token::Unit("ml".to_string()));
}

#[test]
fn test_tokenise_simple_separated_unit() {
    let input = "5 ml";
    let tokens = tokenise(input);
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token, Token::NumberString("5".to_string()));
    assert_eq!(tokens[1].token, Token::Unit("ml".to_string()));
}

#[test]
fn test_tokenise_unknown_attached() {
    let input = "20xyz";
    let tokens = tokenise(input);
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token, Token::Unknown("20xyz".to_string()));
}

// --- Multi-word (compound) unit tests ---

#[test]
fn test_compound_unit_kg_dose() {
    let tokens: Vec<_> = tokenise("5 kg dose")
        .into_iter()
        .map(|s| s.token)
        .collect();
    assert_eq!(
        tokens,
        vec![Token::NumberString("5".into()), Token::Unit("kg dose".into())]
    );
}

#[test]
fn test_compound_unit_fl_oz() {
    let tokens: Vec<_> = tokenise("5 fl oz")
        .into_iter()
        .map(|s| s.token)
        .collect();
    assert_eq!(
        tokens,
        vec![Token::NumberString("5".into()), Token::Unit("fl oz".into())]
    );
}

#[test]
fn test_compound_unit_case_insensitive() {
    let tokens: Vec<_> = tokenise("5 FL OZ")
        .into_iter()
        .map(|s| s.token)
        .collect();
    assert_eq!(
        tokens,
        vec![Token::NumberString("5".into()), Token::Unit("FL OZ".into())]
    );
}

#[test]
fn test_compound_unit_trailing_punct() {
    // Trailing punct on the last word is stripped as Unknown
    let tokens: Vec<_> = tokenise("5 fl oz.")
        .into_iter()
        .map(|s| s.token)
        .collect();
    assert_eq!(
        tokens,
        vec![
            Token::NumberString("5".into()),
            Token::Unit("fl oz".into()),
            Token::Unknown(".".into()),
        ]
    );
}

#[test]
fn test_compound_unit_leading_punct() {
    // Leading punct on word1 is emitted as Unknown before the Unit
    let tokens: Vec<_> = tokenise("(fl oz)")
        .into_iter()
        .map(|s| s.token)
        .collect();
    assert_eq!(
        tokens,
        vec![
            Token::Unknown("(".into()),
            Token::Unit("fl oz".into()),
            Token::Unknown(")".into()),
        ]
    );
}

#[test]
fn test_compound_unit_positions() {
    // "give 1 fl oz daily" — verify start/end offsets of compound unit
    let spans = tokenise("give 1 fl oz daily");
    let unit = spans.iter().find(|s| matches!(&s.token, Token::Unit(_))).unwrap();
    assert_eq!(unit.token, Token::Unit("fl oz".into()));
    assert_eq!(unit.start, 7);
    assert_eq!(unit.end, 12);
}

#[test]
fn test_compound_unit_not_split_by_punct_between_words() {
    // "fl, oz" — comma between the words prevents compound match
    let tokens: Vec<_> = tokenise("fl, oz")
        .into_iter()
        .map(|s| s.token)
        .collect();
    // "fl," → fl (Unit) + , (Unknown); "oz" → Unit
    assert!(tokens.contains(&Token::Unknown(",".into())));
    assert!(!tokens.contains(&Token::Unit("fl oz".into())));
}

// --- Attached number + multi-word unit ---

#[test]
fn test_attached_number_compound_unit_kg_dose() {
    // "12kg dose" — number attached to first word of compound unit
    let tokens: Vec<_> = tokenise("12kg dose")
        .into_iter()
        .map(|s| s.token)
        .collect();
    assert_eq!(
        tokens,
        vec![Token::NumberString("12".into()), Token::Unit("kg dose".into())]
    );
}

#[test]
fn test_attached_number_compound_unit_decimal() {
    // "3.5kg dose" — decimal number attached to first word
    let tokens: Vec<_> = tokenise("3.5kg dose")
        .into_iter()
        .map(|s| s.token)
        .collect();
    assert_eq!(
        tokens,
        vec![Token::NumberString("3.5".into()), Token::Unit("kg dose".into())]
    );
}

#[test]
fn test_attached_number_compound_unit_trailing_punct() {
    // "12kg dose." — trailing punct on the last compound word is stripped as Unknown
    let tokens: Vec<_> = tokenise("12kg dose.")
        .into_iter()
        .map(|s| s.token)
        .collect();
    assert_eq!(
        tokens,
        vec![
            Token::NumberString("12".into()),
            Token::Unit("kg dose".into()),
            Token::Unknown(".".into()),
        ]
    );
}

#[test]
fn test_attached_number_compound_unit_in_sentence() {
    // The originally-failing full sentence
    let tokens: Vec<_> = tokenise("Give 12kg dose once daily for 7 days.")
        .into_iter()
        .map(|s| s.token)
        .collect();
    let num_idx = tokens
        .iter()
        .position(|t| *t == Token::NumberString("12".into()))
        .expect("NumberString(12) not found");
    assert_eq!(tokens[num_idx + 1], Token::Unit("kg dose".into()));
}
