import { tokenise } from "../src/tokenise";
import { describe, it, expect } from "vitest";

describe("tokenise", () => {
  it("handles number words", () => {
    const t = tokenise("twenty-one");
    expect(t[0].token.type).toBe("NumberWord");
    expect(t[0].token.value).toBe("twenty-one");
  });

  it("handles value+unit combos", () => {
    const t = tokenise("3.5kg");
    expect(t[0].token.type).toBe("NumberString");
    expect(t[0].token.value).toBe("3.5");
    expect(t[1].token.type).toBe("Unit");
    expect(t[1].token.value).toBe("kg");
  });

  it("handles punctuation", () => {
    const t = tokenise("days.");
    expect(t[0].token.value).toBe("days");
    expect(t[1].token.value).toBe(".");
  });

  it("handles positions", () => {
    const t = tokenise("100g of sugar");
    expect(t[0].token.value).toBe("100");
    expect(t[0].start).toBe(0);
    expect(t[0].end).toBe(3);
    expect(t[1].token.value).toBe("g");
    expect(t[1].start).toBe(3);
    expect(t[1].end).toBe(4);
    expect(t[2].token.value).toBe("of");
    expect(t[2].start).toBe(5);
    expect(t[2].end).toBe(7);
    expect(t[3].token.value).toBe("sugar");
    expect(t[3].start).toBe(8);
    expect(t[3].end).toBe(13);
  });
});
