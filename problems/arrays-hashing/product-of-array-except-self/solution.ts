/**
 * Product of Array Except Self
 *
 * Pattern: arrays-hashing
 * Difficulty: Medium
 *
 * Approach:
 * Use prefix and postfix products without division
 * 1. First pass: Calculate prefix products (product of all elements to the left)
 * 2. Second pass: Calculate postfix products (product of all elements to the right) and multiply with prefix
 *
 * Example: [1,2,3,4]
 * - Prefix:  [1, 1, 2, 6]  (1, 1*1, 1*1*2, 1*1*2*3)
 * - Postfix: [24, 12, 4, 1] (2*3*4, 3*4, 4, 1)
 * - Result:  [24, 12, 8, 6] (prefix[i] * postfix[i])
 *
 * Time Complexity: O(n)
 * Space Complexity: O(1) - not counting the output array
 */

export function productExceptSelf(nums: number[]): number[] {
  let product = 1;
  let result = [];

  // Prefix pass: For each position, store product of all elements to the LEFT
  // Example: [1,2,3,4]
  // i=0: result[0] = 1 (nothing to left), then product becomes 1
  // i=1: result[1] = 1 (only 1 to left), then product becomes 1*2 = 2
  // i=2: result[2] = 2 (1*2 to left), then product becomes 2*3 = 6
  // i=3: result[3] = 6 (1*2*3 to left), then product becomes 6*4 = 24
  // After this: result = [1, 1, 2, 6]
  for (let i = 0; i < nums.length; i++) {
    result[i] = product;
    product = product * nums[i];
  }

  // Reset to 1 because we're starting a NEW calculation from the right
  product = 1;

  // Postfix pass: For each position, multiply by product of all elements to the RIGHT
  // Example continuing: result = [1, 1, 2, 6]
  // j=3: result[3] = 6 * 1 = 6 (nothing to right), then product becomes 4
  // j=2: result[2] = 2 * 4 = 8 (only 4 to right), then product becomes 4*3 = 12
  // j=1: result[1] = 1 * 12 = 12 (3*4 to right), then product becomes 12*2 = 24
  // j=0: result[0] = 1 * 24 = 24 (2*3*4 to right), then product becomes 24*1 = 24
  // Final: result = [24, 12, 8, 6]
  for (let j = nums.length - 1; j >= 0; j--) {
    result[j] = result[j] * product;
    product = product * nums[j];
  }

  return result;
}
