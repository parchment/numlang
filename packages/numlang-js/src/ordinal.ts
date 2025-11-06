import { NumberToWordsOptions, defaultOptions } from "./options";
import { scales, ones, tens, toWords } from "./cardinal";

const ordinalSuffixes: { [key: number]: string } = {
  0: "th",
  1: "st",
  2: "nd",
  3: "rd",
  4: "th",
  5: "th",
  6: "th",
  7: "th",
  8: "th",
  9: "th",
};

const ordinalOnes: { [key: string]: string } = {
  one: "first",
  two: "second",
  three: "third",
  four: "fourth",
  five: "fifth",
  six: "sixth",
  seven: "seventh",
  eight: "eighth",
  nine: "ninth",
};

const ordinalTens: { [key: string]: string } = {
  ten: "tenth",
  twenty: "twentieth",
  thirty: "thirtieth",
  forty: "fortieth",
  fifty: "fiftieth",
  sixty: "sixtieth",
  seventy: "seventieth",
  eighty: "eightieth",
  ninety: "ninetieth",
};

function stringIncludes(str: string, substr: string): boolean {
  return str.indexOf(substr) !== -1;
}

function isFloat(n: number | string): boolean {
  if (typeof n === "number") return n % 1 !== 0;
  if (typeof n === "string")
    return n.includes(".") && !Number.isNaN(parseFloat(n));
  return false;
}

export function toOrdinal(number: number | string): string {
  let num: number;
  if (typeof number === "string") {
    if (isFloat(number)) {
      throw new Error(
        "Ordinal forms for floats are not standard in English. Please provide an integer.",
      );
    }
    num = parseInt(number, 10);
    if (isNaN(num)) throw new Error("Invalid number string provided");
  } else if (typeof number === "number") {
    if (!isFinite(number) || isNaN(number))
      throw new Error("Invalid number provided");
    if (isFloat(number)) {
      throw new Error(
        "Ordinal forms for floats are not standard in English. Please provide an integer.",
      );
    }
    num = number;
  } else {
    throw new Error(
      "Input must be a number or a string representation of a number",
    );
  }
  const absNum = Math.abs(num);
  if (absNum % 100 >= 11 && absNum % 100 <= 13) return num + "th";
  const lastDigit = absNum % 10;
  const suffix = ordinalSuffixes[lastDigit] || "th";
  return num + suffix;
}

export function toWordsOrdinal(
  number: number | string,
  options: NumberToWordsOptions = {},
): string {
  const mergedOptions = { ...defaultOptions, ...options };
  let num: number;
  if (typeof number === "string") {
    if (isFloat(number)) {
      throw new Error(
        "Ordinal word forms for floats are not standard in English. Please provide an integer.",
      );
    }
    num = parseInt(number, 10);
    if (isNaN(num)) throw new Error("Invalid number string provided");
  } else if (typeof number === "number") {
    if (!isFinite(number) || isNaN(number))
      throw new Error("Invalid number provided");
    if (isFloat(number)) {
      throw new Error(
        "Ordinal word forms for floats are not standard in English. Please provide an integer.",
      );
    }
    num = number;
  } else {
    throw new Error(
      "Input must be a number or a string representation of a number",
    );
  }
  if (num === 0) return "zeroth";
  let prefix = "";
  if (num < 0) {
    prefix = "negative ";
    num = Math.abs(num);
  }
  let words = toWords(num, mergedOptions);
  if (prefix) return prefix + toWordsOrdinal(Math.abs(num), mergedOptions);
  for (let i = 1; i < 10; i++) {
    if (words === ones[i]) return ordinalOnes[words];
  }
  for (const key in ordinalTens) {
    if (ordinalTens.hasOwnProperty(key) && words === key)
      return ordinalTens[key];
  }
  if (stringIncludes(words, "-")) {
    const parts = words.split("-");
    if (parts.length === 2) {
      const lastWord = parts[1];
      if (ordinalOnes[lastWord]) return parts[0] + "-" + ordinalOnes[lastWord];
    }
  }
  const lastSpace = words.lastIndexOf(" ");
  if (lastSpace !== -1) {
    const lastWord = words.substring(lastSpace + 1);
    for (let i = 0; i < scales.length; i++) {
      if (scales[i] && lastWord === scales[i])
        return words.substring(0, lastSpace + 1) + lastWord + "th";
    }
    if (ordinalOnes[lastWord])
      return words.substring(0, lastSpace + 1) + ordinalOnes[lastWord];
    if (ordinalTens[lastWord])
      return words.substring(0, lastSpace + 1) + ordinalTens[lastWord];
  }
  return words + "th";
}
