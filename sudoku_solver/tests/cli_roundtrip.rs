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
