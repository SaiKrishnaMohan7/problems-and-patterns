/// Longest Consecutive Sequence
///
/// Pattern: arrays-hashing
/// Difficulty: Medium
///
/// Approach:
/// Use a hash set to store all numbers for O(1) lookup.
/// For each number, check if it's the start of a sequence (no left neighbor).
/// If it is, count how many consecutive numbers exist to the right.
/// Track the maximum sequence length found.
///
/// Time Complexity: O(n) - Each number is visited at most twice
/// Space Complexity: O(n) - Hash set to store all numbers

use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    // Build a set of all numbers for O(1) lookup
    let num_set: HashSet<i32> = nums.iter().copied().collect();

    let mut longest = 0;

    for &num in &nums {
        let left_neighbor = num - 1;

        // If left_neighbor is not in set, num is the start of a sequence
        if !num_set.contains(&left_neighbor) {
            let mut length = 0;
            // Count how many consecutive right neighbors exist
            while num_set.contains(&(num + length)) {
                length += 1;
            }
            longest = longest.max(length);
        }
    }

    longest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // Example 1: unsorted array with consecutive sequence
        assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        // Explanation: The longest consecutive sequence is [1, 2, 3, 4]
    }

    #[test]
    fn test_example_2() {
        // Example 2: array with duplicates
        assert_eq!(longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
        // Explanation: The longest consecutive sequence is [0, 1, 2, 3, 4, 5, 6, 7, 8]
    }

    #[test]
    fn test_edge_case_empty_array() {
        assert_eq!(longest_consecutive(vec![]), 0);
    }

    #[test]
    fn test_edge_case_single_element() {
        assert_eq!(longest_consecutive(vec![1]), 1);
    }

    #[test]
    fn test_edge_case_no_consecutive_numbers() {
        assert_eq!(longest_consecutive(vec![10, 30, 50]), 1);
    }

    #[test]
    fn test_edge_case_all_consecutive() {
        assert_eq!(longest_consecutive(vec![1, 2, 3, 4, 5]), 5);
    }

    #[test]
    fn test_edge_case_negative_numbers() {
        // Edge case: negative numbers
        assert_eq!(longest_consecutive(vec![-1, -2, 0, 1, 2]), 5);
        // Explanation: The sequence is [-2, -1, 0, 1, 2]
    }
}
