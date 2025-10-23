import { NumberToWordsOptions, defaultOptions } from "./options";

const ones = [
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

const tens = [
  "",
  "",
  "twenty",
  "thirty",
  "forty",
  "fifty",
  "sixty",
  "seventy",
  "eighty",
  "ninety",
];

const scales = [
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

function convertLessThanOneThousand(
  number: number,
  options: NumberToWordsOptions = {},
): string {
  const mergedOptions = { ...defaultOptions, ...options };
  if (number === 0) return "";
  let words = "";
  if (number < 20) {
    words = ones[number];
  } else if (number < 100) {
    const remainder = number % 10;
    words = tens[Math.floor(number / 10)];
    if (remainder > 0) {
      words += "-" + ones[remainder];
    }
  } else {
    const remainder = number % 100;
    words = ones[Math.floor(number / 100)] + " hundred";
    if (remainder > 0) {
      words +=
        (mergedOptions.useAnd ? " and " : " ") +
        convertLessThanOneThousand(remainder, mergedOptions);
    }
  }
  return words;
}

export function toWords(
  number: number | string,
  options: NumberToWordsOptions = {},
): string {
  let num: number;
  if (typeof number === "string") {
    num = parseInt(number, 10);
    if (isNaN(num)) throw new Error("Invalid number string provided");
  } else if (typeof number === "number") {
    if (!isFinite(number) || isNaN(number))
      throw new Error("Invalid number provided");
    num = number;
  } else {
    throw new Error(
      "Input must be a number or a string representation of a number",
    );
  }
  num = Math.floor(num);
  const mergedOptions = { ...defaultOptions, ...options };
  if (num === 0) return "zero";
  let prefix = "";
  if (num < 0) {
    prefix = "negative ";
    num = Math.abs(num);
  }
  let words = "";
  let scaleIndex = 0;
  while (num > 0) {
    const chunk = num % 1000;
    if (chunk > 0) {
      const chunkWords = convertLessThanOneThousand(chunk, mergedOptions);
      const scale = scales[scaleIndex];
      words =
        chunkWords +
        (scale ? " " + scale : "") +
        (words && mergedOptions.useCommas ? ", " : words ? " " : "") +
        words;
    }
    scaleIndex++;
    num = Math.floor(num / 1000);
  }
  let result = prefix + words.trim();
  if (mergedOptions.appendOnly) result += " only";
  if (mergedOptions.capitalize)
    result = result.charAt(0).toUpperCase() + result.slice(1);
  if (mergedOptions.uppercase) result = result.toUpperCase();
  return result;
}

export { scales, ones, tens };
