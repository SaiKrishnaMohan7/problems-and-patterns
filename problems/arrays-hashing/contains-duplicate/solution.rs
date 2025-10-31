/// Contains Duplicate
///
/// Pattern: arrays-hashing
/// Difficulty: Easy
///
/// Approach:
/// The tricky part of this question is the wording, it says something like, "return true if element repeats at least twice"
/// Which would make you think `freqMap[item] >= 2` but if you look carefully, the condition is that an element has to occur more than once!
/// Which, in english, can be expressed as "element repeats atleast twice!"
///
/// Time Complexity: O(n)
/// Space Complexity: O(n)

pub fn solution_function(nums: Vec<i32>) -> bool {
    let mut freq_map = std::collections::HashMap::new();
    for &item in &nums {
        if freq_map.contains_key(&item) {
            return true;
        } else {
            freq_map.insert(item, 1);
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = vec![1, 2, 3, 1];
        assert_eq!(true, solution_function(input));
    }

    #[test]
    fn test_example_2() {
        let input = vec![1, 2, 3, 4];
        assert_eq!(false, solution_function(input));
    }

    #[test]
    fn test_edge_case_empty_input() {
        let input = Vec::new();
        assert_eq!(false, solution_function(input));
    }
}
