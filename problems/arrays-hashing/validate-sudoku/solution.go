// Package validate_sudoku implements a solution for Valid Sudoku
//
// Pattern: Arrays & Hashing
// Difficulty: Medium
//
// Approach:
// Use hash sets (maps in Go) to track seen numbers in each row, column, and 3x3 sub-box.
// For each cell, check if the number already exists in its row, column, or box.
// If a duplicate is found, return false. Otherwise, add the number to the corresponding sets.
//
// The key insight is calculating which 3x3 box a cell belongs to using (row / 3, col / 3).
//
// Time Complexity: O(1) - Fixed 9x9 board = 81 cells
// Space Complexity: O(1) - At most 9 rows + 9 columns + 9 boxes, each with at most 9 elements
package validate_sudoku

import "fmt"

// IsValidSudoku determines if a 9x9 Sudoku board is valid
func IsValidSudoku(board [][]byte) bool {
	rows := make(map[int]map[byte]bool)
	columns := make(map[int]map[byte]bool)
	squares := make(map[string]map[byte]bool)

	// Iterate through each cell in the 9x9 board
	for row := 0; row < 9; row++ {
		for column := 0; column < 9; column++ {
			num := board[row][column]

			// Skip empty cells
			if num == '.' {
				continue
			}

			// Calculate which 3x3 box this cell belongs to
			boxKey := fmt.Sprintf("%d,%d", row/3, column/3)

			// Initialize maps if they don't exist
			if rows[row] == nil {
				rows[row] = make(map[byte]bool)
			}
			if columns[column] == nil {
				columns[column] = make(map[byte]bool)
			}
			if squares[boxKey] == nil {
				squares[boxKey] = make(map[byte]bool)
			}

			// Check if number already exists in row, column, or box
			if rows[row][num] || columns[column][num] || squares[boxKey][num] {
				return false
			}

			// Add number to corresponding sets
			rows[row][num] = true
			columns[column][num] = true
			squares[boxKey][num] = true
		}
	}

	return true
}
