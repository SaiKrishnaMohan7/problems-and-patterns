// Package topK_frequent_elements implements a solution for Top K frequent elements
//
// Pattern: arrays-hashing
// Difficulty: Medium
//
// IDEA:
//  Use array indices as count and store the numbers from nums that match the count under
//  those indices in an array. Bucket sort concept used.
// ALGO IMPL:
//  Create map to count numbers; array elem as key and count as value
//  Create the buckets with length 1 greater than input arr length, as we are using indices
//    to keep track of count and if nums is length 6, it is possible that an element may repeat 6 times
//    and if we created bucket of the same length as input array (0 to 5 indices) then we won't be able to
//    count the element repeating 6 times!
//  Populate the map by counting the numbers; { num: count }
//  Since we want K MOST FREQUENT, we start looping from behind
//
// Time Complexity: O(n)
// Space Complexity: O(n)
package topK_frequent_elements

// TopKFrequent finds the k most frequent elements
func TopKFrequent(nums []int, k int) []int {
	var freqMap map[int]int
	var buckets [][]int
	var result []int

	// Build frequency map
	freqMap = make(map[int]int)
	for _, num := range nums {
		count, exists := freqMap[num]
		if exists {
			freqMap[num] = count + 1
		} else {
			freqMap[num] = 1
		}
	}

	// Bucket sort: index = frequency, value = slice of numbers with that frequency
	buckets = make([][]int, len(nums)+1)
	for i := range buckets {
		buckets[i] = []int{}
	}

	for num, freq := range freqMap {
		buckets[freq] = append(buckets[freq], num)
	}

	// Collect top k frequent elements
	result = []int{}
	for i := len(buckets) - 1; i >= 0 && len(result) < k; i-- {
		if len(buckets[i]) > 0 {
			result = append(result, buckets[i]...)
		}
	}

	// Trim to exactly k elements
	if len(result) > k {
		result = result[:k]
	}

	return result
}
