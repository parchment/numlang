use std::collections::HashMap;

pub fn parse_words(s: &str) -> Result<i64, String> {
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

    let mut result = 0i64;
    let mut current = 0i64;
    let mut negative = false;

    let s_clean = s.replace("-", " ");
    let tokens: Vec<&str> = s_clean.split_whitespace().collect();

    for token in tokens {
        if token == "negative" {
            negative = true;
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
    }
    result += current;
    if negative {
        result = -result;
    }
    Ok(result)
}
