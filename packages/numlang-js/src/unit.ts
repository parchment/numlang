const unitMap: Record<string, string> = {
  // Volume
  ml: "milliliter",
  cc: "cubic centimeter",
  l: "liter",
  dl: "deciliter",
  ul: "microliter",
  "fl oz": "fluid ounce",
  gal: "gallon",
  qt: "quart",
  pt: "pint",
  tbsp: "tablespoon",
  tsp: "teaspoon",
  gtt: "drop",
  spray: "spray",
  puff: "puff",
  "mg/kg": "milligram per kilogram",
  "ml/kg": "milliliter per kilogram",
  "mcg/kg": "microgram per kilogram",
  "ug/kg": "microgram per kilogram",

  // Mass/weight
  mg: "milligram",
  g: "gram",
  kg: "kilogram",
  mcg: "microgram",
  μg: "microgram",
  lb: "pound",
  oz: "ounce",
  iu: "international unit",
  u: "unit",
  meq: "milliequivalent",

  // Length
  mm: "millimeter",
  cm: "centimeter",
  m: "meter",
  km: "kilometer",
  in: "inch",
  ft: "foot",
  yd: "yard",
  mi: "mile",

  // Time
  s: "second",
  sec: "second",
  min: "minute",
  h: "hour",
  hr: "hour",
  d: "day",
  wk: "week",
  mo: "month",
  yr: "year",

  // Dosage forms
  tab: "tablet",
  cap: "capsule",
  supp: "suppository",
  amp: "ampule",
  vial: "vial",
  patch: "patch",
  chew: "chew",
  dose: "dose",
  sachet: "sachet",
  pump: "pump",
  app: "application",
  pipette: "pipette",
};

function toPlural(s: string): string {
  // Simple pluralisation for units
  if (s.endsWith("y")) return s.slice(0, -1) + "ies";
  if (
    s.endsWith("ch") ||
    s.endsWith("sh") ||
    s.endsWith("s") ||
    s.endsWith("x") ||
    s.endsWith("z")
  )
    return s + "es";
  return s + "s";
}

function reverseUnitMap(): Record<string, string> {
  const m: Record<string, string> = {};
  for (const [abbr, expanded] of Object.entries(unitMap)) {
    m[expanded] = abbr;
    m[toPlural(expanded)] = abbr;
  }
  return m;
}

const reverseMap = reverseUnitMap();

export function expandUnit(unit: string): string | undefined {
  const key = unit.trim().toLowerCase();
  return unitMap[key];
}

export function abbreviateUnit(expanded: string): string | undefined {
  const key = expanded.trim().toLowerCase();
  return reverseMap[key];
}
