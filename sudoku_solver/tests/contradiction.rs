
use sudoku_solver::{is_contradictory, parse_grid};

fn make_lines(lines: &[&str]) -> Vec<String> {
    lines.iter().map(|s| s.to_string()).collect()
}

#[test]
fn test_contradiction_no_candidate() {
    // This grid has a cell (0, 0) where no number can be placed.
    // Row 0 contains 2-9, and the box contains 1.
    let lines = make_lines(&[
        ".23456789",
        "1........",
        ".........",
        ".........",
        ".........",
        ".........",
        ".........",
        ".........",
        ".........",
    ]);
    let grid = parse_grid(&lines).unwrap();
    assert!(is_contradictory(&grid));
}

#[test]
fn test_contradiction_number_cannot_be_placed_in_row() {
    // In this grid, the number 1 cannot be placed in the first row.
    let lines = make_lines(&[
        "023456789",
        "100000000",
        "000000000",
        "000000000",
        "000000000",
        "000000000",
        "000000000",
        "000000000",
        "000000000",
    ]);
    let grid = parse_grid(&lines).unwrap();
    assert!(is_contradictory(&grid));
}

#[test]
fn test_no_contradiction() {
    let lines = make_lines(&[
        "530070000",
        "600195000",
        "098000060",
        "800060003",
        "400803001",
        "700020006",
        "060000280",
        "000419005",
        "000080079",
    ]);
    let grid = parse_grid(&lines).unwrap();
    assert!(!is_contradictory(&grid));
}
