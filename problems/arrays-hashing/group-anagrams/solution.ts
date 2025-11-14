/**
 * Group Anagrams
 *
 * Pattern: arrays-hashing
 * Difficulty: Medium
 *
 * Approach:
 * 1. Create a map
 * 2. Loop through all the strings in the array
 *    1. Sort each string
 *    2. if sorted string not in map
 *       1. set it to an empty array
 *    3. else push the string currently being visited into the array corresponding to the sorted string key
 * 4. Loop through the keys of the object or use a method to grab all the values from the map and return the grouped anagrams
 * Time Complexity: O(m * nlogn) n - str len, m - array len
 * Space Complexity: O(m + n)
 */

export function groupAnagrams(strs: string[]): string[][] {
  const map: Record<string, string[]> = {};

  for (const str of strs) { // Note: for...in (loops over properties) should not be used to iterate over an Array where the index order is important.
    const sortedStr = str.split('').sort().join('');
    if (!map[sortedStr]) {
      map[sortedStr] = [];
    }
    map[sortedStr].push(str); // so that the first str also gets grouped!
  }

  const groupedAnagrams = Object.values(map);
  return groupedAnagrams;
}
