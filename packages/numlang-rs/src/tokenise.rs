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

/// Splits leading and trailing ASCII punctuation from a token.
/// E.g., "days." -> ["days", "."], "\"foo!\"" -> ["\"", "foo", "!\""]
fn split_punct<'a>(token: &'a str) -> Vec<&'a str> {
    let mut tokens = Vec::new();
    let mut start = 0;
    let mut end = token.len();

    // Find first non-punctuation
    for (i, c) in token.char_indices() {
        if !c.is_ascii_punctuation() {
            start = i;
            break;
        }
    }
    // Find last non-punctuation
    for (i, c) in token.char_indices().rev() {
        if !c.is_ascii_punctuation() {
            end = i + c.len_utf8();
            break;
        }
    }
    if start > 0 {
        tokens.push(&token[..start]);
    }
    if start < end {
        tokens.push(&token[start..end]);
    }
    if end < token.len() {
        tokens.push(&token[end..]);
    }
    tokens
}

/// Tokenises an input string into tokens.
/// Preserves hyphenated number words as single tokens.
/// Splits value+unit combos (e.g., "200g" -> ["200", "g"]).
/// Separates leading/trailing punctuation as separate tokens.
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
        for part in split_punct(raw) {
            if part.is_empty() {
                continue;
            }

            // Value+unit combos (e.g., "200g", "3.5kg")
            if let Some(idx) = part.find(|c: char| c.is_alphabetic()) {
                if idx > 0 && part[..idx].chars().all(|c| c.is_digit(10) || c == '.') {
                    let (num, unit) = part.split_at(idx);
                    tokens.push(Token::NumberString(num.to_string()));
                    tokens.push(Token::Unit(unit.to_string()));
                    continue;
                }
            }

            // Pure number string
            if part.chars().all(|c| c.is_digit(10) || c == '.' || c == '-')
                && part.chars().any(|c| c.is_digit(10))
            {
                tokens.push(Token::NumberString(part.to_string()));
                continue;
            }

            // Known number word (including hyphenated, e.g., "twenty-one")
            if number_words.contains(part) || part.split('-').all(|sub| number_words.contains(sub))
            {
                tokens.push(Token::NumberWord(part.to_string()));
                continue;
            }

            // Punctuation as unknown
            if part.chars().all(|c| c.is_ascii_punctuation()) {
                tokens.push(Token::Unknown(part.to_string()));
                continue;
            }

            // Unknown
            tokens.push(Token::Unknown(part.to_string()));
        }
    }
    tokens
}
