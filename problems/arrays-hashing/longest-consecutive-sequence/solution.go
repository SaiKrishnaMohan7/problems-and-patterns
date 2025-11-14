// Package longest_consecutive_sequence implements a solution for Longest Consecutive Sequence
//
// Pattern: arrays-hashing
// Difficulty: Medium
//
// Approach:
// Use a hash set to store all numbers for O(1) lookup.
// For each number, check if it's the start of a sequence (no left neighbor).
// If it is, count how many consecutive numbers exist to the right.
// Track the maximum sequence length found.
//
// Time Complexity: O(n) - Each number is visited at most twice
// Space Complexity: O(n) - Hash set to store all numbers
package longest_consecutive_sequence

func LongestConsecutive(nums []int) int {
	if len(nums) == 0 {
		return 0
	}

	// Build a set of all numbers for O(1) lookup
	numSet := make(map[int]bool)
	for _, num := range nums {
		numSet[num] = true
	}

	longest := 0

	for _, num := range nums {
		leftNeighbor := num - 1

		// If leftNeighbor is not in set, num is the start of a sequence
		if !numSet[leftNeighbor] {
			length := 0
			// Count how many consecutive right neighbors exist
			for numSet[num+length] {
				length++
			}
			if length > longest {
				longest = length
			}
		}
	}

	return longest
}
