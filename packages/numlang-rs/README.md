# numlang

A lightweight Rust library to convert numbers into their linguistic equivalents—and back.

## Features

- Convert numbers to words (cardinal form), including floats (e.g., `12.34` → `"twelve point three four"`)
- Convert numbers to ordinal form (1st, 2nd, 3rd) — **floats not supported**
- Convert numbers to ordinal words (first, second, third) — **floats not supported**
- Supports negative numbers
- Configurable formatting options
- Comprehensive test coverage
- Parse words back to numbers (**integers and floats supported**)

## Installation

Add to your `Cargo.toml`:

```toml
numlang = "0.2"
```

## Usage

```rust
use numlang::{to_words, to_ordinal, to_words_ordinal, Options, parse_words};

// Convert numbers to words (supports floats)
to_words(12.34, &Options::default());            // "twelve point three four"
to_words(-0.56, &Options::default());            // "negative zero point five six"

// Parse words to numbers (integers and floats)
parse_words("forty-two").unwrap();                // 42.0
parse_words("one thousand two hundred thirty-four").unwrap(); // 1234.0
parse_words("negative seven").unwrap();           // -7.0
parse_words("twelve point three four").unwrap();  // 12.34
parse_words("negative zero point five six").unwrap(); // -0.56
parse_words("one hundred twenty-three point four five six").unwrap(); // 123.456
```

### Formatting Options

The `Options` struct customizes output:

```rust
let mut opts = Options::default();
opts.use_and = true;
to_words(123, &opts); // "one hundred and twenty-three"
```

## API

- `to_words(number, &Options)` — Converts a number (integer or float) to words.
- `to_ordinal(number)` — Converts an integer to its ordinal form (e.g., 1st, 2nd).
- `to_words_ordinal(number, &Options)` — Converts an integer to its ordinal word form.
- `parse_words(s: &str)` — Parses number words to a `f64` (supports floats and integers).

## Limitations

- Parsing words to numbers supports floats and integers, but not fractions like "one half".
- Very large numbers (beyond `i64`/`f64` limits) may not be handled accurately.
- Decimal parsing from words expects digit words after "point" (e.g., `"point five six"`).

## License

MIT
