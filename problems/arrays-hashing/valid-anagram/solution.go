// Package valid_anagram implements a solution for Valid Anagram
//
// Pattern: arrays-hashing
// Difficulty: Easy
//
// Approach:
// 1. Check if the two strings are of the same length
// 	1. if not return false
// 	2. else proceed
// 2. Create two maps or JS objects
// 3. Loop through the strings and fill out the map with char as key and its count as value
// 4. Get the keys of one of the objects
// 	1. loop through that and compare the value of the same key in the other string's map
		// 1. if the values are the same return true
		// 2. else even if one doesn't match, break out of the loop and return false

//
// Time Complexity: O(n)
// Space Complexity: O(n)
package valid_anagram

// SolutionFunction TODO: Implement solution
func SolutionFunction(strA string, strB string) bool {
	if len(strA) != len(strB) {
		return false
	}

	charCountA := make(map[rune]int)
	charCountB := make(map[rune]int)

	for i := 0; i < len(strA); i++ {
		charCountA[rune(strA[i])]++
		charCountB[rune(strB[i])]++
	}

	for char, countA := range charCountA {
		countB, exists := charCountB[char]
		if !exists || countA != countB {
			return false
		}
	}

	return true
}
