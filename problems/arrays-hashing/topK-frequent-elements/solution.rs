/// Top K frequent elements
///
/// Pattern: arrays-hashing
/// Difficulty: Medium
///
/// IDEA:
///  Use array indices as count and store the numbers from nums that match the count under
///  those indices in an array. Bucket sort concept used.
/// ALGO IMPL:
///  Create map to count numbers; array elem as key and count as value
///  Create the buckets with length 1 greater than input arr length, as we are using indices
///    to keep track of count and if nums is length 6, it is possible that an element may repeat 6 times
///    and if we created bucket of the same length as input array (0 to 5 indices) then we won't be able to
///    count the element repeating 6 times!
///  Populate the map by counting the numbers; { num: count }
///  Since we want K MOST FREQUENT, we start looping from behind
///
/// Time Complexity: O(n)
/// Space Complexity: O(n)

use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut freq_map: HashMap<i32, usize>;
    let mut buckets: Vec<Vec<i32>>;
    let mut result: Vec<i32>;

    // Build frequency map
    freq_map = HashMap::new();
    for &num in &nums {
        if freq_map.contains_key(&num) {
            let count = freq_map.get(&num).unwrap();
            freq_map.insert(num, count + 1);
        } else {
            freq_map.insert(num, 1);
        }
    }

    // Bucket sort: index = frequency, value = vector of numbers with that frequency
    buckets = vec![Vec::new(); nums.len() + 1];

    for (&num, &freq) in &freq_map {
        buckets[freq].push(num);
    }

    // Collect top k frequent elements
    result = Vec::new();
    for i in (0..buckets.len()).rev() {
        if result.len() >= k as usize {
            break;
        }
        if !buckets[i].is_empty() {
            result.extend(&buckets[i]);
        }
    }

    // Trim to exactly k elements
    result.truncate(k as usize);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to sort arrays for comparison
    fn sort_vec(mut vec: Vec<i32>) -> Vec<i32> {
        vec.sort();
        vec
    }

    #[test]
    fn test_example_1_multiple_elements_with_different_frequencies() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        let result = top_k_frequent(nums, k);

        assert_eq!(sort_vec(result), sort_vec(vec![1, 2]));
    }

    #[test]
    fn test_example_2_single_unique_element() {
        let nums = vec![1];
        let k = 1;
        let result = top_k_frequent(nums, k);

        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_example_3_all_elements_have_same_frequency() {
        let nums = vec![1, 2, 3];
        let k = 3;
        let result = top_k_frequent(nums, k);

        assert_eq!(result.len(), 3);
        assert_eq!(sort_vec(result), vec![1, 2, 3]);
    }

    #[test]
    fn test_edge_case_k_equals_number_of_unique_elements() {
        let nums = vec![4, 4, 5, 5, 6, 6];
        let k = 3;
        let result = top_k_frequent(nums, k);

        assert_eq!(result.len(), 3);
        assert_eq!(sort_vec(result), vec![4, 5, 6]);
    }

    #[test]
    fn test_edge_case_k_equals_1() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 1;
        let result = top_k_frequent(nums, k);

        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_complex_case_negative_numbers() {
        let nums = vec![-1, -1, -1, -2, -2, 3];
        let k = 2;
        let result = top_k_frequent(nums, k);

        assert_eq!(sort_vec(result), sort_vec(vec![-1, -2]));
    }

    #[test]
    fn test_complex_case_large_frequency_difference() {
        let nums = vec![1, 1, 1, 1, 1, 2, 3, 3];
        let k = 2;
        let result = top_k_frequent(nums, k);

        assert_eq!(sort_vec(result), sort_vec(vec![1, 3]));
    }
}
