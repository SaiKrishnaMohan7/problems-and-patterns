/**
 * Valid Anagram
 *
 * Pattern: arrays-hashing
 * Difficulty: Easy
 *
 * Approach:
 * 1. Check if the two strings are of the same length
 *  1. if not return false
 *  2. else proceed
2. Create two maps or JS objects
3. Loop through the strings and fill out the map with char as key and its count as value
4. Get the keys of one of the objects
	1. loop through that and compare the value of the same key in the other string's map
		1. if the values are the same return true
		2. else even if one doesn't match, break out of the loop and return false
 *
 * Time Complexity: O(n)
 * Space Complexity: O(n)
 */

export function solutionFunction(strA: string, strB: string): boolean {
  if (strA.length !== strB.length) return false;

  const mapA: Record<string, number> = {};
  const mapB: Record<string, number> = {};

  for (let i = 0; i < strA.length; i++) {
    const charA = strA[i];
    const charB = strB[i];

    mapA[charA] = (mapA[charA] || 0) + 1;
    mapB[charB] = (mapB[charB] || 0) + 1;
  }

  for (const key in mapA) {
    if (mapA[key] !== mapB[key]) {
      return false;
    }
  }

  return true;
}
