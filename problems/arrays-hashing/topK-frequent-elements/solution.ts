/**
 * Top K frequent elements
 *
 * Pattern: arrays-hashing
 * Difficulty: Medium
 *
 * IDEA:
 *  Use array indeces as count and store the numbers from nums that match the count under
 *  those indeces in an array. Bucket sort concept used.
 * ALGO IMPL:
 *  Create map to count numbers; array elem as key and count as value
 *  Create the buckets with length 1 greater than input arr length, as we are using indices
 *    to keep track of count and if nums is length 6, it is possible that an element may repeat 6 times
 *    and if we created bucket of the same length as input array (0 to 5 indices) then we won't be able to
 *    count the element repeating 6 times!
 *  Populate the map by counting the numbers; { num: count }
 *  Since we want K MOST FREQUENT, we start looping from behind
 *
 * Time Complexity: O(n)
 * Space Complexity: O(n)
 */

export function topKFrequent(nums: number[], k: number): number[] {
  let freqMap: Map<number, number>;
  let buckets: number[][];
  let result: number[];

  // Build frequency map
  freqMap = new Map<number, number>();
  for (const num of nums) {
    if (freqMap.has(num)) {
      freqMap.set(num, freqMap.get(num)! + 1);
    } else {
      freqMap.set(num, 1);
    }
  }

  // Bucket sort: index = frequency, value = array of numbers with that frequency
  buckets = Array(nums.length + 1).fill(null).map(() => []);

  for (const [num, freq] of freqMap.entries()) {
    buckets[freq].push(num);
  }

  // Collect top k frequent elements
  result = [];
  for (let i = buckets.length - 1; i >= 0 && result.length < k; i--) {
    if (buckets[i].length > 0) {
      result.push(...buckets[i]);
    }
  }

  return result.slice(0, k);
}
