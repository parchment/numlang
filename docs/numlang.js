// numlang.js - Browser bundle of numlang library

(function (global) {
  "use strict";

  // Default options
  const defaultOptions = {
    useCommas: false,
    useAnd: false,
    appendOnly: false,
    uppercase: false,
    capitalize: false,
  };

  // Cardinal numbers
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

  // Ordinal suffixes and words
  const ordinalSuffixes = {
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

  const ordinalOnes = {
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

  const ordinalTens = {
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

  function convertLessThanOneThousand(number, options = {}) {
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

  function convertDecimalPart(decimal) {
    return decimal
      .split("")
      .map((d) => {
        const n = Number(d);
        return n >= 0 && n <= 9 ? ones[n] : "";
      })
      .filter(Boolean)
      .join(" ");
  }

  function toWords(number, options = {}) {
    let num;
    let original = number;

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

  function stringIncludes(str, substr) {
    return str.indexOf(substr) !== -1;
  }

  function isFloat(n) {
    if (typeof n === "number") return n % 1 !== 0;
    if (typeof n === "string")
      return n.includes(".") && !Number.isNaN(parseFloat(n));
    return false;
  }

  function toOrdinal(number) {
    let num;
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

  function toWordsOrdinal(number, options = {}) {
    const mergedOptions = { ...defaultOptions, ...options };
    let num;
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
        if (ordinalOnes[lastWord])
          return parts[0] + "-" + ordinalOnes[lastWord];
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

  // Export the library
  const numlang = {
    toWords: toWords,
    toOrdinal: toOrdinal,
    toWordsOrdinal: toWordsOrdinal,
  };

  // Browser export
  if (typeof window !== "undefined") {
    window.numlang = numlang;
  }

  // Node.js export
  if (typeof module !== "undefined" && module.exports) {
    module.exports = numlang;
  }
})(this);
