package topK_frequent_elements

import (
	"reflect"
	"sort"
	"testing"
)

// sortArray sorts a slice for comparison
func sortArray(arr []int) []int {
	sorted := make([]int, len(arr))
	copy(sorted, arr)
	sort.Ints(sorted)
	return sorted
}

func TestTopKFrequent(t *testing.T) {
	tests := []struct {
		name     string
		nums     []int
		k        int
		expected []int
	}{
		{
			name:     "Example 1: Multiple elements with different frequencies",
			nums:     []int{1, 1, 1, 2, 2, 3},
			k:        2,
			expected: []int{1, 2},
		},
		{
			name:     "Example 2: Single unique element",
			nums:     []int{1},
			k:        1,
			expected: []int{1},
		},
		{
			name:     "Example 3: All elements have same frequency",
			nums:     []int{1, 2, 3},
			k:        3,
			expected: []int{1, 2, 3},
		},
		{
			name:     "Edge case: k equals number of unique elements",
			nums:     []int{4, 4, 5, 5, 6, 6},
			k:        3,
			expected: []int{4, 5, 6},
		},
		{
			name:     "Edge case: k = 1",
			nums:     []int{1, 1, 1, 2, 2, 3},
			k:        1,
			expected: []int{1},
		},
		{
			name:     "Complex case: Negative numbers",
			nums:     []int{-1, -1, -1, -2, -2, 3},
			k:        2,
			expected: []int{-1, -2},
		},
		{
			name:     "Complex case: Large frequency difference",
			nums:     []int{1, 1, 1, 1, 1, 2, 3, 3},
			k:        2,
			expected: []int{1, 3},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := TopKFrequent(tt.nums, tt.k)

			// Sort both result and expected for comparison
			sortedResult := sortArray(result)
			sortedExpected := sortArray(tt.expected)

			if !reflect.DeepEqual(sortedResult, sortedExpected) {
				t.Errorf("TopKFrequent() = %v, want %v", sortedResult, sortedExpected)
			}
		})
	}
}
