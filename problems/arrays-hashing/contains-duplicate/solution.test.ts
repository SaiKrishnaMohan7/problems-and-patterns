import { describe, test, expect } from 'vitest';
import { solutionFunction } from './solution';

describe('Contains Duplicate', () => {
  test('Example 1', () => {
    const input = [1, 2, 3, 1];
    expect(solutionFunction(input)).toBe(true);
  });

  test('Example 2', () => {
    const input = [1, 2, 3, 4];
    expect(solutionFunction(input)).toBe(false);
  });

  test('Edge case: empty input', () => {
    const input: number[] = [];
    expect(solutionFunction(input)).toBe(false);
  });
});
