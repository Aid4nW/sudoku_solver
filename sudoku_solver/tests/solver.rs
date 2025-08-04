use sudoku_solver::{parse_grid, solve, is_valid_grid, find_empty};

fn make_lines(lines: &[&str]) -> Vec<String> {
    lines.iter().map(|s| s.to_string()).collect()
}

#[test]
fn test_solver_solvable() {
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
    let mut grid = parse_grid(&lines).unwrap();
    assert!(solve(&mut grid));
    // Check that the grid is fully filled and valid
    assert!(find_empty(&grid).is_none(), "Grid should be fully filled");
    assert!(is_valid_grid(&grid), "Solved grid should be valid");
}

#[test]
fn test_solver_unsolvable() {
    // This grid is valid but unsolvable (contradiction in last row)
    let lines = make_lines(&[
        "530070000",
        "600195000",
        "098000060",
        "800060003",
        "400803001",
        "700020006",
        "060000280",
        "000419005",
        "000080071", // changed last two digits to create a contradiction
    ]);
    let mut grid = parse_grid(&lines).unwrap();
    assert!(!solve(&mut grid));
}
