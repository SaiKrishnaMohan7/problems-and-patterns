# Encode and Decode Strings

## Problem Description

Design an algorithm to encode a list of strings to a single string. The encoded string is then decoded back to the original list of strings.

Please implement `encode` and `decode`.

## Examples

### Example 1

**Input:**
```
["neet","code","love","you"]
```

**Output:**
```
["neet","code","love","you"]
```

**Explanation:**
The strings are encoded into a single string, then decoded back to the original list. The encode/decode should preserve all strings exactly.

### Example 2

**Input:**
```
["we","say",":","yes"]
```

**Output:**
```
["we","say",":","yes"]
```

**Explanation:**
Even strings with special characters like ":" should be preserved exactly during encoding and decoding.

### Example 3

**Input:**
```
["hello","","world"]
```

**Output:**
```
["hello","","world"]
```

**Explanation:**
Empty strings should also be preserved in the encoding/decoding process.

## Constraints

- `0 <= strs.length <= 100`
- `0 <= strs[i].length <= 200`
- `strs[i]` contains only UTF-8 characters
- Strings may contain **any** characters including delimiters

## Follow-up

How would you handle the case where strings can contain any character, including your delimiter? The naive approach of using a simple delimiter (like comma or pipe) won't work if that delimiter appears in the string itself.

**Hint:** Length-prefix encoding is a robust solution!

## Notes

- This is a classic system design problem
- The key challenge is handling arbitrary characters in the strings
- Common approaches:
  1. **Simple delimiter** (fails if delimiter is in strings): `"hello|world"`
  2. **Escaped delimiter** (complex parsing): `"hello\|world"`
  3. **Length-prefix encoding** (optimal): `"5%hello5%world"`
- Length-prefix encoding works because:
  - We store the length before each string
  - We know exactly how many characters to read
  - The delimiter can appear in the string content without issues
- Format: `{length}{delimiter}{string}{length}{delimiter}{string}...`
- This pattern is used in real protocols like HTTP chunked encoding
