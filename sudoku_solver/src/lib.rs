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
