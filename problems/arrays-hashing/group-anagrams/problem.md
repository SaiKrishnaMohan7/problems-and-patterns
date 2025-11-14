# Group Anagrams

## Problem Description

Given an array of strings `strs`, group the anagrams together. You can return the answer in any order.

An **Anagram** is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

## Examples

### Example 1

**Input:**
```
strs = ["eat","tea","tan","ate","nat","bat"]
```

**Output:**
```
[["bat"],["nat","tan"],["ate","eat","tea"]]
```

**Explanation:**
- "eat", "tea", and "ate" are anagrams (all contain letters e, a, t)
- "tan" and "nat" are anagrams (all contain letters t, a, n)
- "bat" has no anagrams

### Example 2

**Input:**
```
strs = [""]
```

**Output:**
```
[[""]]
```

**Explanation:**
A single empty string forms one group.

### Example 3

**Input:**
```
strs = ["a"]
```

**Output:**
```
[["a"]]
```

**Explanation:**
A single character string with no anagrams forms its own group.

## Constraints

- `1 <= strs.length <= 10^4`
- `0 <= strs[i].length <= 100`
- `strs[i]` consists of lowercase English letters

## Follow-up

Can you solve this problem in O(m * n) time instead of O(m * n log n), where m is the number of strings and n is the average length of strings?

**Hint:** Instead of sorting each string, consider using a character frequency count as the key.

## Notes

- The order of groups in the output doesn't matter
- The order of strings within each group doesn't matter
- Two common approaches:
  1. **Sorting approach**: Sort each string and use the sorted version as a hash key (O(m * n log n))
  2. **Counting approach**: Use character frequency as a hash key (O(m * n))
