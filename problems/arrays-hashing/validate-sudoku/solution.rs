/// Valid Sudoku
///
/// Pattern: Arrays & Hashing
/// Difficulty: Medium
///
/// Approach:
/// Use hash sets to track seen numbers in each row, column, and 3x3 sub-box.
/// For each cell, check if the number already exists in its row, column, or box.
/// If a duplicate is found, return false. Otherwise, add the number to the corresponding sets.
///
/// The key insight is calculating which 3x3 box a cell belongs to using (row / 3, col / 3).
///
/// Time Complexity: O(1) - Fixed 9x9 board = 81 cells
/// Space Complexity: O(1) - At most 9 rows + 9 columns + 9 boxes, each with at most 9 elements

use std::collections::{HashMap, HashSet};

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rows: HashMap<usize, HashSet<char>> = HashMap::new();
    let mut columns: HashMap<usize, HashSet<char>> = HashMap::new();
    let mut squares: HashMap<(usize, usize), HashSet<char>> = HashMap::new();

    // Iterate through each cell in the 9x9 board
    for row in 0..9 {
        for column in 0..9 {
            let num = board[row][column];

            // Skip empty cells
            if num == '.' {
                continue;
            }

            // Calculate which 3x3 box this cell belongs to
            let box_key = (row / 3, column / 3);

            // Initialize sets if they don't exist
            rows.entry(row).or_insert_with(HashSet::new);
            columns.entry(column).or_insert_with(HashSet::new);
            squares.entry(box_key).or_insert_with(HashSet::new);

            // Check if number already exists in row, column, or box
            if rows[&row].contains(&num)
                || columns[&column].contains(&num)
                || squares[&box_key].contains(&num)
            {
                return false;
            }

            // Add number to corresponding sets
            rows.get_mut(&row).unwrap().insert(num);
            columns.get_mut(&column).unwrap().insert(num);
            squares.get_mut(&box_key).unwrap().insert(num);
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1_valid_sudoku_board() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(is_valid_sudoku(board), true);
    }

    #[test]
    fn test_example_2_invalid_sudoku_duplicate_in_row() {
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(is_valid_sudoku(board), false);
    }

    #[test]
    fn test_invalid_sudoku_duplicate_in_column() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['5', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(is_valid_sudoku(board), false);
    }

    #[test]
    fn test_invalid_sudoku_duplicate_in_3x3_box() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '5', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(is_valid_sudoku(board), false);
    }

    #[test]
    fn test_edge_case_all_empty_cells() {
        let board = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        assert_eq!(is_valid_sudoku(board), true);
    }
}
