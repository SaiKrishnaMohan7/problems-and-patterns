package product_of_array_except_self

import (
	"reflect"
	"testing"
)

func TestProductExceptSelf(t *testing.T) {
	tests := []struct {
		name     string
		nums     []int
		expected []int
	}{
		{
			name:     "Example 1: Basic case",
			nums:     []int{1, 2, 3, 4},
			expected: []int{24, 12, 8, 6},
		},
		{
			name:     "Example 2: Array with different values",
			nums:     []int{-1, 1, 0, -3, 3},
			expected: []int{0, 0, 9, 0, 0},
		},
		{
			name:     "Example 3: Two elements",
			nums:     []int{1, 2},
			expected: []int{2, 1},
		},
		{
			name:     "Edge case: Array with ones",
			nums:     []int{1, 1, 1, 1},
			expected: []int{1, 1, 1, 1},
		},
		{
			name:     "Edge case: Array with zero at start",
			nums:     []int{0, 2, 3},
			expected: []int{6, 0, 0},
		},
		{
			name:     "Edge case: Array with zero at end",
			nums:     []int{2, 3, 0},
			expected: []int{0, 0, 6},
		},
		{
			name:     "Edge case: Array with zero in middle",
			nums:     []int{2, 0, 3},
			expected: []int{0, 6, 0},
		},
		{
			name:     "Complex case: Negative numbers",
			nums:     []int{-2, -3, -4},
			expected: []int{12, 8, 6},
		},
		{
			name:     "Complex case: Mixed positive and negative",
			nums:     []int{2, -3, 4, -5},
			expected: []int{60, -40, 30, -24},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := ProductExceptSelf(tt.nums)

			if !reflect.DeepEqual(result, tt.expected) {
				t.Errorf("ProductExceptSelf() = %v, want %v", result, tt.expected)
			}
		})
	}
}
