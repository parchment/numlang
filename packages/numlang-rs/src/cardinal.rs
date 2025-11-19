use crate::Options;

pub const ONES: [&str; 20] = [
    "",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

pub const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

pub const SCALES: [&str; 12] = [
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
    "sextillion",
    "septillion",
    "octillion",
    "nonillion",
    "decillion",
];

fn convert_less_than_one_thousand(n: u16, options: &Options) -> String {
    if n == 0 {
        return "".to_string();
    }
    if n < 20 {
        return ONES[n as usize].to_string();
    }
    if n < 100 {
        let ten = n / 10;
        let rem = n % 10;
        if rem == 0 {
            TENS[ten as usize].to_string()
        } else {
            format!("{}-{}", TENS[ten as usize], ONES[rem as usize])
        }
    } else {
        let hundred = n / 100;
        let rem = n % 100;
        let and = if options.use_and && rem > 0 {
            " and "
        } else {
            " "
        };
        if rem == 0 {
            format!("{} hundred", ONES[hundred as usize])
        } else {
            format!(
                "{} hundred{}{}",
                ONES[hundred as usize],
                and,
                convert_less_than_one_thousand(rem, options)
            )
        }
    }
}

fn convert_decimal_part(decimal: &str) -> String {
    decimal
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| if d == 0 { "zero" } else { ONES[d as usize] })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn to_words(number: impl Into<f64>, options: &Options) -> String {
    let num = number.into();
    let mut prefix = String::new();
    let mut abs_num = num;
    if num < 0.0 {
        prefix = "negative ".to_string();
        abs_num = -num;
    }

    // Convert to string to preserve decimal precision
    let num_str = format!("{}", abs_num);
    let parts: Vec<&str> = num_str.split('.').collect();
    let int_part = parts[0];
    let decimal_part = if parts.len() > 1 { parts[1] } else { "" };

    let int_num: i64 = int_part.parse().unwrap_or(0);

    let mut words = if int_num == 0 {
        "zero".to_string()
    } else {
        let mut n = int_num;
        let mut scale = 0;
        let mut chunks = Vec::new();
        while n > 0 {
            let chunk = (n % 1000) as u16;
            if chunk > 0 {
                let mut chunk_str = convert_less_than_one_thousand(chunk, options);
                if !SCALES[scale].is_empty() {
                    chunk_str.push(' ');
                    chunk_str.push_str(SCALES[scale]);
                }
                chunks.push(chunk_str);
            }
            n /= 1000;
            scale += 1;
        }
        chunks.reverse();
        if options.use_commas {
            chunks.join(", ")
        } else {
            chunks.join(" ")
        }
    };

    if !decimal_part.is_empty() {
        words.push_str(" point ");
        words.push_str(&convert_decimal_part(decimal_part));
    }

    let mut result = prefix + &words;
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
