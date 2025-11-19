export type Token =
  | { type: "NumberWord"; value: string }
  | { type: "NumberString"; value: string }
  | { type: "Unit"; value: string }
  | { type: "Unknown"; value: string };

export interface TokenSpan {
  token: Token;
  start: number;
  end: number;
}

const numberWordsSet = new Set([
  "zero",
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
  "twenty",
  "thirty",
  "forty",
  "fifty",
  "sixty",
  "seventy",
  "eighty",
  "ninety",
  "hundred",
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
  "point",
  "negative",
  "and",
]);

function isNumberWord(word: string): boolean {
  if (numberWordsSet.has(word)) return true;
  // Hyphenated number word (e.g., "twenty-one")
  return word.split("-").every((w) => numberWordsSet.has(w));
}

function isNumberString(word: string): boolean {
  return /^-?\d+(\.\d+)?$/.test(word);
}

function isUnit(word: string): boolean {
  // Simple heuristic: 2-4 letters, all alpha, no vowels (like "mg", "ml", "cc", "IU")
  const w = word.trim();
  if (w.length < 1 || w.length > 4) return false;
  if (!/^[a-zA-Z]+$/.test(w)) return false;
  return !/[aeiou]/i.test(w);
}

function splitValueUnit(word: string): [string, string] | null {
  // e.g., "200g", "3.5kg"
  const match = /^([0-9]+(?:\.[0-9]+)?)([a-zA-Z]{1,4})$/.exec(word);
  if (match) return [match[1], match[2]];
  return null;
}

function splitPunctWithOffsets(
  token: string,
  tokenStart: number,
): Array<{ sub: string; start: number; end: number }> {
  let start = 0,
    end = token.length;
  // Find first non-punct
  for (let i = 0; i < token.length; i++) {
    if (!isPunct(token[i])) {
      start = i;
      break;
    }
  }
  // Find last non-punct
  for (let i = token.length - 1; i >= 0; i--) {
    if (!isPunct(token[i])) {
      end = i + 1;
      break;
    }
  }
  const out: Array<{ sub: string; start: number; end: number }> = [];
  if (start > 0)
    out.push({
      sub: token.slice(0, start),
      start: tokenStart,
      end: tokenStart + start,
    });
  if (start < end)
    out.push({
      sub: token.slice(start, end),
      start: tokenStart + start,
      end: tokenStart + end,
    });
  if (end < token.length)
    out.push({
      sub: token.slice(end),
      start: tokenStart + end,
      end: tokenStart + token.length,
    });
  return out;
}

function isPunct(c: string): boolean {
  return /[!"#$%&'()*+,\-./:;<=>?@[\\\]^_`{|}~]/.test(c);
}

export function tokenise(input: string): TokenSpan[] {
  const tokens: TokenSpan[] = [];
  const s = input.trim();
  let idx = 0;
  const len = s.length;

  while (idx < len) {
    // Skip whitespace
    while (idx < len && /\s/.test(s[idx])) idx++;
    if (idx >= len) break;

    // Find end of token
    let end = idx;
    while (end < len && !/\s/.test(s[end])) end++;
    const raw = s.slice(idx, end);

    for (const { sub, start, end: subEnd } of splitPunctWithOffsets(raw, idx)) {
      if (!sub) continue;
      const subLc = sub.toLowerCase();

      // Value+unit combos
      const vu = splitValueUnit(sub);
      if (vu) {
        tokens.push({
          token: { type: "NumberString", value: vu[0] },
          start,
          end: start + vu[0].length,
        });
        tokens.push({
          token: { type: "Unit", value: vu[1] },
          start: start + vu[0].length,
          end: subEnd,
        });
        continue;
      }

      // Pure number string
      if (isNumberString(sub)) {
        tokens.push({
          token: { type: "NumberString", value: sub },
          start,
          end: subEnd,
        });
        continue;
      }

      // Number word (including hyphenated)
      if (isNumberWord(subLc)) {
        tokens.push({
          token: { type: "NumberWord", value: sub },
          start,
          end: subEnd,
        });
        continue;
      }

      // Punctuation as unknown
      if ([...sub].every(isPunct)) {
        tokens.push({
          token: { type: "Unknown", value: sub },
          start,
          end: subEnd,
        });
        continue;
      }

      // Unit
      if (isUnit(sub)) {
        tokens.push({
          token: { type: "Unit", value: sub },
          start,
          end: subEnd,
        });
        continue;
      }

      // Unknown
      tokens.push({
        token: { type: "Unknown", value: sub },
        start,
        end: subEnd,
      });
    }
    idx = end;
  }
  return tokens;
}
