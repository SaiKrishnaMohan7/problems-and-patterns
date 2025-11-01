import { describe, test, expect } from 'vitest';
import { solutionFunction } from './solution';

describe('Two Sum', () => {
  test('Example 1', () => {
    const result = solutionFunction([2, 7, 11, 15], 9);
    expect(result).toEqual([0, 1]);
  });

  test('Example 2', () => {
    const result = solutionFunction([3, 2, 4], 6);
    expect(result).toEqual([1, 2]);
  });

  test('Edge case: empty input', () => {
    const result = solutionFunction([], 0);
    expect(result).toEqual([]);
  });
});
