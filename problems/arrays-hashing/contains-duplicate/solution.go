// Package contains-duplicate implements a solution for Contains Duplicate
//
// Pattern: arrays-hashing
// Difficulty: Easy
//
// Approach:
// The tricky part of this question is the wording, it says something like, "return true if element repeats at least twice"
// Which would make you think `freqMap[item] >= 2` but if you look carefully, the condition is that an element has to occur more than once!
// Which, in english, can be expressed as "element repeats atleast twice!"
//
// Time Complexity: O(n)
// Space Complexity: O(n)
package contains_duplicate

func SolutionFunction(nums []int) bool {
	freqMap := make(map[int]int)
		for _, item := range nums {
    	if freqMap[item] >= 1 {
      	return true
    	} else {
      	freqMap[item] = 1
    	}
  }
  return false
}
