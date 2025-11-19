import { expandUnit, abbreviateUnit } from "../src/unit";
import { describe, it, expect } from "vitest";

describe("expandUnit", () => {
  it("expands basic units", () => {
    expect(expandUnit("ml")).toBe("milliliter");
    expect(expandUnit("MG")).toBe("milligram");
    expect(expandUnit("kg")).toBe("kilogram");
    expect(expandUnit("oz")).toBe("ounce");
    expect(expandUnit("IU")).toBe("international unit");
    expect(expandUnit("mcg")).toBe("microgram");
    expect(expandUnit("μg")).toBe("microgram");
    expect(expandUnit("cc")).toBe("cubic centimeter");
    expect(expandUnit("gtt")).toBe("drop");
    expect(expandUnit("tab")).toBe("tablet");
    expect(expandUnit("cap")).toBe("capsule");
    expect(expandUnit("patch")).toBe("patch");
    expect(expandUnit("dose")).toBe("dose");
    expect(expandUnit("unknown")).toBeUndefined();
  });

  it("expands plural and case", () => {
    expect(expandUnit("Tab")).toBe("tablet");
    expect(expandUnit("Cap")).toBe("capsule");
    expect(expandUnit("chew")).toBe("chew");
  });
});

describe("abbreviateUnit", () => {
  it("abbreviates singular", () => {
    expect(abbreviateUnit("milliliter")).toBe("ml");
    expect(abbreviateUnit("tablet")).toBe("tab");
    expect(abbreviateUnit("drop")).toBe("gtt");
    expect(["mcg", "μg"]).toContain(abbreviateUnit("microgram"));
  });

  it("abbreviates plural", () => {
    expect(abbreviateUnit("milliliters")).toBe("ml");
    expect(abbreviateUnit("tablets")).toBe("tab");
    expect(abbreviateUnit("drops")).toBe("gtt");
    expect(["mcg", "μg"]).toContain(abbreviateUnit("micrograms"));
  });

  it("abbreviates case-insensitive", () => {
    expect(abbreviateUnit("Milliliters")).toBe("ml");
    expect(abbreviateUnit("TABLETS")).toBe("tab");
  });

  it("returns undefined for unknown", () => {
    expect(abbreviateUnit("unknownunit")).toBeUndefined();
    expect(abbreviateUnit("")).toBeUndefined();
  });
});
