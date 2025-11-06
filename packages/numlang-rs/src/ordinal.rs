use crate::cardinal::to_words;
use crate::Options;

const ORDINAL_SUFFIXES: [&str; 10] = ["th", "st", "nd", "rd", "th", "th", "th", "th", "th", "th"];

const ORDINAL_ONES: [(&str, &str); 9] = [
    ("one", "first"),
    ("two", "second"),
    ("three", "third"),
    ("four", "fourth"),
    ("five", "fifth"),
    ("six", "sixth"),
    ("seven", "seventh"),
    ("eight", "eighth"),
    ("nine", "ninth"),
];

const ORDINAL_TENS: [(&str, &str); 8] = [
    ("ten", "tenth"),
    ("twenty", "twentieth"),
    ("thirty", "thirtieth"),
    ("forty", "fortieth"),
    ("fifty", "fiftieth"),
    ("sixty", "sixtieth"),
    ("seventy", "seventieth"),
    ("eighty", "eightieth"),
    // ninety handled below
];

pub fn to_ordinal(number: impl Into<i64>) -> String {
    let num = number.into();
    if num < 0 {
        return format!("-{}", to_ordinal(-num));
    }
    let abs_num = num.abs();
    if abs_num % 100 >= 11 && abs_num % 100 <= 13 {
        format!("{}th", num)
    } else {
        let last_digit = (abs_num % 10) as usize;
        format!("{}{}", num, ORDINAL_SUFFIXES[last_digit])
    }
}

pub fn to_words_ordinal(number: impl Into<i64>, options: &Options) -> String {
    let num = number.into();
    if num == 0 {
        return "zeroth".to_string();
    }
    if num < 0 {
        return format!("negative {}", to_words_ordinal(-num, options));
    }
    let words = to_words(num as f64, options);

    // Check for simple cases
    for &(card, ord) in ORDINAL_ONES.iter() {
        if words == card {
            return format_with_options(ord, options);
        }
    }
    for &(card, ord) in ORDINAL_TENS.iter() {
        if words == card {
            return format_with_options(ord, options);
        }
    }
    if words == "ninety" {
        return format_with_options("ninetieth", options);
    }

    // Handle hyphenated (e.g., twenty-one)
    if let Some(idx) = words.find('-') {
        let (prefix, last) = words.split_at(idx + 1);
        let last = &last[0..];
        for &(card, ord) in ORDINAL_ONES.iter() {
            if last == card {
                return format_with_options(&format!("{}{}", prefix, ord), options);
            }
        }
    }

    // Handle last word (e.g., one thousand two hundred thirty-four)
    if let Some(idx) = words.rfind(' ') {
        let (prefix, last) = words.split_at(idx + 1);
        let last = &last[0..];
        for &(card, ord) in ORDINAL_ONES.iter() {
            if last == card {
                return format_with_options(&format!("{}{}", prefix, ord), options);
            }
        }
        for &(card, ord) in ORDINAL_TENS.iter() {
            if last == card {
                return format_with_options(&format!("{}{}", prefix, ord), options);
            }
        }
        if last == "ninety" {
            return format_with_options(&format!("{}ninetieth", prefix), options);
        }
    }

    // Default: add "th"
    format_with_options(&format!("{}th", words), options)
}

fn format_with_options(s: &str, options: &Options) -> String {
    let mut result = s.to_string();
    if options.append_only {
        result.push_str(" only");
    }
    if options.capitalize && !result.is_empty() {
        let mut chars = result.chars();
        if let Some(first) = chars.next() {
            result = first.to_uppercase().collect::<String>() + chars.as_str();
        }
    }
    if options.uppercase {
        result = result.to_uppercase();
    }
    result
}
