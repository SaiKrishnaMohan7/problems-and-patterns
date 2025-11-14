package group_anagrams

import (
	"reflect"
	"sort"
	"testing"
)

// sortGroups sorts groups for comparison (order of groups doesn't matter)
func sortGroups(groups [][]string) [][]string {
	// Sort strings within each group
	for i := range groups {
		sort.Strings(groups[i])
	}

	// Sort groups by their first element
	sort.Slice(groups, func(i, j int) bool {
		if len(groups[i]) == 0 || len(groups[j]) == 0 {
			return len(groups[i]) < len(groups[j])
		}
		return groups[i][0] < groups[j][0]
	})

	return groups
}

func TestGroupAnagrams(t *testing.T) {
	tests := []struct {
		name     string
		input    []string
		expected [][]string
	}{
		{
			name:  "Example 1: Basic case with multiple anagram groups",
			input: []string{"eat", "tea", "tan", "ate", "nat", "bat"},
			expected: [][]string{
				{"bat"},
				{"nat", "tan"},
				{"ate", "eat", "tea"},
			},
		},
		{
			name:     "Example 2: Single empty string",
			input:    []string{""},
			expected: [][]string{{""}},
		},
		{
			name:     "Example 3: Single character string",
			input:    []string{"a"},
			expected: [][]string{{"a"}},
		},
		{
			name:     "Edge case: empty array",
			input:    []string{},
			expected: [][]string{},
		},
		{
			name:  "Edge case: no anagrams (all unique words)",
			input: []string{"abc", "def", "ghi"},
			expected: [][]string{
				{"abc"},
				{"def"},
				{"ghi"},
			},
		},
		{
			name:  "Edge case: all strings are anagrams",
			input: []string{"abc", "bca", "cab", "acb"},
			expected: [][]string{
				{"abc", "bca", "cab", "acb"},
			},
		},
		{
			name:  "Complex case: multiple groups with different lengths",
			input: []string{"listen", "silent", "hello", "world", "enlist"},
			expected: [][]string{
				{"hello"},
				{"listen", "silent", "enlist"},
				{"world"},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := GroupAnagrams(tt.input)

			// Sort both result and expected for comparison
			sortedResult := sortGroups(result)
			sortedExpected := sortGroups(tt.expected)

			if !reflect.DeepEqual(sortedResult, sortedExpected) {
				t.Errorf("GroupAnagrams() = %v, want %v", sortedResult, sortedExpected)
			}
		})
	}
}
