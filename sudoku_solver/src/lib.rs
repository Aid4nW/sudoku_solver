/// Returns true if the grid is unsolvable by constraint propagation (no possible candidate for a cell, or a number cannot be placed in a row/col/box).
/// Prints debug information if a contradiction is found.
///
/// # Arguments
/// * `grid` - 9x9 Sudoku grid as Option<u8> (None for empty)
///
/// # Returns
/// * `true` if a contradiction is found, `false` otherwise
pub fn is_contradictory(grid: &[[Option<u8>; 9]; 9]) -> bool {
    // For each cell, check if there is at least one candidate
    for i in 0..9 {
        for j in 0..9 {
            if grid[i][j].is_none() {
                let mut has_candidate = false;
                for num in 1..=9 {
                    if can_place(grid, i, j, num) {
                        has_candidate = true;
                        break;
                    }
                }
                if !has_candidate {
                    return true;
                }
            }
        }
    }
    // For each number, check if it can be placed somewhere in each row, col, and box
    for num in 1..=9 {
        for i in 0..9 {
            // Row: check all empty cells in the row
            let mut can_place_somewhere = false;
            for j in 0..9 {
                if grid[i][j].is_none() && can_place(grid, i, j, num) {
                    can_place_somewhere = true;
                    break;
                }
            }
            if !can_place_somewhere && !grid[i].contains(&Some(num)) {
                return true;
            }
            // Col: check all empty cells in the column
            let mut can_place_somewhere = false;
            for j in 0..9 {
                if grid[j][i].is_none() && can_place(grid, j, i, num) {
                    can_place_somewhere = true;
                    break;
                }
            }
            if !can_place_somewhere && !(0..9).any(|j| grid[j][i] == Some(num)) {
                return true;
            }
        }
        // Box: check all empty cells in the box
        for box_row in 0..3 {
            for box_col in 0..3 {
                let mut found = false;
                for i in 0..3 {
                    for j in 0..3 {
                        let r = box_row * 3 + i;
                        let c = box_col * 3 + j;
                        if grid[r][c].is_none() && can_place(grid, r, c, num) {
                            found = true;
                        }
                        if grid[r][c] == Some(num) {
                            found = true;
                        }
                    }
                }
                if !found {
                    return true;
                }
            }
        }
    }
    false
}
/// Solve the Sudoku puzzle in-place using backtracking.
///
/// # Arguments
/// * `grid` - mutable reference to 9x9 Sudoku grid
///
/// # Returns
/// * `true` if solved, `false` if unsolvable
pub fn solve(grid: &mut [[Option<u8>; 9]; 9]) -> bool {
    if let Some((row, col)) = find_empty(grid) {
        for num in 1..=9 {
            if can_place(grid, row, col, num) {
                grid[row][col] = Some(num);
                if solve(grid) {
                    return true;
                }
                grid[row][col] = None;
            }
        }
        false
    } else {
        true // No empty cell left, puzzle solved
    }
}
/// Find the next empty cell in the grid.
///
/// # Arguments
/// * `grid` - 9x9 Sudoku grid
///
/// # Returns
/// * `Some((row, col))` if an empty cell is found, `None` if full
pub fn find_empty(grid: &[[Option<u8>; 9]; 9]) -> Option<(usize, usize)> {
    for i in 0..9 {
        for j in 0..9 {
            if grid[i][j].is_none() {
                return Some((i, j));
            }
        }
    }
    None
}

/// Check if a number can be placed at (row, col) according to Sudoku rules.
///
/// # Arguments
/// * `grid` - 9x9 Sudoku grid
/// * `row` - row index (0-8)
/// * `col` - column index (0-8)
/// * `num` - number to place (1-9)
///
/// # Returns
/// * `true` if the number can be placed, `false` otherwise
/// Pretty-print the Sudoku grid to stdout in a human-readable format.
///
/// # Arguments
/// * `grid` - 9x9 Sudoku grid
/// Parse a vector of 9 strings into a 9x9 Sudoku grid.
///
/// # Arguments
/// * `lines` - slice of 9 strings, each 9 characters long
///
/// # Returns
/// * `Ok(grid)` if parsing succeeds, `Err(msg)` if input is invalid
/// Validate that a Sudoku grid has no rule violations (rows, columns, boxes).
///
/// # Arguments
/// * `grid` - 9x9 Sudoku grid
///
/// # Returns
/// * `true` if valid, `false` if any rule is violated
pub fn can_place(grid: &[[Option<u8>; 9]; 9], row: usize, col: usize, num: u8) -> bool {
    // Check row and column
    for i in 0..9 {
        if grid[row][i] == Some(num) || grid[i][col] == Some(num) {
            return false;
        }
    }
    // Check 3x3 box
    let box_row = (row / 3) * 3;
    let box_col = (col / 3) * 3;
    for i in 0..3 {
        for j in 0..3 {
            if grid[box_row + i][box_col + j] == Some(num) {
                return false;
            }
        }
    }
    true
}
pub fn pretty_print_grid(grid: &[[Option<u8>; 9]; 9]) {
    for (i, row) in grid.iter().enumerate() {
        if i % 3 == 0 && i != 0 {
            println!("------+-------+------");
        }
        for (j, cell) in row.iter().enumerate() {
            if j % 3 == 0 && j != 0 {
                print!("| ");
            }
            match cell {
                Some(n) => print!("{} ", n),
                None => print!(". "),
            }
        }
        println!("");
    }
}
pub fn parse_grid(lines: &[String]) -> Result<[[Option<u8>; 9]; 9], String> {
    if lines.len() != 9 {
        return Err("Input must have exactly 9 lines.".to_string());
    }
    let mut grid = [[None; 9]; 9];
    for (i, line) in lines.iter().enumerate() {
        if line.len() != 9 {
            return Err(format!("Line {} must be exactly 9 characters.", i + 1));
        }
        for (j, c) in line.chars().enumerate() {
            grid[i][j] = match c {
                '1'..='9' => Some(c.to_digit(10).unwrap() as u8),
                '0' | '.' => None,
                _ => return Err(format!("Invalid character '{}' at line {}, col {}.", c, i + 1, j + 1)),
            };
        }
    }
    Ok(grid)
}

pub fn is_valid_grid(grid: &[[Option<u8>; 9]; 9]) -> bool {
    // Check rows, columns, and 3x3 boxes for duplicates
    for i in 0..9 {
        let mut row = [false; 10];
        let mut col = [false; 10];
        for j in 0..9 {
            if let Some(val) = grid[i][j] {
                if row[val as usize] { return false; }
                row[val as usize] = true;
            }
            if let Some(val) = grid[j][i] {
                if col[val as usize] { return false; }
                col[val as usize] = true;
            }
        }
    }
    // Check 3x3 boxes
    for box_row in 0..3 {
        for box_col in 0..3 {
            let mut seen = [false; 10];
            for i in 0..3 {
                for j in 0..3 {
                    if let Some(val) = grid[box_row*3 + i][box_col*3 + j] {
                        if seen[val as usize] { return false; }
                        seen[val as usize] = true;
                    }
                }
            }
        }
    }
    true
}
