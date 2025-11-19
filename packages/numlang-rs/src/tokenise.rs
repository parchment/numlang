use crate::cardinal::{ONES, SCALES, TENS};
use crate::unit;
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
/// Splits value+unit combos (e.g., "200g" -> ["200", "g"], "20mg/kg" -> ["20", "mg/kg"]).
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

    // Build known unit set (abbreviations and compound units)
    let mut unit_set = HashSet::new();
    for (abbr, _) in unit::unit_map() {
        unit_set.insert(abbr.to_lowercase());
    }

    let input = input.trim();
    let mut idx = 0;
    let input_bytes = input.as_bytes();
    let len = input.len();

    while idx < len {
        // Skip whitespace
        while idx < len && input_bytes[idx].is_ascii_whitespace() {
            idx += 1;
        }
        if idx >= len {
            break;
        }
        // Find end of token
        let start = idx;
        let mut end = idx;
        while end < len && !input_bytes[end].is_ascii_whitespace() {
            end += 1;
        }
        let raw = &input[start..end];

        for (sub, sub_start_offset, sub_len) in split_punct_with_offsets(raw, start) {
            let sub_start = sub_start_offset;
            let sub_end = sub_start + sub_len;
            if sub.is_empty() {
                continue;
            }
            let sub_lc = sub.to_lowercase();

            // Value+unit combos (e.g., "200g", "3.5kg", "20mg/kg")
            // Find first non-number character
            let mut i = 0;
            for (idx_char, c) in sub_lc.char_indices() {
                if !(c.is_digit(10) || c == '.' || c == '-') {
                    i = idx_char;
                    break;
                }
            }
            // If i > 0 and the rest matches a known unit, split
            if i > 0 && i < sub_lc.len() {
                let num = &sub[..i];
                let unit_candidate = &sub[i..];
                let unit_candidate_lc = unit_candidate.to_lowercase();
                if unit_set.contains(unit_candidate_lc.as_str()) {
                    tokens.push(TokenSpan {
                        token: Token::NumberString(num.to_string()),
                        start: sub_start,
                        end: sub_start + num.len(),
                    });
                    tokens.push(TokenSpan {
                        token: Token::Unit(unit_candidate.to_string()),
                        start: sub_start + num.len(),
                        end: sub_end,
                    });
                    continue;
                }
            }

            // Pure number string
            if sub_lc
                .chars()
                .all(|c| c.is_digit(10) || c == '.' || c == '-')
                && sub_lc.chars().any(|c| c.is_digit(10))
            {
                tokens.push(TokenSpan {
                    token: Token::NumberString(sub.to_string()),
                    start: sub_start,
                    end: sub_end,
                });
                continue;
            }

            // Known number word (including hyphenated, e.g., "twenty-one")
            if number_words.contains(sub_lc.as_str())
                || sub_lc.split('-').all(|subw| number_words.contains(subw))
            {
                tokens.push(TokenSpan {
                    token: Token::NumberWord(sub.to_string()),
                    start: sub_start,
                    end: sub_end,
                });
                continue;
            }

            // Known unit (standalone)
            if unit_set.contains(sub_lc.as_str()) {
                tokens.push(TokenSpan {
                    token: Token::Unit(sub.to_string()),
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
        idx = end;
    }
    tokens
}
