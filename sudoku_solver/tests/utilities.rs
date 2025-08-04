use sudoku_solver::{parse_grid, find_empty, can_place};

fn make_lines(lines: &[&str]) -> Vec<String> {
    lines.iter().map(|s| s.to_string()).collect()
}

#[test]
fn test_find_empty() {
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
    assert_eq!(find_empty(&grid), Some((0, 2)));
}

#[test]
fn test_find_empty_full() {
    let lines = make_lines(&[
        "534678912",
        "672195348",
        "198342567",
        "859761423",
        "426853791",
        "713924856",
        "961537284",
        "287419635",
        "345286179",
    ]);
    let grid = parse_grid(&lines).unwrap();
    assert_eq!(find_empty(&grid), None);
}

#[test]
fn test_can_place() {
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
    // Can place 4 at (0,2)
    assert!(can_place(&grid, 0, 2, 4));
    // Can't place 5 at (0,2) (already in row)
    assert!(!can_place(&grid, 0, 2, 5));
    // Can't place 6 at (0,2) (already in col)
    assert!(!can_place(&grid, 0, 2, 6));
    // Can't place 9 at (0,2) (already in box)
    assert!(!can_place(&grid, 0, 2, 9));
}
