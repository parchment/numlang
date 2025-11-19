import { toPlural, toSingular } from "../src/plural";
import { describe, it, expect } from "vitest";

describe("toPlural", () => {
  it("regular", () => {
    expect(toPlural("tablet")).toBe("tablets");
    expect(toPlural("patch")).toBe("patches");
    expect(toPlural("capsule")).toBe("capsules");
    expect(toPlural("dose")).toBe("doses");
    expect(toPlural("chew")).toBe("chews");
    expect(toPlural("vial")).toBe("vials");
    expect(toPlural("ampule")).toBe("ampules");
    expect(toPlural("drop")).toBe("drops");
    expect(toPlural("spray")).toBe("sprays");
    expect(toPlural("puff")).toBe("puffs");
  });

  it("irregular", () => {
    expect(toPlural("foot")).toBe("feet");
    expect(toPlural("inch")).toBe("inches");
    expect(toPlural("mouse")).toBe("mice");
    expect(toPlural("tooth")).toBe("teeth");
  });

  it("abbreviation", () => {
    expect(toPlural("mg")).toBe("mg");
    expect(toPlural("ml")).toBe("ml");
    expect(toPlural("IU")).toBe("IU");
    expect(toPlural("cc")).toBe("cc");
  });
});

describe("toSingular", () => {
  it("regular", () => {
    expect(toSingular("tablets")).toBe("tablet");
    expect(toSingular("patches")).toBe("patch");
    expect(toSingular("capsules")).toBe("capsule");
    expect(toSingular("doses")).toBe("dose");
    expect(toSingular("chews")).toBe("chew");
    expect(toSingular("vials")).toBe("vial");
    expect(toSingular("ampules")).toBe("ampule");
    expect(toSingular("drops")).toBe("drop");
    expect(toSingular("sprays")).toBe("spray");
    expect(toSingular("puffs")).toBe("puff");
  });

  it("irregular", () => {
    expect(toSingular("feet")).toBe("foot");
    expect(toSingular("inches")).toBe("inch");
    expect(toSingular("mice")).toBe("mouse");
    expect(toSingular("teeth")).toBe("tooth");
  });

  it("abbreviation", () => {
    expect(toSingular("mg")).toBe("mg");
    expect(toSingular("ml")).toBe("ml");
    expect(toSingular("IU")).toBe("IU");
    expect(toSingular("cc")).toBe("cc");
  });

  it("pluralized abbreviation", () => {
    expect(toSingular("mls")).toBe("ml");
    expect(toSingular("kgs")).toBe("kg");
    expect(toSingular("mgs")).toBe("mg");
    expect(toSingular("ccs")).toBe("cc");
    expect(toSingular("IUs")).toBe("IU");
  });
});
