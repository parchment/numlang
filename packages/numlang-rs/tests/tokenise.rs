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
        let actual = tokenise(input);
        assert_eq!(actual, expected, "Failed for input: {}", input);
    }
}
