# numlang

A lightweight JavaScript library to convert numbers into their linguistic equivalents.

## Features

- Convert numbers to words (cardinal form), including floats (e.g., `12.34` → `"twelve point three four"`)
- Convert numbers to ordinal form (1st, 2nd, 3rd) — **floats not supported**
- Convert numbers to ordinal words (first, second, third) — **floats not supported**
- Supports negative numbers
- Configurable formatting options
- Written in TypeScript with full type definitions
- Zero dependencies
- Comprehensive test coverage

## Installation

### From npmjs (when available)

```bash
npm install numlang
```

Or using yarn:

```bash
yarn add numlang
```

### From GitHub Packages

If installing from GitHub Packages (`@parchment/numlang`), add this to your `.npmrc`:

```
@parchment:registry=https://npm.pkg.github.com
```

Then install:

```bash
npm install @parchment/numlang
```

If the package is private, you’ll also need a GitHub personal access token with `read:packages` scope:

```
 //npm.pkg.github.com/:_authToken=YOUR_GITHUB_TOKEN
```

## Usage

### Basic Usage

```javascript
import { toWords, toOrdinal, toWordsOrdinal } from 'numlang';

// Convert numbers to words (supports floats)
toWords(0);                // "zero"
toWords(1);                // "one"
toWords(42);               // "forty-two"
toWords(100);              // "one hundred"
toWords(1234);             // "one thousand two hundred thirty-four"
toWords(1000000);          // "one million"
toWords(-7);               // "negative seven"
toWords(12.34);            // "twelve point three four"
toWords(-0.56);            // "negative zero point five six"
toWords("123.456");        // "one hundred twenty-three point four five six"

// Convert to ordinals (1st, 2nd, 3rd)
// Floats are **not supported** and will throw an error
toOrdinal(1);              // "1st"
toOrdinal(2);              // "2nd"
toOrdinal(3);              // "3rd"
toOrdinal(11);             // "11th"
toOrdinal(21);             // "21st"
toOrdinal(42);             // "42nd"
toOrdinal(103);            // "103rd"
// toOrdinal(1.5);         // Throws: Ordinal forms for floats are not standard in English.

// Convert to ordinal words (first, second, third)
// Floats are **not supported** and will throw an error
toWordsOrdinal(1);         // "first"
toWordsOrdinal(2);         // "second"
toWordsOrdinal(3);         // "third"
toWordsOrdinal(21);        // "twenty-first"
toWordsOrdinal(100);       // "one hundredth"
toWordsOrdinal(1000);      // "one thousandth"
toWordsOrdinal(1234);      // "one thousand two hundred thirty-fourth"
// toWordsOrdinal(2.7);    // Throws: Ordinal word forms for floats are not standard in English.
```

### Formatting Options

The `toWords` and `toWordsOrdinal` functions accept an options object as the second parameter:

```javascript
// Using the "and" conjunction
toWords(123, { useAnd: true });
// "one hundred and twenty-three"

// Using commas in the output
toWords(1234, { useCommas: true });
// "one thousand, two hundred thirty-four"

// Using both commas and "and"
toWords(1234, { useCommas: true, useAnd: true });
// "one thousand, two hundred and thirty-four"

// Converting to uppercase
toWords(1234, { uppercase: true });
// "ONE THOUSAND TWO HUNDRED THIRTY-FOUR"

// Capitalizing the first letter
toWords(1234, { capitalize: true });
// "One thousand two hundred thirty-four"

// Appending "only"
toWords(1234, { appendOnly: true });
// "one thousand two hundred thirty-four only"
```

These options can also be applied to `toWordsOrdinal`:

```javascript
toWordsOrdinal(1234, { useCommas: true, useAnd: true, capitalize: true });
// "One thousand, two hundred and thirty-fourth"
```

## API

### toWords(number, options?)

Converts a number (integer or float) to words.

- `number`: The number to convert (number or string; floats supported)
- `options`: Optional configuration object:
  - `useAnd`: Boolean - Include "and" in appropriate places (default: false)
  - `useCommas`: Boolean - Include commas in the output (default: false)
  - `appendOnly`: Boolean - Append "only" to the output (default: false)
  - `uppercase`: Boolean - Convert to uppercase (default: false)
  - `capitalize`: Boolean - Capitalize the first letter (default: false)

Returns a string representation of the number in words.

### toOrdinal(number)

Converts a number to its ordinal form (e.g., 1st, 2nd, 3rd).

- `number`: The number to convert (number or string; **must be integer**)

Returns a string with the number and its ordinal suffix.

**Note:**
Floats are not supported and will throw an error.

### toWordsOrdinal(number, options?)

Converts a number to its ordinal word form (e.g., first, second, third).

- `number`: The number to convert (number or string; **must be integer**)
- `options`: Same options as `toWords`

Returns a string with the ordinal word representation of the number.

**Note:**
Floats are not supported and will throw an error.

## Limitations

- For exact decimal precision, pass floats as strings (e.g., `"1000.010"` preserves trailing zeros).
- Very large numbers (beyond JavaScript's Number.MAX_SAFE_INTEGER) may not be handled accurately.

## License

MIT
