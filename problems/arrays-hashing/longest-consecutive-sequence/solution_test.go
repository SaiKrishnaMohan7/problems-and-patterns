package longest_consecutive_sequence

import "testing"

func TestLongestConsecutive(t *testing.T) {
	tests := []struct {
		name     string
		input    []int
		expected int
	}{
		{
			name:     "Example 1: unsorted array with consecutive sequence",
			input:    []int{100, 4, 200, 1, 3, 2},
			expected: 4,
			// Explanation: The longest consecutive sequence is [1, 2, 3, 4]
		},
		{
			name:     "Example 2: array with duplicates",
			input:    []int{0, 3, 7, 2, 5, 8, 4, 6, 0, 1},
			expected: 9,
			// Explanation: The longest consecutive sequence is [0, 1, 2, 3, 4, 5, 6, 7, 8]
		},
		{
			name:     "Edge case: empty array",
			input:    []int{},
			expected: 0,
		},
		{
			name:     "Edge case: single element",
			input:    []int{1},
			expected: 1,
		},
		{
			name:     "Edge case: no consecutive numbers",
			input:    []int{10, 30, 50},
			expected: 1,
		},
		{
			name:     "Edge case: all consecutive",
			input:    []int{1, 2, 3, 4, 5},
			expected: 5,
		},
		{
			name:     "Edge case: negative numbers",
			input:    []int{-1, -2, 0, 1, 2},
			expected: 5,
			// Explanation: The sequence is [-2, -1, 0, 1, 2]
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := LongestConsecutive(tt.input)
			if result != tt.expected {
				t.Errorf("got %v, want %v", result, tt.expected)
			}
		})
	}
}
