import { fromWords } from "../src/parse";
import { describe, it, expect } from "vitest";

describe("fromWords", () => {
  it("parses basic numbers", () => {
    expect(fromWords("zero")).toBe(0);
    expect(fromWords("one")).toBe(1);
    expect(fromWords("forty-two")).toBe(42);
    expect(fromWords("one hundred")).toBe(100);
    expect(fromWords("one thousand two hundred thirty-four")).toBe(1234);
    expect(fromWords("negative seven")).toBe(-7);
  });

  it("parses floats", () => {
    expect(fromWords("twelve point three four")).toBe(12.34);
    expect(fromWords("negative zero point five six")).toBe(-0.56);
    expect(fromWords("one hundred twenty-three point four five six")).toBe(
      123.456,
    );
  });

  it("parses frequency adverbs", () => {
    expect(fromWords("once")).toBe(1);
    expect(fromWords("twice")).toBe(2);
    expect(fromWords("thrice")).toBe(3);
  });

  it("parses number strings", () => {
    expect(fromWords("2")).toBe(2);
    expect(fromWords("12.34")).toBe(12.34);
    expect(fromWords("-7")).toBe(-7);
    expect(fromWords("0")).toBe(0);
  });

  it("throws on unknown tokens", () => {
    expect(() => fromWords("foo")).toThrow();
    expect(() => fromWords("one foo")).toThrow();
  });
});
