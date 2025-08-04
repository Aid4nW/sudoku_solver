
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut grid: Vec<String> = Vec::new();
    println!("Please enter the Sudoku puzzle (9 lines, 9 characters each):");
    for (i, line) in stdin.lock().lines().take(9).enumerate() {
        match line {
            Ok(l) => {
                if l.len() != 9 {
                    eprintln!("Error: Line {} must be exactly 9 characters.", i + 1);
                    return;
                }
                if !l.chars().all(|c| c.is_digit(10) || c == '.') {
                    eprintln!("Error: Line {} contains invalid characters. Only digits 0-9 or '.' are allowed.", i + 1);
                    return;
                }
                grid.push(l);
            },
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                return;
            }
        }
    }
    println!("\nYou entered:");
    for row in &grid {
        println!("{}", row);
    }
}
