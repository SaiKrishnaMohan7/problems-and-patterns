import { describe, test, expect } from 'vitest';
import { topKFrequent } from './solution';

// Helper to sort arrays for comparison (order doesn't matter for same frequency)
function sortArray(arr: number[]): number[] {
  return arr.sort((a, b) => a - b);
}

describe('Top K Frequent Elements', () => {
  test('Example 1: Multiple elements with different frequencies', () => {
    const nums = [1, 1, 1, 2, 2, 3];
    const k = 2;
    const result = topKFrequent(nums, k);

    expect(sortArray(result)).toEqual(sortArray([1, 2]));
  });

  test('Example 2: Single unique element', () => {
    const nums = [1];
    const k = 1;
    const result = topKFrequent(nums, k);

    expect(result).toEqual([1]);
  });

  test('Example 3: All elements have same frequency', () => {
    const nums = [1, 2, 3];
    const k = 3;
    const result = topKFrequent(nums, k);

    expect(result.length).toBe(3);
    expect(sortArray(result)).toEqual([1, 2, 3]);
  });

  test('Edge case: k equals array length', () => {
    const nums = [4, 4, 5, 5, 6, 6];
    const k = 3;
    const result = topKFrequent(nums, k);

    expect(result.length).toBe(3);
    expect(sortArray(result)).toEqual([4, 5, 6]);
  });

  test('Edge case: k = 1', () => {
    const nums = [1, 1, 1, 2, 2, 3];
    const k = 1;
    const result = topKFrequent(nums, k);

    expect(result).toEqual([1]);
  });

  test('Complex case: Negative numbers', () => {
    const nums = [-1, -1, -1, -2, -2, 3];
    const k = 2;
    const result = topKFrequent(nums, k);

    expect(sortArray(result)).toEqual(sortArray([-1, -2]));
  });

  test('Complex case: Large frequency difference', () => {
    const nums = [1, 1, 1, 1, 1, 2, 3, 3];
    const k = 2;
    const result = topKFrequent(nums, k);

    expect(sortArray(result)).toEqual(sortArray([1, 3]));
  });
});
