package encode_and_decode_strings

import (
	"reflect"
	"strings"
	"testing"
)

func TestEncodeDecodeStrings(t *testing.T) {
	tests := []struct {
		name  string
		input []string
	}{
		{
			name:  "Example 1: Basic strings",
			input: []string{"hello", "world"},
		},
		{
			name:  "Example 2: Multiple strings",
			input: []string{"neet", "code", "love", "you"},
		},
		{
			name:  "Example 3: Strings with delimiter character",
			input: []string{"we", "say", ":", "yes", "%", "!"},
		},
		{
			name:  "Edge case: Empty array",
			input: []string{},
		},
		{
			name:  "Edge case: Array with empty strings",
			input: []string{"", "", ""},
		},
		{
			name:  "Edge case: Single empty string",
			input: []string{""},
		},
		{
			name:  "Complex case: Mixed empty and non-empty strings",
			input: []string{"hello", "", "world", "", "test"},
		},
		{
			name:  "Complex case: Strings with special characters",
			input: []string{"#@!", "123", "a b c", "new\nline"},
		},
		{
			name:  "Complex case: Long strings",
			input: []string{strings.Repeat("a", 1000), strings.Repeat("b", 500)},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			encoded := Encode(tt.input)
			decoded := Decode(encoded)

			if !reflect.DeepEqual(decoded, tt.input) {
				t.Errorf("Encode/Decode mismatch: got %v, want %v", decoded, tt.input)
			}
		})
	}
}

func TestEncodeEmpty(t *testing.T) {
	input := []string{}
	encoded := Encode(input)

	if encoded != "" {
		t.Errorf("Encode empty array: got %q, want empty string", encoded)
	}
}
