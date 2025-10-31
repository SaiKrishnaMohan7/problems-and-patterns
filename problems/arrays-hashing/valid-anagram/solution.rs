/// Valid Anagram
///
/// Pattern: arrays-hashing
/// Difficulty: Easy
///
/// Approach:
/// 1. Check if the two strings are of the same length
// 	1. if not return false
// 	2. else proceed
// 2. Create two maps or JS objects
// 3. Loop through the strings and fill out the map with char as key and its count as value
// 4. Get the keys of one of the objects
// 	1. loop through that and compare the value of the same key in the other string's map
// 		1. if the values are the same return true
// 		2. else even if one doesn't match, break out of the loop and return false

///
/// Time Complexity: O(?)
/// Space Complexity: O(?)

pub fn solution_function(str_a: &str, str_b: &str) -> bool {
    if str_a.len() != str_b.len() {
        return false;
    }

    let mut char_count_a = std::collections::HashMap::new();
    let mut char_count_b = std::collections::HashMap::new();

    for c in str_a.chars() {
        *char_count_a.entry(c).or_insert(0) += 1;
    }

    for c in str_b.chars() {
        *char_count_b.entry(c).or_insert(0) += 1;
    }

    char_count_a == char_count_b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let result = solution_function("anagram", "nagaram");
        assert_eq!(result, true);
    }

    #[test]
    fn test_example_2() {
        let result = solution_function("rat", "car");
        assert_eq!(result, false);
    }

    #[test]
    fn test_edge_case_empty_input() {
        let result = solution_function("", "");
        assert_eq!(result, true);
    }
}
