/// Encode and Decode Strings
///
/// Pattern: arrays-hashing
/// Difficulty: Medium
///
/// Approach:
/// Use length-prefix encoding: For each string, store its length followed by a delimiter,
/// then the string itself. This way even if the delimiter appears in the string content,
/// we know exactly how many characters to read based on the length prefix.
///
/// Format: {length}{delimiter}{string}{length}{delimiter}{string}...
/// Example: ["hello", "world"] -> "5%hello5%world"
///
/// Time Complexity: O(n) where n is total length of all strings
/// Space Complexity: O(n) for the encoded string

pub fn encode(strs: Vec<String>) -> String {
    let mut encoded_str: String;
    let delimiter: char;

    encoded_str = String::new();
    delimiter = '%';

    for str in strs {
        let str_length = str.len();
        encoded_str.push_str(&str_length.to_string());
        encoded_str.push(delimiter);
        encoded_str.push_str(&str);
    }

    encoded_str
}

pub fn decode(encoded_str: String) -> Vec<String> {
    let mut result: Vec<String>;
    let mut i: usize;
    let delimiter: char;

    result = Vec::new();
    i = 0;
    delimiter = '%';

    let chars: Vec<char> = encoded_str.chars().collect();

    while i < chars.len() {
        let mut j = i;

        // Find the delimiter to get the length
        while chars[j] != delimiter {
            j += 1;
        }

        let str_length: usize = encoded_str[i..j].parse().unwrap();
        let beginning_idx_of_str = j + 1;
        let ending_idx_of_str = beginning_idx_of_str + str_length;
        let str: String = chars[beginning_idx_of_str..ending_idx_of_str]
            .iter()
            .collect();
        result.push(str);

        i = ending_idx_of_str;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1_basic_strings() {
        let input = vec!["hello".to_string(), "world".to_string()];
        let encoded = encode(input.clone());
        let decoded = decode(encoded);

        assert_eq!(decoded, input);
    }

    #[test]
    fn test_example_2_multiple_strings() {
        let input = vec![
            "neet".to_string(),
            "code".to_string(),
            "love".to_string(),
            "you".to_string(),
        ];
        let encoded = encode(input.clone());
        let decoded = decode(encoded);

        assert_eq!(decoded, input);
    }

    #[test]
    fn test_example_3_strings_with_delimiter_character() {
        let input = vec![
            "we".to_string(),
            "say".to_string(),
            ":".to_string(),
            "yes".to_string(),
            "%".to_string(),
            "!".to_string(),
        ];
        let encoded = encode(input.clone());
        let decoded = decode(encoded);

        assert_eq!(decoded, input);
    }

    #[test]
    fn test_edge_case_empty_array() {
        let input: Vec<String> = vec![];
        let encoded = encode(input.clone());
        let decoded = decode(encoded.clone());

        assert_eq!(decoded, input);
        assert_eq!(encoded, "");
    }

    #[test]
    fn test_edge_case_array_with_empty_strings() {
        let input = vec!["".to_string(), "".to_string(), "".to_string()];
        let encoded = encode(input.clone());
        let decoded = decode(encoded);

        assert_eq!(decoded, input);
    }

    #[test]
    fn test_edge_case_single_empty_string() {
        let input = vec!["".to_string()];
        let encoded = encode(input.clone());
        let decoded = decode(encoded);

        assert_eq!(decoded, input);
    }

    #[test]
    fn test_complex_case_mixed_empty_and_non_empty_strings() {
        let input = vec![
            "hello".to_string(),
            "".to_string(),
            "world".to_string(),
            "".to_string(),
            "test".to_string(),
        ];
        let encoded = encode(input.clone());
        let decoded = decode(encoded);

        assert_eq!(decoded, input);
    }

    #[test]
    fn test_complex_case_strings_with_special_characters() {
        let input = vec![
            "#@!".to_string(),
            "123".to_string(),
            "a b c".to_string(),
            "new\nline".to_string(),
        ];
        let encoded = encode(input.clone());
        let decoded = decode(encoded);

        assert_eq!(decoded, input);
    }

    #[test]
    fn test_complex_case_long_strings() {
        let input = vec!["a".repeat(1000), "b".repeat(500)];
        let encoded = encode(input.clone());
        let decoded = decode(encoded);

        assert_eq!(decoded, input);
    }
}
