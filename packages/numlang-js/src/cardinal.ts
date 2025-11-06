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

function convertDecimalPart(decimal: string): string {
  return decimal
    .split("")
    .map((d) => {
      const n = Number(d);
      return n >= 0 && n <= 9 ? ones[n] : "";
    })
    .filter(Boolean)
    .join(" ");
}

export function toWords(
  number: number | string,
  options: NumberToWordsOptions = {},
): string {
  let num: number;
  let original: string | number = number;
  if (typeof number === "string") {
    num = parseFloat(number);
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

  const mergedOptions = { ...defaultOptions, ...options };

  let prefix = "";
  if (num < 0) {
    prefix = "negative ";
    num = Math.abs(num);
    original =
      typeof original === "string" ? original.replace(/^[-]/, "") : num;
  }

  let hasDecimal = false;
  let decimalPart = "";
  let integerPart = 0;

  if (typeof original === "string" && original.includes(".")) {
    hasDecimal = true;
    const [intStr, decStr] = original.split(".");
    integerPart = parseInt(intStr, 10);
    decimalPart = decStr || "";
  } else if (typeof original === "number" && num % 1 !== 0) {
    hasDecimal = true;
    // Try to infer decimal precision from the number itself
    // If the number is like 1000.01, toString() gives "1000.01"
    // If it's 1000.010, toString() gives "1000.01" (trailing zero lost)
    // So, default to 2 decimals for numbers
    const defaultDecimals = 2;
    let decimals = defaultDecimals;
    // If the user wrote 1000.0001, keep 4 decimals
    const numStr = num.toString();
    if (numStr.includes(".")) {
      decimals = numStr.split(".")[1].length;
    }
    const fixedStr = num.toFixed(decimals);
    const [intStr, decStr] = fixedStr.split(".");
    integerPart = parseInt(intStr, 10);
    decimalPart = decStr || "";
  } else {
    integerPart = Math.floor(num);
  }

  let words = "";
  let scaleIndex = 0;
  let intWords = "";

  if (integerPart === 0) {
    intWords = "zero";
  } else {
    let tempNum = integerPart;
    while (tempNum > 0) {
      const chunk = tempNum % 1000;
      if (chunk > 0) {
        const chunkWords = convertLessThanOneThousand(chunk, mergedOptions);
        const scale = scales[scaleIndex];
        intWords =
          chunkWords +
          (scale ? " " + scale : "") +
          (intWords && mergedOptions.useCommas ? ", " : intWords ? " " : "") +
          intWords;
      }
      scaleIndex++;
      tempNum = Math.floor(tempNum / 1000);
    }
  }

  words = intWords.trim();

  if (hasDecimal && decimalPart.length > 0) {
    words += " point " + convertDecimalPart(decimalPart);
  }

  let result = prefix + words;
  if (mergedOptions.appendOnly) result += " only";
  if (mergedOptions.capitalize)
    result = result.charAt(0).toUpperCase() + result.slice(1);
  if (mergedOptions.uppercase) result = result.toUpperCase();
  return result;
}

export { scales, ones, tens };
