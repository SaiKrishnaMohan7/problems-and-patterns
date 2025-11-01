/// Two Sum
///
/// Pattern: arrays-hashing
/// Difficulty: Easy
///
/// Approach:
///1. Create a map that will hold the array elements as key and array index as value
// 2. Loop through the array of elements
// 	1. calculate the diffference `target - nums[i]`
// 	2. if the difference is in the map
// 		1. Push the index of the difference and the current index into the array
// 		2. break out of the loop and return
// 	2. else set `nums[i]` as key and the curr index as the value in the map
//
///
/// Time Complexity: O(?)
/// Space Complexity: O(?)

pub fn solution_function(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_map = std::collections::HashMap::new();
    let mut result = Vec::new();

    for (i, &num) in nums.iter().enumerate() {
        let difference = target - num;
        if let Some(&idx) = num_map.get(&difference) {
            result.push(idx as i32);
            result.push(i as i32);
            break;
        }
        num_map.insert(num, i);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let result = solution_function(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_example_2() {
        let result = solution_function(vec![3, 2, 4], 6);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_edge_case_empty_input() {
        let result = solution_function(vec![], 0);
        assert_eq!(result, vec![]);
    }
}
