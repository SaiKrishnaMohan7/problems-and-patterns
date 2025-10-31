# Contains Duplicate

## Problem Description

Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.

## Examples

### Example 1

Output: true

**Input:**
```
nums = [1,2,3,1]
```

**Output:**
```
true
```

**Explanation:**
The element 1 occurs at the indices 0 and 3.

### Example 2

**Input:**
```
nums = [1,1,1,3,3,4,3,2,4,2]
```

**Output:**
```
true
```

**Explanation:**
All elements are distinct.

## Constraints
 1 <= nums.length <= 105
-109 <= nums[i] <= 109
-

## Follow-up


## Notes

- **Time: O(n) Space: O(n)**
- The tricky part of this question is the wording, it says something like, "return true if element repeats at least twice"
	- Which would make you think `freqMap[item] >= 2` but if you look carefully, the condition is that an element has to occur more than once! Which, in english, can be expressed as "element repeats atleast twice!"


