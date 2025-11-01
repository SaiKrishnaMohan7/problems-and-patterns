/**
 * Two Sum
 *
 * Pattern: arrays-hashing
 * Difficulty: Easy
 *
 * Approach:
 * Trick: the array is unsorted so there are two ways to solve this problem
	- You could sort the array adn run two pointers through it [TwoSum2] OR
	- create map of elements to idx and check for the presence of the difference in the map!

  1. Create a map that will hold the array elements as key and array index as value
2. Loop through the array of elements
	1. calculate the diffference `target - nums[i]`
	2. if the difference is in the map
		1. Push the index of the difference and the current index into the array
		2. break out of the loop and return
	2. else set `nums[i]` as key and the curr index as the value in the map
 *
 * Time Complexity: O(n)
 * Space Complexity: O(n)
 */

export function solutionFunction(nums: number[], target: number): number[] {
  const numMap = new Map<number, number>();
  const indices: number[] = [];

  for (let i = 0; i < nums.length; i++) {
    const difference = target - nums[i];
    if (numMap.has(difference)) {
      const diffIdx = numMap.get(difference);
      if (diffIdx !== undefined) {
        indices.push(diffIdx, i);
      }
        break;
      }
    numMap.set(nums[i], i);
  }

  return indices;
}
