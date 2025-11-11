use crate::string::from_string;
use std::collections::HashMap;

pub fn from_words(s: &str) -> Result<f64, String> {
    // Try to parse as a number string first
    if let Ok(num) = from_string(s) {
        return Ok(num);
    }

    let freq_adverbs = [("once", 1.0), ("twice", 2.0), ("thrice", 3.0)];

    let s_lower = s.to_lowercase();
    for &(w, v) in freq_adverbs.iter() {
        if s_lower.contains(w) {
            return Ok(v);
        }
    }

    let ones = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("ten", 10),
        ("eleven", 11),
        ("twelve", 12),
        ("thirteen", 13),
        ("fourteen", 14),
        ("fifteen", 15),
        ("sixteen", 16),
        ("seventeen", 17),
        ("eighteen", 18),
        ("nineteen", 19),
    ];
    let tens = [
        ("twenty", 20),
        ("thirty", 30),
        ("forty", 40),
        ("fifty", 50),
        ("sixty", 60),
        ("seventy", 70),
        ("eighty", 80),
        ("ninety", 90),
    ];
    let scales = [
        ("hundred", 100),
        ("thousand", 1_000),
        ("million", 1_000_000),
        ("billion", 1_000_000_000),
        ("trillion", 1_000_000_000_000),
    ];
    let fractions = [
        ("half", 2.0),
        ("halves", 2.0),
        ("third", 3.0),
        ("thirds", 3.0),
        ("quarter", 4.0),
        ("quarters", 4.0),
        ("fourth", 4.0),
        ("fourths", 4.0),
        ("fifth", 5.0),
        ("fifths", 5.0),
        ("sixth", 6.0),
        ("sixths", 6.0),
        ("seventh", 7.0),
        ("sevenths", 7.0),
        ("eighth", 8.0),
        ("eighths", 8.0),
        ("ninth", 9.0),
        ("ninths", 9.0),
        ("tenth", 10.0),
        ("tenths", 10.0),
    ];

    let mut word_map = HashMap::new();
    for &(w, v) in ones.iter().chain(tens.iter()).chain(scales.iter()) {
        word_map.insert(w, v);
    }

    let mut fraction_map = HashMap::new();
    for &(w, denom) in fractions.iter() {
        fraction_map.insert(w, denom);
    }

    let s_clean = s_lower.replace("-", " ");
    let tokens: Vec<&str> = s_clean.split_whitespace().collect();

    let mut result = 0i64;
    let mut current = 0i64;
    let mut negative = false;
    let mut i = 0;
    let len = tokens.len();

    let mut decimal_str = String::new();
    let mut fraction_value = 0.0;
    let mut found_fraction = false;

    while i < len {
        let token = tokens[i];
        if token == "negative" {
            negative = true;
        } else if token == "point" {
            i += 1;
            while i < len {
                if let Some(&v) = word_map.get(tokens[i]) {
                    if v >= 0 && v <= 9 {
                        decimal_str.push_str(&v.to_string());
                    } else {
                        return Err(format!("Invalid decimal digit: {}", tokens[i]));
                    }
                } else {
                    return Err(format!("Unknown token: {}", tokens[i]));
                }
                i += 1;
            }
            break;
        } else if token == "and" {
            i += 1;
            let mut numerator = 1;
            if i < len && tokens[i] == "a" {
                numerator = 1;
                i += 1;
            } else if i < len {
                if let Some(&v) = word_map.get(tokens[i]) {
                    numerator = v;
                    i += 1;
                }
            }
            if i < len {
                let frac_token = tokens[i];
                if let Some(&denom) = fraction_map.get(frac_token) {
                    fraction_value = numerator as f64 / denom;
                    found_fraction = true;
                } else {
                    return Err(format!("Unknown fraction: {}", frac_token));
                }
            }
            break;
        } else if let Some(&denom) = fraction_map.get(token) {
            let numerator = if current > 0 { current } else { 1 };
            fraction_value = numerator as f64 / denom;
            found_fraction = true;
            current = 0;
            break;
        } else if let Some(&v) = word_map.get(token) {
            if v < 100 {
                current += v;
            } else {
                if current == 0 {
                    current = 1;
                }
                current *= v;
                if v >= 1000 {
                    result += current;
                    current = 0;
                }
            }
        } else if token == "a" {
            current += 1;
        } else {
            if let Some(&denom) = fraction_map.get(token) {
                if current > 0 {
                    fraction_value = current as f64 / denom;
                    found_fraction = true;
                    current = 0;
                    break;
                }
            }
            return Err(format!("Unknown token: {}", token));
        }
        i += 1;
    }
    result += current;
    let mut value = result as f64;
    if !decimal_str.is_empty() {
        let decimal_val: f64 = format!("0.{}", decimal_str).parse().unwrap_or(0.0);
        value += decimal_val;
    }
    if found_fraction {
        value += fraction_value;
    }
    if negative {
        value = -value;
    }
    Ok(value)
}
