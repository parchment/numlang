import { toOrdinal, toWordsOrdinal } from '../src/ordinal';

import { describe, it, expect } from 'vitest';

describe('toOrdinal', () => {
  it('handles basic ordinals', () => {
    expect(toOrdinal(1)).toBe('1st');
    expect(toOrdinal(2)).toBe('2nd');
    expect(toOrdinal(3)).toBe('3rd');
    expect(toOrdinal(11)).toBe('11th');
    expect(toOrdinal(21)).toBe('21st');
    expect(toOrdinal(42)).toBe('42nd');
    expect(toOrdinal(103)).toBe('103rd');
  });
});

describe('toWordsOrdinal', () => {
  it('handles basic ordinal words', () => {
    expect(toWordsOrdinal(1)).toBe('first');
    expect(toWordsOrdinal(2)).toBe('second');
    expect(toWordsOrdinal(3)).toBe('third');
    expect(toWordsOrdinal(21)).toBe('twenty-first');
    expect(toWordsOrdinal(100)).toBe('one hundredth');
    expect(toWordsOrdinal(1000)).toBe('one thousandth');
    expect(toWordsOrdinal(1234)).toBe('one thousand two hundred thirty-fourth');
  });

  it('handles formatting options', () => {
    expect(toWordsOrdinal(1234, { useCommas: true, useAnd: true, capitalize: true }))
      .toBe('One thousand, two hundred and thirty-fourth');
  });
});
