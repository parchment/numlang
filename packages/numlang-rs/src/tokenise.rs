use crate::cardinal::{ONES, SCALES, TENS};
use std::collections::HashSet;

/// Represents a token extracted from the input string.
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    NumberWord(String),
    NumberString(String),
    Unit(String),
    Unknown(String),
}

/// Tokenises an input string into tokens.
/// Preserves hyphenated number words as single tokens.
/// Splits value+unit combos (e.g., "200g" -> ["200", "g"]).
pub fn tokenise(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    // Build known number word set
    let mut number_words = HashSet::new();
    for &w in ONES.iter().chain(TENS.iter()).chain(SCALES.iter()) {
        if !w.is_empty() {
            number_words.insert(w);
        }
    }
    number_words.insert("point");
    number_words.insert("negative");
    number_words.insert("and");
    number_words.insert("hundred");

    let input = input.trim().to_lowercase();
    let raw_tokens: Vec<&str> = input.split_whitespace().collect();

    for raw in raw_tokens {
        // Value+unit combos (e.g., "200g", "3.5kg")
        if let Some(idx) = raw.find(|c: char| c.is_alphabetic()) {
            if idx > 0 && raw[..idx].chars().all(|c| c.is_digit(10) || c == '.') {
                let (num, unit) = raw.split_at(idx);
                tokens.push(Token::NumberString(num.to_string()));
                tokens.push(Token::Unit(unit.to_string()));
                continue;
            }
        }

        // Pure number string
        if raw.chars().all(|c| c.is_digit(10) || c == '.' || c == '-') {
            tokens.push(Token::NumberString(raw.to_string()));
            continue;
        }

        // Known number word (including hyphenated, e.g., "twenty-one")
        if number_words.contains(raw) || raw.split('-').all(|part| number_words.contains(part)) {
            tokens.push(Token::NumberWord(raw.to_string()));
            continue;
        }

        // Unknown
        tokens.push(Token::Unknown(raw.to_string()));
    }
    tokens
}
