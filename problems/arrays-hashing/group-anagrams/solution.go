// Package group_anagrams implements a solution for Group Anagrams
//
// Pattern: arrays-hashing
// Difficulty: Medium
//
// Approach:
// 1. Create a map to store grouped anagrams
// 2. Loop through all strings in the array
//    - Sort each string to use as a key (anagrams will have the same sorted form)
//    - If the sorted string is not in the map, initialize it with an empty slice
//    - Append the original string to the slice corresponding to its sorted key
// 3. Return all values from the map (the grouped anagrams)
//
// Time Complexity: O(m * n log n) where n is the length of the longest string, m is the number of strings
// Space Complexity: O(m * n) to store all strings in the map
package group_anagrams

import (
	"sort"
	"strings"
)

// GroupAnagrams groups strings that are anagrams of each other
func GroupAnagrams(strs []string) [][]string {
	anagramMap := make(map[string][]string)

	for _, str := range strs {
		// Sort the string to use as key
		sortedStr := sortString(str)

		// Append to the corresponding group
		anagramMap[sortedStr] = append(anagramMap[sortedStr], str)
	}

	// Collect all groups
	result := make([][]string, 0, len(anagramMap))
	for _, group := range anagramMap {
		result = append(result, group)
	}

	return result
}

// sortString sorts the characters in a string
func sortString(s string) string {
	chars := strings.Split(s, "")
	sort.Strings(chars)
	return strings.Join(chars, "")
}
