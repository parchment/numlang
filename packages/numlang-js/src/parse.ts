const ones: { [key: string]: number } = {
  zero: 0,
  one: 1,
  two: 2,
  three: 3,
  four: 4,
  five: 5,
  six: 6,
  seven: 7,
  eight: 8,
  nine: 9,
  ten: 10,
  eleven: 11,
  twelve: 12,
  thirteen: 13,
  fourteen: 14,
  fifteen: 15,
  sixteen: 16,
  seventeen: 17,
  eighteen: 18,
  nineteen: 19,
};

const tens: { [key: string]: number } = {
  twenty: 20,
  thirty: 30,
  forty: 40,
  fifty: 50,
  sixty: 60,
  seventy: 70,
  eighty: 80,
  ninety: 90,
};

const scales: { [key: string]: number } = {
  hundred: 100,
  thousand: 1_000,
  million: 1_000_000,
  billion: 1_000_000_000,
  trillion: 1_000_000_000_000,
};

const freqAdverbs: { [key: string]: number } = {
  once: 1,
  twice: 2,
  thrice: 3,
};

export function fromWords(s: string): number {
  // Try direct number string first
  const sTrim = s.trim();
  if (/^-?\d+(\.\d+)?$/.test(sTrim)) return parseFloat(sTrim);

  const sLower = sTrim.toLowerCase().replace(/-/g, " ");
  if (freqAdverbs[sLower]) return freqAdverbs[sLower];

  const tokens = sLower.split(/\s+/);
  let result = 0,
    current = 0,
    negative = false,
    i = 0;
  let decimalStr = "";

  while (i < tokens.length) {
    const token = tokens[i];
    if (token === "negative") {
      negative = true;
    } else if (token === "point") {
      i++;
      while (i < tokens.length) {
        const d = ones[tokens[i]];
        if (typeof d === "number" && d >= 0 && d <= 9) {
          decimalStr += d.toString();
        } else {
          throw new Error(`Invalid decimal digit: ${tokens[i]}`);
        }
        i++;
      }
      break;
    } else if (ones[token] !== undefined) {
      current += ones[token];
    } else if (tens[token] !== undefined) {
      current += tens[token];
    } else if (scales[token] !== undefined) {
      if (current === 0) current = 1;
      current *= scales[token];
      if (scales[token] >= 1000) {
        result += current;
        current = 0;
      }
    } else if (token === "and") {
      // skip "and"
    } else if (freqAdverbs[token] !== undefined) {
      return freqAdverbs[token];
    } else {
      throw new Error(`Unknown token: ${token}`);
    }
    i++;
  }
  result += current;
  let value = result;
  if (decimalStr) value += parseFloat("0." + decimalStr);
  if (negative) value = -value;
  return value;
}
