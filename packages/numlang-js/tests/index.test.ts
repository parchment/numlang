import numlang, { toWords, toOrdinal, toWordsOrdinal } from '../src';

import { describe, it, expect } from 'vitest';

describe('numlang default export', () => {
  it('exports all functions', () => {
    expect(numlang.toWords).toBe(toWords);
    expect(numlang.toOrdinal).toBe(toOrdinal);
    expect(numlang.toWordsOrdinal).toBe(toWordsOrdinal);
  });
});
