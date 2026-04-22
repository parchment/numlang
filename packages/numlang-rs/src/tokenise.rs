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

/// Attempts to match a compound (multi-word) unit starting at the current token.
///
/// `raw_parts` is the result of `split_punct_with_offsets` on the current raw token.
/// `end` is the byte index just past the current raw token (start of remaining input).
/// Returns the tokens to emit and the new `idx` (past the last consumed word), or `None`.
fn try_compound_unit(
    input: &str,
    input_bytes: &[u8],
    len: usize,
    end: usize,
    raw_parts: &[(String, usize, usize)],
    multi_word_units: &[Vec<String>],
) -> Option<(Vec<TokenSpan>, usize)> {
    // Find the non-punct core of the current token.
    let (core_str, core_abs_start, _core_len) = raw_parts
        .iter()
        .find(|(s, _, _)| !s.chars().all(|c| c.is_ascii_punctuation()))?;

    // Word1 must not have trailing punct — that punct would fall between the two unit words.
    if raw_parts.len() > 1 && raw_parts.last().unwrap().0.chars().all(|c| c.is_ascii_punctuation()) {
        return None;
    }

    let core_lc = core_str.to_lowercase();

    for compound in multi_word_units {
        if core_lc != compound[0] {
            continue;
        }

        let mut scan_idx = end;
        // (raw_str, raw_start, core_str, core_abs_start, core_abs_end)
        let mut lookahead: Vec<(String, usize, String, usize, usize)> = Vec::new();
        let mut matched = true;
        let last_word_idx = compound.len() - 2; // index within compound[1..]

        for (i, expected) in compound[1..].iter().enumerate() {
            // Skip whitespace
            while scan_idx < len && input_bytes[scan_idx].is_ascii_whitespace() {
                scan_idx += 1;
            }
            if scan_idx >= len {
                matched = false;
                break;
            }

            let next_start = scan_idx;
            let mut next_end = scan_idx;
            while next_end < len && !input_bytes[next_end].is_ascii_whitespace() {
                next_end += 1;
            }

            let next_raw = &input[next_start..next_end];
            let next_parts = split_punct_with_offsets(next_raw, next_start);
            let is_last = i == last_word_idx;

            if let Some((nc_str, nc_abs_start, nc_len)) = next_parts
                .iter()
                .find(|(s, _, _)| !s.chars().all(|c| c.is_ascii_punctuation()))
            {
                if nc_str.to_lowercase() != *expected {
                    matched = false;
                    break;
                }
                // No leading punct allowed on any lookahead word.
                if next_parts.first().unwrap().0.chars().all(|c| c.is_ascii_punctuation()) {
                    matched = false;
                    break;
                }
                // Trailing punct only allowed on the last word of the compound.
                let has_trailing_punct = next_parts.len() > 1
                    && next_parts.last().unwrap().0.chars().all(|c| c.is_ascii_punctuation());
                if !is_last && has_trailing_punct {
                    matched = false;
                    break;
                }
                lookahead.push((
                    next_raw.to_string(),
                    next_start,
                    nc_str.to_string(),
                    *nc_abs_start,
                    nc_abs_start + nc_len,
                ));
                scan_idx = next_end;
            } else {
                matched = false;
                break;
            }
        }

        if !matched {
            continue;
        }

        let mut result: Vec<TokenSpan> = Vec::new();

        // Leading punct of word1.
        if raw_parts[0].0.chars().all(|c| c.is_ascii_punctuation()) {
            result.push(TokenSpan {
                token: Token::Unknown(raw_parts[0].0.clone()),
                start: raw_parts[0].1,
                end: raw_parts[0].1 + raw_parts[0].2,
            });
        }

        // The compound unit span covers from core_abs_start to the last lookahead core's end.
        let unit_start = *core_abs_start;
        let unit_end = lookahead.last().unwrap().4;
        let mut unit_str = core_str.to_string();
        for la in &lookahead {
            unit_str.push(' ');
            unit_str.push_str(&la.2);
        }
        result.push(TokenSpan {
            token: Token::Unit(unit_str),
            start: unit_start,
            end: unit_end,
        });

        // Trailing punct of last word.
        let last_la = lookahead.last().unwrap();
        let last_parts = split_punct_with_offsets(&last_la.0, last_la.1);
        if last_parts.len() > 1 {
            let trailing = last_parts.last().unwrap();
            if trailing.0.chars().all(|c| c.is_ascii_punctuation()) {
                result.push(TokenSpan {
                    token: Token::Unknown(trailing.0.clone()),
                    start: trailing.1,
                    end: trailing.1 + trailing.2,
                });
            }
        }

        // scan_idx is the end of the last consumed raw token (past any trailing punct).
        return Some((result, scan_idx));
    }

    None
}

/// Tokenises an input string into tokens with character positions.
/// Preserves hyphenated number words as single tokens.
/// Splits value+unit combos (e.g., "200g" -> ["200", "g"], "20mg/kg" -> ["20", "mg/kg"]).
/// Separates leading/trailing punctuation as separate tokens.
/// Recognises multi-word units (e.g., "kg dose", "fl oz") via a longest-match lookahead.
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

    // Multi-word units sorted by descending word count (longest match first).
    // Secondary sort is lexicographic for determinism.
    let mut multi_word_units: Vec<Vec<String>> = unit_set
        .iter()
        .filter(|k| k.contains(' '))
        .map(|k| k.split(' ').map(|w| w.to_string()).collect::<Vec<_>>())
        .collect();
    multi_word_units.sort_by(|a, b| b.len().cmp(&a.len()).then_with(|| a.cmp(b)));

    let input = input.trim();
    let mut idx = 0;
    let input_bytes = input.as_bytes();
    let len = input.len();

    'outer: while idx < len {
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

        // Compute punct-split parts once; reused for both compound check and fallthrough.
        let raw_parts = split_punct_with_offsets(raw, start);

        // Try compound unit match before single-token classification.
        if let Some((compound_tokens, new_idx)) =
            try_compound_unit(input, input_bytes, len, end, &raw_parts, &multi_word_units)
        {
            tokens.extend(compound_tokens);
            idx = new_idx;
            continue 'outer;
        }

        for (sub, sub_start_offset, sub_len) in raw_parts {
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
