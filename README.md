# Sudoku Solver in Rust

A command-line Sudoku solver written in Rust.
Efficiently solves standard 9x9 Sudoku puzzles with robust validation, clear output, and comprehensive tests.
Built using Github Copilot.

## Features
- Accepts Sudoku puzzles via standard input (stdin)
- Input format: 9 lines of 9 characters (digits 1-9, 0 or . for empty cells)
- Pretty-prints the parsed and solved grid
- Detects and reports unsolvable or invalid puzzles
- Fast backtracking solver with constraint propagation
- Comprehensive unit and integration tests

## Usage

### Build and Run

```sh
cargo run --release --bin sudoku_solver
```

### Input Format
You will be prompted to enter 9 lines, each with 9 characters:
- Digits 1-9 for filled cells
- 0 or . for empty cells

Example input:
```
530070000
600195000
098000060
800060003
400803001
700020006
060000280
000419005
000080079
```

### Output
- The program prints the parsed grid, attempts to solve it, and prints the solution or an error message if unsolvable or invalid.

## Testing

Run all tests (unit, integration, edge cases):

```sh
cargo test
```

## Documentation
- All public functions are documented in `src/lib.rs`.
- See `PRD.md` for requirements and implementation plan.
