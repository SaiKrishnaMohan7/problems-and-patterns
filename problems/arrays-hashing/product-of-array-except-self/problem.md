# Product of Array Except Self

## Problem Description

Given an integer array `nums`, return an array `answer` such that `answer[i]` is equal to the product of all the elements of `nums` except `nums[i]`.

The product of any prefix or suffix of `nums` is **guaranteed** to fit in a **32-bit** integer.

You must write an algorithm that runs in **O(n)** time and **without using the division operation**.

## Examples

### Example 1

**Input:**
```
nums = [1,2,3,4]
```

**Output:**
```
[24,12,8,6]
```

**Explanation:**
- answer[0] = 2 * 3 * 4 = 24
- answer[1] = 1 * 3 * 4 = 12
- answer[2] = 1 * 2 * 4 = 8
- answer[3] = 1 * 2 * 3 = 6

### Example 2

**Input:**
```
nums = [-1,1,0,-3,3]
```

**Output:**
```
[0,0,9,0,0]
```

**Explanation:**
- answer[0] = 1 * 0 * (-3) * 3 = 0
- answer[1] = (-1) * 0 * (-3) * 3 = 0
- answer[2] = (-1) * 1 * (-3) * 3 = 9
- answer[3] = (-1) * 1 * 0 * 3 = 0
- answer[4] = (-1) * 1 * 0 * (-3) = 0

## Constraints

- `2 <= nums.length <= 10^5`
- `-30 <= nums[i] <= 30`
- The product of any prefix or suffix of `nums` is **guaranteed** to fit in a **32-bit** integer

## Follow-up

Can you solve the problem in **O(1)** extra space complexity? (The output array does not count as extra space for space complexity analysis.)

**Hint:** Use the output array itself to store prefix products, then multiply with postfix products in a second pass!

## Notes

- **Why no division?** The naive approach would be to calculate the total product and divide by each element, but:
  - Division operation is restricted by the problem
  - Handling zeros becomes tricky (would need special cases)
  - Division can cause precision issues with floating point

- **Optimal approach: Prefix and Postfix products**
  1. **First pass (left to right)**: Calculate prefix products
     - For index i, store the product of all elements to the left
  2. **Second pass (right to left)**: Calculate postfix products
     - For index i, multiply with the product of all elements to the right

- **Example walkthrough for [1,2,3,4]:**
  ```
  Initial:  result = [1, 1, 1, 1]

  Prefix pass (left to right):
  - i=0: result[0] = 1 (no elements to left)
  - i=1: result[1] = 1 (only 1 to left)
  - i=2: result[2] = 2 (1*2 to left)
  - i=3: result[3] = 6 (1*2*3 to left)
  After prefix: [1, 1, 2, 6]

  Postfix pass (right to left):
  - i=3: result[3] = 6 * 1 = 6 (no elements to right)
  - i=2: result[2] = 2 * 4 = 8 (only 4 to right)
  - i=1: result[1] = 1 * 12 = 12 (3*4 to right)
  - i=0: result[0] = 1 * 24 = 24 (2*3*4 to right)
  Final result: [24, 12, 8, 6]
  ```

- **Time Complexity:** O(n) - two passes through the array
- **Space Complexity:** O(1) - only using the output array (which doesn't count)
