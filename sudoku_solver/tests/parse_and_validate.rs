use sudoku_solver::{parse_grid, is_valid_grid};

fn make_lines(lines: &[&str]) -> Vec<String> {
    lines.iter().map(|s| s.to_string()).collect()
}

#[test]
fn test_valid_grid() {
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
    let grid = parse_grid(&lines).expect("Should parse");
    assert!(is_valid_grid(&grid));
}

#[test]
fn test_invalid_length() {
    let lines = make_lines(&[
        "53007000", // 8 chars
        "600195000",
        "098000060",
        "800060003",
        "400803001",
        "700020006",
        "060000280",
        "000419005",
        "000080079",
    ]);
    assert!(parse_grid(&lines).is_err());
}

#[test]
fn test_invalid_characters() {
    let lines = make_lines(&[
        "53007A000",
        "600195000",
        "098000060",
        "800060003",
        "400803001",
        "700020006",
        "060000280",
        "000419005",
        "000080079",
    ]);
    assert!(parse_grid(&lines).is_err());
}

#[test]
fn test_duplicate_in_row() {
    let lines = make_lines(&[
        "553070000", // two 5s in row
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
    assert!(!is_valid_grid(&grid));
}

#[test]
fn test_duplicate_in_col() {
    let lines = make_lines(&[
        "530070000",
        "530195000", // two 5s in col 1
        "098000060",
        "800060003",
        "400803001",
        "700020006",
        "060000280",
        "000419005",
        "000080079",
    ]);
    let grid = parse_grid(&lines).unwrap();
    assert!(!is_valid_grid(&grid));
}

#[test]
fn test_duplicate_in_box() {
    let lines = make_lines(&[
        "530070000",
        "600195000",
        "098500060", // two 5s in top-left box
        "800060003",
        "400803001",
        "700020006",
        "060000280",
        "000419005",
        "000080079",
    ]);
    let grid = parse_grid(&lines).unwrap();
    assert!(!is_valid_grid(&grid));
}
