package contains_duplicate

import "testing"

func TestSolutionFunction(t *testing.T) {
	tests := []struct {
		name     string
		input    []int
		expected bool
	}{
		{
			name:     "Example 1",
			input:    []int{1, 2, 3, 1},
			expected: true,
		},
		{
			name:     "Example 2",
			input:    []int{1, 2, 3, 4},
			expected: false,
		},
		{
			name:     "Edge case: empty input",
			input:    []int{},
			expected: false,
		},
		{
			name:     "Example 2",
			input:    []int{1, 2, 3, 1},
			expected: true,
		},
		{
			name:     "Edge case: empty input",
			input:    nil,
			expected: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := SolutionFunction(tt.input)
			if result != tt.expected {
				t.Errorf("got %v, want %v", result, tt.expected)
			}
		})
	}
}
