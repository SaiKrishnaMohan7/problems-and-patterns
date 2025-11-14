import { longestConsecutive } from './solution';

describe('Longest Consecutive Sequence', () => {
  test('Example 1: unsorted array with consecutive sequence', () => {
    expect(longestConsecutive([100, 4, 200, 1, 3, 2])).toBe(4);
    // Explanation: The longest consecutive sequence is [1, 2, 3, 4]
  });

  test('Example 2: array with duplicates', () => {
    expect(longestConsecutive([0, 3, 7, 2, 5, 8, 4, 6, 0, 1])).toBe(9);
    // Explanation: The longest consecutive sequence is [0, 1, 2, 3, 4, 5, 6, 7, 8]
  });

  test('Edge case: empty array', () => {
    expect(longestConsecutive([])).toBe(0);
  });

  test('Edge case: single element', () => {
    expect(longestConsecutive([1])).toBe(1);
  });

  test('Edge case: no consecutive numbers', () => {
    expect(longestConsecutive([10, 30, 50])).toBe(1);
  });

  test('Edge case: all consecutive', () => {
    expect(longestConsecutive([1, 2, 3, 4, 5])).toBe(5);
  });

  test('Edge case: negative numbers', () => {
    expect(longestConsecutive([-1, -2, 0, 1, 2])).toBe(5);
    // Explanation: The sequence is [-2, -1, 0, 1, 2]
  });
});
