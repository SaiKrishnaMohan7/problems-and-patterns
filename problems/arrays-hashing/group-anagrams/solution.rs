/// Group Anagrams
///
/// Pattern: arrays-hashing
/// Difficulty: Medium
///
/// Approach:
/// 1. Create a HashMap to store grouped anagrams
/// 2. Loop through all strings in the vector
///    - Sort each string to use as a key (anagrams will have the same sorted form)
///    - If the sorted string is not in the map, initialize it with an empty vector
///    - Push the original string to the vector corresponding to its sorted key
/// 3. Return all values from the map (the grouped anagrams)
///
/// Time Complexity: O(m * n log n) where n is the length of the longest string, m is the number of strings
/// Space Complexity: O(m * n) to store all strings in the map

use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut anagram_map: HashMap<String, Vec<String>> = HashMap::new();

    for s in strs {
        // Sort the string to use as key
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort_unstable();
        let sorted_str: String = chars.into_iter().collect();

        // Add to the corresponding group
        anagram_map
            .entry(sorted_str)
            .or_insert_with(Vec::new)
            .push(s);
    }

    // Collect all groups into a result vector
    anagram_map.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to sort groups for comparison (order of groups doesn't matter)
    fn sort_groups(mut groups: Vec<Vec<String>>) -> Vec<Vec<String>> {
        // Sort strings within each group
        for group in &mut groups {
            group.sort();
        }

        // Sort groups by their first element
        groups.sort_by(|a, b| {
            if a.is_empty() || b.is_empty() {
                return a.len().cmp(&b.len());
            }
            a[0].cmp(&b[0])
        });

        groups
    }

    #[test]
    fn test_example_1_basic_case_with_multiple_anagram_groups() {
        let input = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let result = group_anagrams(input);
        let expected = vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
        ];

        assert_eq!(sort_groups(result), sort_groups(expected));
    }

    #[test]
    fn test_example_2_single_empty_string() {
        let input = vec!["".to_string()];
        let result = group_anagrams(input);
        let expected = vec![vec!["".to_string()]];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_example_3_single_character_string() {
        let input = vec!["a".to_string()];
        let result = group_anagrams(input);
        let expected = vec![vec!["a".to_string()]];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_edge_case_empty_array() {
        let input: Vec<String> = vec![];
        let result = group_anagrams(input);
        let expected: Vec<Vec<String>> = vec![];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_edge_case_no_anagrams_all_unique() {
        let input = vec!["abc".to_string(), "def".to_string(), "ghi".to_string()];
        let result = group_anagrams(input);
        let expected = vec![
            vec!["abc".to_string()],
            vec!["def".to_string()],
            vec!["ghi".to_string()],
        ];

        assert_eq!(sort_groups(result), sort_groups(expected));
    }

    #[test]
    fn test_edge_case_all_strings_are_anagrams() {
        let input = vec![
            "abc".to_string(),
            "bca".to_string(),
            "cab".to_string(),
            "acb".to_string(),
        ];
        let result = group_anagrams(input);

        assert_eq!(result.len(), 1);
        let mut sorted_result = result[0].clone();
        sorted_result.sort();
        assert_eq!(
            sorted_result,
            vec![
                "abc".to_string(),
                "acb".to_string(),
                "bca".to_string(),
                "cab".to_string()
            ]
        );
    }

    #[test]
    fn test_complex_case_multiple_groups_with_different_lengths() {
        let input = vec![
            "listen".to_string(),
            "silent".to_string(),
            "hello".to_string(),
            "world".to_string(),
            "enlist".to_string(),
        ];
        let result = group_anagrams(input);
        let expected = vec![
            vec!["hello".to_string()],
            vec![
                "listen".to_string(),
                "silent".to_string(),
                "enlist".to_string(),
            ],
            vec!["world".to_string()],
        ];

        assert_eq!(sort_groups(result), sort_groups(expected));
    }
}
