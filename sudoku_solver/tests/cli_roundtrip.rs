use std::process::{Command, Stdio};
    #[test]
    fn test_cli_solves_puzzle() {
        let input = "530070000\n600195000\n098000060\n800060003\n400803001\n700020006\n060000280\n000419005\n000080079\n";
        let expected_solution = [
            "5 3 4 | 6 7 8 | 9 1 2",
            "6 7 2 | 1 9 5 | 3 4 8",
            "1 9 8 | 3 4 2 | 5 6 7",
            "------+-------+------",
            "8 5 9 | 7 6 1 | 4 2 3",
            "4 2 6 | 8 5 3 | 7 9 1",
            "7 1 3 | 9 2 4 | 8 5 6",
            "------+-------+------",
            "9 6 1 | 5 3 7 | 2 8 4",
            "2 8 7 | 4 1 9 | 6 3 5",
            "3 4 5 | 2 8 6 | 1 7 9",
        ];
        let output = Command::new(env!("CARGO_BIN_EXE_sudoku_solver"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                use std::io::Write;
                child.stdin.as_mut().unwrap().write_all(input.as_bytes())?;
                let output = child.wait_with_output()?;
                Ok(output)
            })
            .expect("Failed to run binary");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let solution_start = stdout.find("Solution:");
        assert!(solution_start.is_some(), "Output missing 'Solution:' header.\nFull output:\n{}", stdout);
        let solution_section = &stdout[solution_start.unwrap()..];
        for line in expected_solution.iter() {
            assert!(solution_section.contains(line), "Output missing solution line: {}\nSolution section:\n{}", line, solution_section);
        }
    }

    #[test]
    fn test_cli_unsolvable_puzzle() {
        let input = "530070000\n600195000\n098000060\n800060003\n400803001\n700020006\n060000280\n000419005\n000080071\n";
        let output = Command::new(env!("CARGO_BIN_EXE_sudoku_solver"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                use std::io::Write;
                child.stdin.as_mut().unwrap().write_all(input.as_bytes())?;
                let output = child.wait_with_output()?;
                Ok(output)
            })
            .expect("Failed to run binary");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let combined = format!("{}\n{}", stdout, stderr);
        assert!(combined.contains("Error: Initial grid violates Sudoku rules."), "Output missing initial grid validation error.\nFull output:\n{}", combined);
    }
#[cfg(test)]
mod tests {
    use std::process::{Command, Stdio};

    #[test]
    fn test_input_output_roundtrip() {
        let input = "530070000\n600195000\n098000060\n800060003\n400803001\n700020006\n060000280\n000419005\n000080079\n";
        let expected = [
            "5 3 . | . 7 . | . . .",
            "6 . . | 1 9 5 | . . .",
            ". 9 8 | . . . | . 6 .",
            "------+-------+------",
            "8 . . | . 6 . | . . 3",
            "4 . . | 8 . 3 | . . 1",
            "7 . . | . 2 . | . . 6",
            "------+-------+------",
            ". 6 . | . . . | 2 8 .",
            ". . . | 4 1 9 | . . 5",
            ". . . | . 8 . | . 7 9",
        ];
        let output = Command::new(env!("CARGO_BIN_EXE_sudoku_solver"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                use std::io::Write;
                child.stdin.as_mut().unwrap().write_all(input.as_bytes())?;
                let output = child.wait_with_output()?;
                Ok(output)
            })
            .expect("Failed to run binary");
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Find the pretty-printed grid section
        let grid_start = stdout.find("Parsed grid:");
        assert!(grid_start.is_some(), "Output missing 'Parsed grid:' header.\nFull output:\n{}", stdout);
        let grid_section = &stdout[grid_start.unwrap()..];
        for line in expected.iter() {
            assert!(grid_section.contains(line), "Output missing line: {}\nGrid section:\n{}", line, grid_section);
        }
    }
}
