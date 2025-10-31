package valid_anagram

import "testing"

func TestSolutionFunction(t *testing.T) {
	tests := []struct {
		name     string
		strA     string
		strB     string
		expected bool
	}{
		{
			name:     "Example 1: anagram",
			strA:     "anagram",
			strB:     "nagaram",
			expected: true,
		},
		{
			name:     "Example 2: not anagram",
			strA:     "rat",
			strB:     "car",
			expected: false,
		},
		{
			name:     "Edge case: empty strings",
			strA:     "",
			strB:     "",
			expected: true,
		},
		{
			name:     "Edge case: different lengths",
			strA:     "a",
			strB:     "ab",
			expected: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := SolutionFunction(tt.strA, tt.strB)
			if result != tt.expected {
				t.Errorf("got %v, want %v", result, tt.expected)
			}
		})
	}
}