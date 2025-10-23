import { toWords } from '../src/cardinal';

import { describe, it, expect } from 'vitest';

describe('toWords', () => {
  it('handles zero', () => {
    expect(toWords(0)).toBe('zero');
  });

  it('handles positive numbers', () => {
    expect(toWords(42)).toBe('forty-two');
    expect(toWords(100)).toBe('one hundred');
    expect(toWords(1234)).toBe('one thousand two hundred thirty-four');
  });

  it('handles negative numbers', () => {
    expect(toWords(-7)).toBe('negative seven');
  });

  it('handles formatting options', () => {
    expect(toWords(123, { useAnd: true })).toBe('one hundred and twenty-three');
    expect(toWords(1234, { useCommas: true })).toBe('one thousand, two hundred thirty-four');
    expect(toWords(1234, { uppercase: true })).toBe('ONE THOUSAND TWO HUNDRED THIRTY-FOUR');
    expect(toWords(1234, { capitalize: true })).toBe('One thousand two hundred thirty-four');
    expect(toWords(1234, { appendOnly: true })).toBe('one thousand two hundred thirty-four only');
  });
});
