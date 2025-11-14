# Top K Frequent Elements

## Problem Description

Given an integer array `nums` and an integer `k`, return the `k` most frequent elements. You may return the answer in **any order**.

## Examples

### Example 1

**Input:**
```
nums = [1,1,1,2,2,3], k = 2
```

**Output:**
```
[1,2]
```

**Explanation:**
The element 1 appears 3 times, 2 appears 2 times, and 3 appears 1 time. The two most frequent elements are 1 and 2.

### Example 2

**Input:**
```
nums = [1], k = 1
```

**Output:**
```
[1]
```

**Explanation:**
There is only one element, so it is the most frequent.

### Example 3

**Input:**
```
nums = [1,1,1,2,2,3,3,3], k = 2
```

**Output:**
```
[1,3]
```

**Explanation:**
Both 1 and 3 appear 3 times each, making them the two most frequent elements.

## Constraints

- `1 <= nums.length <= 10^5`
- `-10^4 <= nums[i] <= 10^4`
- `k` is in the range `[1, the number of unique elements in the array]`
- It is **guaranteed** that the answer is **unique**

## Follow-up

Your algorithm's time complexity must be better than O(n log n), where n is the array's size.

**Hint:** The optimal solution uses bucket sort and achieves O(n) time complexity!

## Notes

- The order of elements in the output doesn't matter
- Three common approaches:
  1. **Sorting approach**: Sort by frequency (O(n log n))
  2. **Heap approach**: Use a min-heap of size k (O(n log k))
  3. **Bucket sort approach**: Use frequency as bucket index (O(n)) - optimal!
- The bucket sort approach is the most efficient because:
  - Maximum frequency is bounded by array length
  - We can use frequency directly as an array index
  - Collecting top k is just iterating from highest frequency downward
