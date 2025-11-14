import { describe, test, expect } from 'vitest';
import { productExceptSelf } from './solution';

describe('Product of Array Except Self', () => {
  test('Example 1: Basic case', () => {
    const nums = [1, 2, 3, 4];
    const result = productExceptSelf(nums);

    expect(result).toEqual([24, 12, 8, 6]);
  });

  test('Example 2: Array with different values', () => {
    const nums = [-1, 1, 0, -3, 3];
    const result = productExceptSelf(nums);

    // Handle -0 vs 0 in JavaScript
    const normalized = result.map(x => x === 0 ? 0 : x);
    expect(normalized).toEqual([0, 0, 9, 0, 0]);
  });

  test('Example 3: Two elements', () => {
    const nums = [1, 2];
    const result = productExceptSelf(nums);

    expect(result).toEqual([2, 1]);
  });

  test('Edge case: Array with ones', () => {
    const nums = [1, 1, 1, 1];
    const result = productExceptSelf(nums);

    expect(result).toEqual([1, 1, 1, 1]);
  });

  test('Edge case: Array with zero at start', () => {
    const nums = [0, 2, 3];
    const result = productExceptSelf(nums);

    expect(result).toEqual([6, 0, 0]);
  });

  test('Edge case: Array with zero at end', () => {
    const nums = [2, 3, 0];
    const result = productExceptSelf(nums);

    expect(result).toEqual([0, 0, 6]);
  });

  test('Edge case: Array with zero in middle', () => {
    const nums = [2, 0, 3];
    const result = productExceptSelf(nums);

    expect(result).toEqual([0, 6, 0]);
  });

  test('Complex case: Negative numbers', () => {
    const nums = [-2, -3, -4];
    const result = productExceptSelf(nums);

    expect(result).toEqual([12, 8, 6]);
  });

  test('Complex case: Mixed positive and negative', () => {
    const nums = [2, -3, 4, -5];
    const result = productExceptSelf(nums);

    expect(result).toEqual([60, -40, 30, -24]);
  });
});
