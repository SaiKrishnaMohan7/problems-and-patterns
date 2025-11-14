// Package encode_and_decode_strings implements a solution for Encode and Decode strings
//
// Pattern: arrays-hashing
// Difficulty: Medium
//
// Approach:
// Use length-prefix encoding: For each string, store its length followed by a delimiter,
// then the string itself. This way even if the delimiter appears in the string content,
// we know exactly how many characters to read based on the length prefix.
//
// Format: {length}{delimiter}{string}{length}{delimiter}{string}...
// Example: ["hello", "world"] -> "5%hello5%world"
//
// Time Complexity: O(n) where n is total length of all strings
// Space Complexity: O(n) for the encoded string
package encode_and_decode_strings

import (
	"strconv"
	"strings"
)

// Encode encodes a list of strings to a single string
func Encode(strs []string) string {
	var encodedStr strings.Builder
	delimiter := "%"

	for i := range strs {
		str := strs[i]
		strLength := len(str)
		encodedStr.WriteString(strconv.Itoa(strLength))
		encodedStr.WriteString(delimiter)
		encodedStr.WriteString(str)
	}

	return encodedStr.String()
}

// Decode decodes a single string to a list of strings
func Decode(encodedStr string) []string {
	var result []string
	var i int
	var delimiter string

	result = []string{}
	i = 0
	delimiter = "%"

	for i < len(encodedStr) {
		j := i

		// Find the delimiter to get the length
		for encodedStr[j] != delimiter[0] {
			j++
		}

		strLength, _ := strconv.Atoi(encodedStr[i:j])
		beginningIdxOfStr := j + 1
		endingIdxOfStr := beginningIdxOfStr + strLength
		str := encodedStr[beginningIdxOfStr:endingIdxOfStr]
		result = append(result, str)

		i = endingIdxOfStr
	}

	return result
}
