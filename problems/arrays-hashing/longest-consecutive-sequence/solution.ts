/**
 * Longest Consecutive Sequence
 *
 * Pattern: arrays-hashing
 * Difficulty: Medium
 *
 * Approach:
 * Use a hash set to store all numbers for O(1) lookup.
 * For each number, check if it's the start of a sequence (no left neighbor).
 * If it is, count how many consecutive numbers exist to the right.
 * Track the maximum sequence length found.
 *
 * Time Complexity: O(n) - Each number is visited at most twice
 * Space Complexity: O(n) - Hash set to store all numbers
 */

// Time: O(n) Space: O(n) as we are creating another ds
export function longestConsecutive(nums: number[]): number {
  // Build out a map of nums array
  let numsValueToIdxMap = new Map();

  for (let i = 0; i < nums.length; ++i) {
    let num = nums[i];
    if (!numsValueToIdxMap.has(num)) {
      numsValueToIdxMap.set(num, i);
    }
  }

  let longest = 0;

  for (let i = 0; i < nums.length; i++) {
    const num = nums[i];
    const leftNeighbor = num - 1; // start of a sequence

    // if leftNeighbor is not in array that means num is the start of the seq
    if (!numsValueToIdxMap.has(leftNeighbor)) {
      let length = 0;
      // If the consecutive right neighbors exist in the map increase length and keep checking for the next in seq
      while(numsValueToIdxMap.has(num + length)) {
        // console.log('num is the start of seq are right neighbors present', length + leftNeighbor);
        ++length;
      }
      longest = Math.max(length, longest);
    }
  }

  return longest;
}
