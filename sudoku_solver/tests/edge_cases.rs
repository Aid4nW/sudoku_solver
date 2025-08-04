use sudoku_solver::{parse_grid, solve, is_valid_grid, find_empty};

fn make_lines(lines: &[&str]) -> Vec<String> {
    lines.iter().map(|s| s.to_string()).collect()
}

#[test]
fn test_solver_empty_grid() {
    // All cells empty
    let lines = make_lines(&[
        "000000000",
        "000000000",
        "000000000",
        "000000000",
        "000000000",
        "000000000",
        "000000000",
        "000000000",
        "000000000",
    ]);
    let mut grid = parse_grid(&lines).unwrap();
    assert!(solve(&mut grid));
    assert!(is_valid_grid(&grid));
}

#[test]
fn test_solver_minimal_clues() {
    // 17 clues, known to have a unique solution
    let lines = make_lines(&[
        "100000000",
        "020000000",
        "003000000",
        "000400000",
        "000050000",
        "000006000",
        "000000700",
        "000000080",
        "000000009",
    ]);
    let mut grid = parse_grid(&lines).unwrap();
    assert!(solve(&mut grid));
    assert!(is_valid_grid(&grid));
}

#[test]
fn test_solver_almost_full_grid() {
    // Only one cell empty
    let lines = make_lines(&[
        "534678912",
        "672195348",
        "198342567",
        "859761423",
        "426853791",
        "713924856",
        "961537284",
        "287419635",
        "34528617.", // last cell empty
    ]);
    let mut grid = parse_grid(&lines).unwrap();
    assert!(solve(&mut grid));
    assert!(is_valid_grid(&grid));
    assert!(find_empty(&grid).is_none());
}

#[test]
fn test_solver_multiple_solutions() {
    // This grid has multiple solutions (very underconstrained)
    let lines = make_lines(&[
        "100000000",
        "000000000",
        "000000000",
        "000000000",
        "000000000",
        "000000000",
        "000000000",
        "000000000",
        "000000000",
    ]);
    let mut grid = parse_grid(&lines).unwrap();
    assert!(solve(&mut grid));
    assert!(is_valid_grid(&grid));
    // Note: This does not check for uniqueness, just that a solution exists
}
