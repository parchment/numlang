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

/// Token with its start and end character positions in the input.
#[derive(Debug, PartialEq, Eq)]
pub struct TokenSpan {
    pub token: Token,
    pub start: usize,
    pub end: usize,
}

/// Splits leading and trailing ASCII punctuation from a token.
/// Returns a vector of (subtoken, offset-from-token-start, length).
fn split_punct_with_offsets(token: &str, token_start: usize) -> Vec<(String, usize, usize)> {
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
        tokens.push((token[..start].to_string(), token_start, start));
    }
    if start < end {
        tokens.push((
            token[start..end].to_string(),
            token_start + start,
            end - start,
        ));
    }
    if end < token.len() {
        tokens.push((
            token[end..].to_string(),
            token_start + end,
            token.len() - end,
        ));
    }
    tokens
}

/// Tokenises an input string into tokens with character positions.
/// Preserves hyphenated number words as single tokens.
/// Splits value+unit combos (e.g., "200g" -> ["200", "g"]).
/// Separates leading/trailing punctuation as separate tokens.
pub fn tokenise(input: &str) -> Vec<TokenSpan> {
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

    let input = input.trim();
    let input_lower = input.to_lowercase();

    // Map from input_lower indices to input indices for position tracking
    // (since lowercasing may change byte offsets for non-ASCII, but for ASCII it's fine)
    // We'll assume ASCII input for simplicity.

    let mut idx = 0;
    for raw in input_lower.split_whitespace() {
        // Find raw token in original input for position tracking
        // Use input.find, starting from idx
        let orig_start = match input[idx..].find(raw) {
            Some(offset) => idx + offset,
            None => idx, // fallback
        };
        let orig_end = orig_start + raw.len();

        for (sub, sub_start_offset, sub_len) in split_punct_with_offsets(raw, orig_start) {
            let sub_start = sub_start_offset;
            let sub_end = sub_start + sub_len;
            if sub.is_empty() {
                continue;
            }

            // Value+unit combos (e.g., "200g", "3.5kg")
            if let Some(i) = sub.find(|c: char| c.is_alphabetic()) {
                if i > 0 && sub[..i].chars().all(|c| c.is_digit(10) || c == '.') {
                    let num = &sub[..i];
                    let unit = &sub[i..];
                    tokens.push(TokenSpan {
                        token: Token::NumberString(num.to_string()),
                        start: sub_start,
                        end: sub_start + num.len(),
                    });
                    tokens.push(TokenSpan {
                        token: Token::Unit(unit.to_string()),
                        start: sub_start + num.len(),
                        end: sub_end,
                    });
                    continue;
                }
            }

            // Pure number string
            if sub.chars().all(|c| c.is_digit(10) || c == '.' || c == '-')
                && sub.chars().any(|c| c.is_digit(10))
            {
                tokens.push(TokenSpan {
                    token: Token::NumberString(sub.to_string()),
                    start: sub_start,
                    end: sub_end,
                });
                continue;
            }

            // Known number word (including hyphenated, e.g., "twenty-one")
            if number_words.contains(sub.as_str())
                || sub.split('-').all(|subw| number_words.contains(subw))
            {
                tokens.push(TokenSpan {
                    token: Token::NumberWord(sub.to_string()),
                    start: sub_start,
                    end: sub_end,
                });
                continue;
            }

            // Punctuation as unknown
            if sub.chars().all(|c| c.is_ascii_punctuation()) {
                tokens.push(TokenSpan {
                    token: Token::Unknown(sub.to_string()),
                    start: sub_start,
                    end: sub_end,
                });
                continue;
            }

            // Unknown
            tokens.push(TokenSpan {
                token: Token::Unknown(sub.to_string()),
                start: sub_start,
                end: sub_end,
            });
        }
        idx = orig_end;
    }
    tokens
}
