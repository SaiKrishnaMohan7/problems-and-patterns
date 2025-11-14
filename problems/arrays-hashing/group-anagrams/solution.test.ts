import { describe, test, expect } from 'vitest';
import { groupAnagrams } from './solution';

// Helper function to sort arrays for comparison (order of groups doesn't matter)
function sortGroups(groups: string[][]): string[][] {
  return groups
    .map(group => group.sort())
    .sort((a, b) => a[0].localeCompare(b[0]));
}

describe('Group Anagrams', () => {
  test('Example 1: Basic case with multiple anagram groups', () => {
    const input = ["eat", "tea", "tan", "ate", "nat", "bat"];
    const result = groupAnagrams(input);
    const expected = [["bat"], ["nat", "tan"], ["ate", "eat", "tea"]];

    expect(sortGroups(result)).toEqual(sortGroups(expected));
  });

  test('Example 2: Single empty string', () => {
    const input = [""];
    const result = groupAnagrams(input);
    const expected = [[""]];

    expect(result).toEqual(expected);
  });

  test('Example 3: Single character string', () => {
    const input = ["a"];
    const result = groupAnagrams(input);
    const expected = [["a"]];

    expect(result).toEqual(expected);
  });

  test('Edge case: empty array', () => {
    const input: string[] = [];
    const result = groupAnagrams(input);
    const expected: string[][] = [];

    expect(result).toEqual(expected);
  });

  test('Edge case: no anagrams (all unique words)', () => {
    const input = ["abc", "def", "ghi"];
    const result = groupAnagrams(input);
    const expected = [["abc"], ["def"], ["ghi"]];

    expect(sortGroups(result)).toEqual(sortGroups(expected));
  });

  test('Edge case: all strings are anagrams', () => {
    const input = ["abc", "bca", "cab", "acb"];
    const result = groupAnagrams(input);

    expect(result.length).toBe(1);
    expect(result[0].sort()).toEqual(["abc", "acb", "bca", "cab"]);
  });

  test('Complex case: multiple groups with different lengths', () => {
    const input = ["listen", "silent", "hello", "world", "enlist"];
    const result = groupAnagrams(input);
    const expected = [["hello"], ["listen", "silent", "enlist"], ["world"]];

    expect(sortGroups(result)).toEqual(sortGroups(expected));
  });
});
