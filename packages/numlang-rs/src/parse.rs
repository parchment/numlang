pub fn parse_words(s: &str) -> Result<f64, String> {
    use std::collections::HashMap;

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

    let mut word_map = HashMap::new();
    for &(w, v) in ones.iter().chain(tens.iter()).chain(scales.iter()) {
        word_map.insert(w, v);
    }

    let s_clean = s.replace("-", " ");
    let tokens: Vec<&str> = s_clean.split_whitespace().collect();

    let mut result = 0i64;
    let mut current = 0i64;
    let mut negative = false;
    let mut i = 0;
    let len = tokens.len();

    // Find "point" if present
    let mut decimal_str = String::new();
    while i < len {
        let token = tokens[i];
        if token == "negative" {
            negative = true;
        } else if token == "point" {
            i += 1;
            // Parse each word after "point" as a digit
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
        } else {
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
    if negative {
        value = -value;
    }
    Ok(value)
}
