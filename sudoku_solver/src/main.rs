

use std::io::{self, BufRead};
use sudoku_solver::{parse_grid, is_valid_grid, pretty_print_grid, solve, is_contradictory};



fn main() {
    let stdin = io::stdin();
    let mut lines: Vec<String> = Vec::new();
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
                lines.push(l);
            },
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                return;
            }
        }
    }
    let grid = match parse_grid(&lines) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            return;
        }
    };
    if !is_valid_grid(&grid) {
        eprintln!("Error: Initial grid violates Sudoku rules.");
        return;
    }
    let mut grid = match parse_grid(&lines) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            return;
        }
    };
    if !is_valid_grid(&grid) {
        eprintln!("Error: Initial grid violates Sudoku rules.");
        return;
    }
    println!("\nParsed grid:");
    pretty_print_grid(&grid);
    if is_contradictory(&grid) {
        println!("\nNo solution exists for the given puzzle.");
        return;
    }
    println!("\nSolving...");
    if solve(&mut grid) {
        println!("\nSolution:");
        pretty_print_grid(&grid);
    } else {
        println!("\nNo solution exists for the given puzzle.");
    }
}
