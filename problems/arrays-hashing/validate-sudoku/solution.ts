/**
 * Valid Sudoku
 *
 * Pattern: Arrays & Hashing
 * Difficulty: Medium
 *
 * Approach:
 * Use hash sets to track seen numbers in each row, column, and 3x3 sub-box.
 * For each cell, check if the number already exists in its row, column, or box.
 * If a duplicate is found, return false. Otherwise, add the number to the corresponding sets.
 *
 * The key insight is calculating which 3x3 box a cell belongs to using (row / 3, col / 3).
 *
 * Time Complexity: O(1) - Fixed 9x9 board = 81 cells
 * Space Complexity: O(1) - At most 9 rows + 9 columns + 9 boxes, each with at most 9 elements
 */

export function isValidSudoku(board: string[][]): boolean {
  const rows: Map<number, Set<string>> = new Map();
  const columns: Map<number, Set<string>> = new Map();
  const squares: Map<string, Set<string>> = new Map();

  // Iterate through each cell in the 9x9 board
  for (let row = 0; row < 9; row++) {
    for (let column = 0; column < 9; column++) {
      const num = board[row][column];

      // Skip empty cells
      if (num === ".") {
        continue;
      }

      // Calculate which 3x3 box this cell belongs to
      const boxKey = `${Math.floor(row / 3)},${Math.floor(column / 3)}`;

      // Initialize sets if they don't exist
      if (!rows.has(row)) {
        rows.set(row, new Set());
      }
      if (!columns.has(column)) {
        columns.set(column, new Set());
      }
      if (!squares.has(boxKey)) {
        squares.set(boxKey, new Set());
      }

      // Check if number already exists in row, column, or box
      if (
        rows.get(row)!.has(num) ||
        columns.get(column)!.has(num) ||
        squares.get(boxKey)!.has(num)
      ) {
        return false;
      }

      // Add number to corresponding sets
      rows.get(row)!.add(num);
      columns.get(column)!.add(num);
      squares.get(boxKey)!.add(num);
    }
  }

  return true;
}
