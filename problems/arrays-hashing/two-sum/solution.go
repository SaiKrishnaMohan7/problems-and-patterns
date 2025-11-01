// Package two_sum implements a solution for Two Sum
//
// Pattern: arrays-hashing
// Difficulty: Easy
//
// Approach:
// Trick: the array is unsorted so there are two ways to solve this problem
// 	- You could sort the array adn run two pointers through it [TwoSum2] OR
// 	- create map of elements to idx and check for the presence of the difference in the map!

//   1. Create a map that will hold the array elements as key and array index as value
// 2. Loop through the array of elements
// 	1. calculate the diffference `target - nums[i]`
// 	2. if the difference is in the map
// 		1. Push the index of the difference and the current index into the array
// 		2. break out of the loop and return
// 	2. else set `nums[i]` as key and the curr index as the value in the map
//
// Time Complexity: O(?)
// Space Complexity: O(?)
package two_sum

// SolutionFunction TODO: Implement solution
func SolutionFunction(nums []int, target int) []int {
	numMap := make(map[int]int)
	indices := []int{}

	for i := 0; i < len(nums); i++ {
		difference := target - nums[i]
		idx, exists := numMap[difference]
		if exists {
			indices = append(indices, idx, i)
			break
		}
		numMap[nums[i]] = i
	}
	return indices
}
