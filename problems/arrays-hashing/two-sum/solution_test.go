package two_sum

import "testing"

func TestSolutionFunction(t *testing.T) {
	tests := []struct {
		name     string
		nums 	 []int
		target	 int
		expected []int
	}{
		{
			name:     "Example 1",
			nums:    []int{2, 7, 11, 15},
			target:   9,
			expected: []int{0, 1},
		},
		{
			name:     "Example 2",
			nums:    []int{3, 2, 4},
			target:   6,
			expected: []int{1, 2},
		},
		{
			name:     "Edge case: empty input",
			nums:    []int{},
			target:   0,
			expected: []int{},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// TODO: Call solution function and compare result
			// result := SolutionFunction(tt.input)
			// if result != tt.expected {
			//     t.Errorf("got %v, want %v", result, tt.expected)
			// }
		})
	}
}
