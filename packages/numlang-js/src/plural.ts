const irregularPlurals: Record<string, string> = {
  foot: "feet",
  inch: "inches",
  mouse: "mice",
  tooth: "teeth",
  tablet: "tablets",
  capsule: "capsules",
  patch: "patches",
  suppository: "suppositories",
  dose: "doses",
  chew: "chews",
  vial: "vials",
  ampule: "ampules",
  drop: "drops",
  spray: "sprays",
  puff: "puffs",
};

const irregularSingulars: Record<string, string> = Object.fromEntries(
  Object.entries(irregularPlurals).map(([sing, plur]) => [plur, sing]),
);

const knownAbbreviations = new Set([
  "mg",
  "ml",
  "cc",
  "g",
  "kg",
  "mcg",
  "μg",
  "lb",
  "oz",
  "iu",
  "IU",
  "u",
  "meq",
  "mm",
  "cm",
  "m",
  "km",
  "in",
  "ft",
  "yd",
  "mi",
  "s",
  "sec",
  "min",
  "h",
  "hr",
  "d",
  "wk",
  "mo",
  "yr",
  "tab",
  "cap",
  "supp",
  "amp",
  "app",
  // Add more as needed
]);

function isAbbreviation(s: string): boolean {
  const str = s.trim();
  if (knownAbbreviations.has(str)) return true;
  if (str.length === 0 || str.length > 4) return false;
  if (!/^[a-zA-Z]+$/.test(str)) return false;
  return !/[aeiou]/i.test(str);
}

export function toPlural(singular: string): string {
  if (isAbbreviation(singular)) return singular;
  if (irregularPlurals[singular]) return irregularPlurals[singular];
  if (
    singular.endsWith("y") &&
    singular.length > 1 &&
    !"aeiou".includes(singular[singular.length - 2].toLowerCase())
  ) {
    return singular.slice(0, -1) + "ies";
  }
  if (
    singular.endsWith("s") ||
    singular.endsWith("x") ||
    singular.endsWith("z") ||
    singular.endsWith("ch") ||
    singular.endsWith("sh")
  ) {
    return singular + "es";
  }
  return singular + "s";
}

export function toSingular(unit: string): string {
  // Handle pluralized abbreviations like "mls" -> "ml"
  if (unit.length > 2 && unit.endsWith("s")) {
    const candidate = unit.slice(0, -1);
    if (isAbbreviation(candidate)) return candidate;
  }
  if (isAbbreviation(unit)) return unit;
  if (irregularSingulars[unit]) return irregularSingulars[unit];
  if (unit.endsWith("ies") && unit.length > 3) {
    return unit.slice(0, -3) + "y";
  }
  if (
    unit.endsWith("ses") ||
    unit.endsWith("xes") ||
    unit.endsWith("zes") ||
    unit.endsWith("ches") ||
    unit.endsWith("shes")
  ) {
    return unit.slice(0, -2);
  }
  if (unit.endsWith("s") && unit.length > 1) {
    return unit.slice(0, -1);
  }
  return unit;
}
