import { describe, test, expect } from 'vitest';
import { solutionFunction } from './solution';

describe('Valid Anagram', () => {
  test('Example 1', () => {
    const result = solutionFunction("anagram", "nagaram");
    expect(result).toBe(true);
  });

  test('Example 2', () => {
    const result = solutionFunction("rat", "car");
    expect(result).toBe(false);
  });

  test('Edge case: empty input', () => {
    const result = solutionFunction("", "");
    expect(result).toBe(true);
  });
});
